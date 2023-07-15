use crate::constants;
use crate::context::context;
use crate::context::context::HMContext;
use crate::context::context::Type::{AiTrainerRewards, NoTokenSplitting, RegularDriver, TokenSplittingFleet};
use crate::pb::hivemapper::types::v1::{
    AiTrainerPayment, Mint, NoSplitPayment, Output, RegularDriverPayment, TokenSplittingPayment, Transfer,
    TransferChecked,
};
use std::ops::Div;
use substreams_solana::instruction::TokenInstruction;
use substreams_solana::pb::sol::v1::{CompiledInstruction, InnerInstructions, TokenBalance, TransactionStatusMeta};

pub fn process_compiled_instruction(
    output: &mut Output,
    timestamp: i64,
    trx_hash: &String,
    meta: &TransactionStatusMeta,
    inst_index: u32,
    inst: &CompiledInstruction,
    accounts: &Vec<Vec<u8>>,
) {
    let instruction_program_account = bs58::encode(&accounts[inst.program_id_index as usize]).into_string();

    if instruction_program_account == constants::HONEY_TOKEN_SPLITTING_INSTRUCTION_PROGRAM {
        let token_account = bs58::encode(&accounts[inst.accounts[1] as usize]).into_string();
        if token_account != constants::HONEY_CONTRACT_ADDRESS {
            return;
        }

        match inst.data[0] {
            constants::HONEY_TOKEN_SPLITTING_INSTRUCTION_BYTE => {
                let fleet_account = bs58::encode(&accounts[inst.accounts[4] as usize]).into_string();
                let fleet_driver_account = bs58::encode(&accounts[inst.accounts[3] as usize]).into_string();
                process_inner_instruction(
                    output,
                    timestamp,
                    trx_hash,
                    HMContext {
                        instruction_index: inst_index,
                        r#type: Some(TokenSplittingFleet(context::TokenSplittingFleet {
                            fleet_account,
                            fleet_driver_account,
                        })),
                    },
                    accounts,
                    &meta.inner_instructions,
                );
            }
            constants::HONEY_REGULAR_DRIVER_INSTRUCTION_BYTE => {
                let driver_account = bs58::encode(&accounts[inst.accounts[2] as usize]).into_string();
                process_inner_instruction(
                    output,
                    timestamp,
                    trx_hash,
                    HMContext {
                        instruction_index: inst_index,
                        r#type: Some(RegularDriver(context::RegularDriver { driver_account })),
                    },
                    accounts,
                    &meta.inner_instructions,
                );
            }
            constants::HONEY_NO_TOKEN_SPLITTING_INSTRUCTION_BYTE => {
                let driver_account = bs58::encode(&accounts[inst.accounts[2] as usize]).into_string();
                process_inner_instruction(
                    output,
                    timestamp,
                    trx_hash,
                    HMContext {
                        instruction_index: inst_index,
                        r#type: Some(NoTokenSplitting(context::NoTokenSplitting { driver_account })),
                    },
                    accounts,
                    &meta.inner_instructions,
                );
            }
            _ => {
                return;
            }
        }
    }

    if instruction_program_account == constants::HONEY_TOKEN_SPLITTING_CONTRACT {
        let token_account = bs58::encode(&accounts[inst.accounts[1] as usize]).into_string();
        if token_account != constants::HONEY_CONTRACT_ADDRESS {
            return;
        }
        match inst.data[0] {
            constants::HONEY_AI_TRAINER_INSTRUCTION_BYTE => {
                let account = bs58::encode(&accounts[inst.accounts[2] as usize]).into_string();
                process_inner_instruction(
                    output,
                    timestamp,
                    trx_hash,
                    HMContext {
                        instruction_index: inst_index,
                        r#type: Some(AiTrainerRewards(context::AiTrainerRewards { account })),
                    },
                    accounts,
                    &meta.inner_instructions,
                );
            }
            _ => {}
        }
    }

    // top level transaction without any inner instructions
    if instruction_program_account == constants::TOKEN_PROGRAM {
        let instruction = TokenInstruction::unpack(&inst.data).unwrap();
        let source = bs58::encode(&accounts[inst.accounts[0] as usize]).into_string();
        match instruction {
            TokenInstruction::Transfer { amount: amt } => {
                let authority = bs58::encode(&accounts[inst.accounts[2] as usize]).into_string();
                if valid_honey_token_transfer(&meta.pre_token_balances, &authority) {
                    let destination = bs58::encode(&accounts[inst.accounts[1] as usize]).into_string();
                    output.transfers.push(Transfer {
                        trx_hash: trx_hash.to_owned(),
                        timestamp,
                        from: source.to_owned(),
                        to: destination.to_owned(),
                        amount: amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64),
                    });
                }
            }
            TokenInstruction::TransferChecked {
                amount: amt,
                decimals: _,
            } => {
                let authority = bs58::encode(&accounts[inst.accounts[3] as usize]).into_string();
                if valid_honey_token_transfer(&meta.pre_token_balances, &authority) {
                    let destination = bs58::encode(&accounts[inst.accounts[2] as usize]).into_string();
                    output.transfer_checks.push(TransferChecked {
                        trx_hash: trx_hash.to_owned(),
                        timestamp,
                        from: source.to_owned(),
                        to: destination.to_owned(),
                        amount: amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64),
                        decimals: constants::HONEY_TOKEN_DECIMALS as i32,
                    })
                }
            }
            _ => {}
        }
    }

    // transfers from inner instructions
    // process_inner_instruction()
}

pub fn process_inner_instruction(
    output: &mut Output,
    timestamp: i64,
    trx_hash: &String,
    context: HMContext,
    accounts: &Vec<Vec<u8>>,
    inner_instructions: &Vec<InnerInstructions>,
) {
    match context.r#type.as_ref().unwrap() {
        TokenSplittingFleet(obj) => {
            let mut manager_mint = None;
            let mut driver_mint = None;
            inner_instructions
                .iter()
                .filter(|&inner_instruction| inner_instruction.index == context.instruction_index)
                .for_each(|inner_instruction| {
                    inner_instruction
                        .instructions
                        .iter()
                        .filter(|&inst| validate_token_program_instruction(accounts, inst.program_id_index as usize))
                        .for_each(|inst| {
                            let account_to = fetch_account_to(&accounts, inst.accounts[1]);
                            let instruction = TokenInstruction::unpack(&inst.data).unwrap();
                            match instruction {
                                TokenInstruction::MintTo { amount } => {
                                    if account_to == obj.fleet_account {
                                        manager_mint = Some(Mint {
                                            timestamp,
                                            trx_hash: trx_hash.to_owned(),
                                            to: account_to.clone(),
                                            amount: amount_to_decimals(
                                                amount as f64,
                                                constants::HONEY_TOKEN_DECIMALS as f64,
                                            ),
                                        });
                                    }

                                    if account_to == obj.fleet_driver_account {
                                        driver_mint = Some(Mint {
                                            timestamp,
                                            trx_hash: trx_hash.clone(),
                                            to: account_to,
                                            amount: amount_to_decimals(
                                                amount as f64,
                                                constants::HONEY_TOKEN_DECIMALS as f64,
                                            ),
                                        });
                                    }
                                }
                                _ => {}
                            }
                        });
                });

            if manager_mint.is_some() && driver_mint.is_some() {
                output.token_splitting_payments.push(TokenSplittingPayment {
                    manager_mint,
                    driver_mint,
                })
            }
        }
        RegularDriver(obj) => {
            let mut driver_mint = None;
            inner_instructions
                .iter()
                .filter(|&inner_instruction| inner_instruction.index == context.instruction_index)
                .for_each(|inner_instruction| {
                    inner_instruction
                        .instructions
                        .iter()
                        .filter(|&inst| validate_token_program_instruction(accounts, inst.program_id_index as usize))
                        .for_each(|inst| {
                            let account_to = fetch_account_to(&accounts, inst.accounts[1]);
                            let instruction = TokenInstruction::unpack(&inst.data).unwrap();
                            match instruction {
                                TokenInstruction::MintTo { amount } => {
                                    if account_to == obj.driver_account {
                                        driver_mint = Some(Mint {
                                            trx_hash: trx_hash.to_owned(),
                                            timestamp,
                                            to: account_to,
                                            amount: amount_to_decimals(
                                                amount as f64,
                                                constants::HONEY_TOKEN_DECIMALS as f64,
                                            ),
                                        });
                                    }
                                }
                                _ => {}
                            }
                        });
                });

            if driver_mint.is_some() {
                output
                    .regular_driver_payments
                    .push(RegularDriverPayment { mint: driver_mint })
            }
        }
        NoTokenSplitting(obj) => {
            let mut no_token_splitting_driver_mint = None;
            inner_instructions
                .iter()
                .filter(|&inner_instruction| inner_instruction.index == context.instruction_index)
                .for_each(|inner_instruction| {
                    inner_instruction
                        .instructions
                        .iter()
                        .filter(|&inst| validate_token_program_instruction(accounts, inst.program_id_index as usize))
                        .for_each(|inst| {
                            let account_to = fetch_account_to(&accounts, inst.accounts[1]);
                            let instruction = TokenInstruction::unpack(&inst.data).unwrap();
                            match instruction {
                                TokenInstruction::MintTo { amount } => {
                                    if account_to == obj.driver_account {
                                        no_token_splitting_driver_mint = Some(Mint {
                                            trx_hash: trx_hash.to_owned(),
                                            timestamp,
                                            to: account_to,
                                            amount: amount_to_decimals(
                                                amount as f64,
                                                constants::HONEY_TOKEN_DECIMALS as f64,
                                            ),
                                        });
                                    }
                                }
                                _ => {}
                            }
                        });
                });

            if no_token_splitting_driver_mint.is_some() {
                output.no_split_payments.push(NoSplitPayment {
                    mint: no_token_splitting_driver_mint,
                })
            }
        }
        AiTrainerRewards(obj) => {
            let mut payment_mint = None;
            inner_instructions
                .iter()
                .filter(|&inner_instruction| inner_instruction.index == context.instruction_index)
                .for_each(|inner_instruction| {
                    inner_instruction
                        .instructions
                        .iter()
                        .filter(|&inst| validate_token_program_instruction(accounts, inst.program_id_index as usize))
                        .for_each(|inst| {
                            let account_to = fetch_account_to(&accounts, inst.accounts[1]);
                            let instruction = TokenInstruction::unpack(&inst.data).unwrap();
                            match instruction {
                                TokenInstruction::MintTo { amount } => {
                                    if account_to == obj.account {
                                        payment_mint = Some(Mint {
                                            trx_hash: trx_hash.to_owned(),
                                            timestamp,
                                            to: account_to,
                                            amount: amount_to_decimals(
                                                amount as f64,
                                                constants::HONEY_TOKEN_DECIMALS as f64,
                                            ),
                                        });
                                    }
                                }
                                _ => {}
                            }
                        })
                });
            if payment_mint.is_some() {
                output.ai_trainer_payments.push(AiTrainerPayment { mint: payment_mint })
            }
        }
    }
}

fn amount_to_decimals(amount: f64, decimal: f64) -> f64 {
    let base: f64 = 10.0;
    return amount.div(&(base.powf(decimal)));
}

fn fetch_account_to(account_keys: &Vec<Vec<u8>>, position: u8) -> String {
    // Instruction account will contain the list of accounts to fetch in the accounts list
    // inst account pos 0 -> mint_info
    // inst account pos 1 -> destination_account_info
    // inst account pos 2 -> owner_info
    return bs58::encode(&account_keys[position as usize]).into_string();
}

fn validate_token_program_instruction(accounts: &Vec<Vec<u8>>, program_id_index: usize) -> bool {
    let program_id = &accounts[program_id_index];
    let account_id = bs58::encode(program_id).into_string();
    return account_id == constants::TOKEN_PROGRAM;
}

fn valid_honey_token_transfer(pre_token_balances: &Vec<TokenBalance>, account: &String) -> bool {
    substreams::log::info!("pre_token_balances: {:?}", pre_token_balances);
    substreams::log::info!("account: {}", account);
    for token_balance in pre_token_balances.iter() {
        if token_balance.owner.eq(account) && token_balance.mint.eq(constants::HONEY_CONTRACT_ADDRESS) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use crate::utils::amount_to_decimals;

    #[test]
    pub fn test_amount_to_decimals() {
        let amount = 4983184141.0;
        let expected = 4.983184141;

        let actual = amount_to_decimals(amount, 9 as f64);
        println!("expected {:?} actual {:?}", expected, actual);
        assert_eq!(expected, actual)
    }
}
