use crate::constants;
use crate::context::context;
use crate::context::context::Type::{
    AiTrainerRewards, InitializeAccount, NoTokenSplitting, RegularDriver, TokenSplittingFleet, Transfer,
    TransferChecked,
};
use crate::context::context::{HMContext, Type};
use crate::pb::hivemapper::types::v1::{
    AiTrainerPayment, InitializedAccount, Mint, NoSplitPayment, Output, RegularDriverPayment, TokenSplittingPayment,
    Transfer as Tr, TransferChecked as TrChecked,
};
use prost_types::Any;
use std::ops::Div;
use substreams::prelude::StoreGet;
use substreams::store::StoreGetArray;
use substreams_solana::pb::sf::solana::r#type::v1::{
    CompiledInstruction, InnerInstruction, InnerInstructions, TokenBalance, TransactionStatusMeta,
};
use substreams_solana::token_instruction::TokenInstruction;

pub fn process_compiled_instruction(
    output: &mut Output,
    timestamp: i64,
    trx_hash: &String,
    meta: &TransactionStatusMeta,
    inst_index: u32,
    inst: &CompiledInstruction,
    accounts: &Vec<String>,
) {
    let instruction_program_account = &accounts[inst.program_id_index as usize];

    if instruction_program_account == constants::HONEY_TOKEN_SPLITTING_INSTRUCTION_PROGRAM {
        let token_account = &accounts[inst.accounts[1] as usize];
        if token_account != constants::HONEY_CONTRACT_ADDRESS {
            return;
        }

        match inst.data[0] {
            constants::HONEY_TOKEN_SPLITTING_INSTRUCTION_BYTE => {
                let fleet_account = &accounts[inst.accounts[4] as usize];
                let fleet_driver_account = &accounts[inst.accounts[3] as usize];
                process_inner_instructions(
                    output,
                    timestamp,
                    trx_hash,
                    HMContext {
                        instruction_index: inst_index,
                        r#type: Some(TokenSplittingFleet(context::TokenSplittingFleet {
                            fleet_account: fleet_account.to_owned(),
                            fleet_driver_account: fleet_driver_account.to_owned(),
                        })),
                    },
                    accounts,
                    &meta.inner_instructions,
                );
            }
            constants::HONEY_REGULAR_DRIVER_INSTRUCTION_BYTE => {
                let driver_account = bs58::encode(&accounts[inst.accounts[2] as usize]).into_string();
                process_inner_instructions(
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
                process_inner_instructions(
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
            _ => {}
        }
        return;
    }

    if instruction_program_account == constants::HONEY_TOKEN_SPLITTING_CONTRACT {
        let token_account = bs58::encode(&accounts[inst.accounts[1] as usize]).into_string();
        if token_account != constants::HONEY_CONTRACT_ADDRESS {
            return;
        }
        match inst.data[0] {
            constants::HONEY_AI_TRAINER_INSTRUCTION_BYTE => {
                let account = bs58::encode(&accounts[inst.accounts[2] as usize]).into_string();
                process_inner_instructions(
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
        return;
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
                    output.transfers.push(Tr {
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
                    output.transfer_checks.push(TrChecked {
                        trx_hash: trx_hash.to_owned(),
                        timestamp,
                        from: source.to_owned(),
                        to: destination.to_owned(),
                        amount: amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64),
                        decimals: constants::HONEY_TOKEN_DECIMALS as i32,
                    })
                }
            }
            // TODO: refactor this...
            TokenInstruction::MintTo { amount: amt } => {
                // todo
            }
            TokenInstruction::MintToChecked {
                amount: amt,
                decimals: _,
            } => {
                // todo
            }
            TokenInstruction::Burn { amount: amt } => {
                // todo
            }
            TokenInstruction::BurnChecked {
                amount: amt,
                decimals: _,
            } => {
                // todo
            }
            TokenInstruction::InitializeAccount {} => {}
            TokenInstruction::InitializeAccount2 { .. } => {}
            TokenInstruction::InitializeAccount3 { .. } => {}
            _ => {}
        }
        return;
    }

    if instruction_program_account == constants::ASSOCIATED_TOKEN_PROGRAM {
        // substreams::log::info!("associated token program");
        // substreams::log::info!("compiled instruction {:?}", inst);
        process_inner_instructions(
            output,
            timestamp,
            trx_hash,
            HMContext {
                instruction_index: inst_index,
                r#type: Some(InitializeAccount(context::InitializeAccount {})),
            },
            accounts,
            &meta.inner_instructions,
        );
        return;
    }

    // transfers from inner instructions
    process_inner_instructions(
        output,
        timestamp,
        trx_hash,
        HMContext {
            instruction_index: inst_index,
            r#type: Some(Transfer(context::Transfer {})),
        },
        accounts,
        &meta.inner_instructions,
    );

    // transfer_checkeds from inner instructions
    process_inner_instructions(
        output,
        timestamp,
        trx_hash,
        HMContext {
            instruction_index: inst_index,
            r#type: Some(TransferChecked(context::TransferChecked {})),
        },
        accounts,
        &meta.inner_instructions,
    );
}

pub fn process_inner_instructions(
    output: &mut Output,
    timestamp: i64,
    trx_hash: &String,
    context: HMContext,
    accounts: &Vec<String>,
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
                            if let Some(mint) = process_inner_instruction_mint(&trx_hash, timestamp, accounts, inst) {
                                if mint.to.eq(&obj.fleet_account) {
                                    manager_mint = Some(mint);
                                } else if mint.to.eq(&obj.fleet_driver_account) {
                                    driver_mint = Some(mint);
                                }
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
            process_inner_instructions_mint(
                context.instruction_index,
                &obj.driver_account,
                trx_hash,
                timestamp,
                inner_instructions,
                accounts,
            )
            .into_iter()
            .for_each(|mint| {
                output
                    .regular_driver_payments
                    .push(RegularDriverPayment { mint: Some(mint) });
            });
        }
        NoTokenSplitting(obj) => {
            process_inner_instructions_mint(
                context.instruction_index,
                &obj.driver_account,
                trx_hash,
                timestamp,
                inner_instructions,
                accounts,
            )
            .into_iter()
            .for_each(|mint| {
                output.ai_trainer_payments.push(AiTrainerPayment { mint: Some(mint) });
            });
        }
        AiTrainerRewards(obj) => {
            process_inner_instructions_mint(
                context.instruction_index,
                &obj.account,
                trx_hash,
                timestamp,
                inner_instructions,
                accounts,
            )
            .into_iter()
            .for_each(|mint| {
                output.no_split_payments.push(NoSplitPayment { mint: Some(mint) });
            });
        }
        Transfer(_) => {
            //todo
        }
        TransferChecked(_) => {
            //todo
        }
        InitializeAccount(_) => {
            inner_instructions
                .iter()
                .filter(|&inner_instruction| inner_instruction.index == context.instruction_index)
                .for_each(|inner_instruction| {
                    inner_instruction
                        .instructions
                        .iter()
                        .filter(|&inst| accounts[inst.program_id_index as usize].eq(constants::TOKEN_PROGRAM))
                        .for_each(|inst| {
                            // substreams::log::info!("transaction hash: {}", trx_hash);
                            // substreams::log::info!("inst {:?}", inner_instruction);
                            let instruction = TokenInstruction::unpack(&inst.data).unwrap();
                            match instruction {
                                TokenInstruction::InitializeAccount {} => {
                                    // todo
                                }
                                TokenInstruction::InitializeAccount2 { owner: ow } => {
                                    // todo
                                }
                                TokenInstruction::InitializeAccount3 { owner: ow } => {
                                    let mint = bs58::encode(&accounts[inst.accounts[1] as usize]).into_string();

                                    if mint != constants::HONEY_CONTRACT_ADDRESS {
                                        return;
                                    }

                                    let account = bs58::encode(&accounts[inst.accounts[0] as usize]).into_string();
                                    let instruction = TokenInstruction::unpack(&inst.data).unwrap();
                                    output.initialized_account.push(InitializedAccount {
                                        trx_hash: trx_hash.to_owned(),
                                        account,
                                        mint,
                                        owner: bs58::encode(ow).into_string(),
                                    })
                                }
                                _ => return,
                            }
                        })
                });
        }
    }
}

fn process_inner_instructions_mint(
    context_instruction_index: u32,
    context_account: &String,
    trx_hash: &String,
    timestamp: i64,
    inner_instructions: &Vec<InnerInstructions>,
    accounts: &Vec<String>,
) -> Vec<Mint> {
    let mut mints = vec![];
    inner_instructions
        .iter()
        .filter(|&inner_instruction| inner_instruction.index == context_instruction_index)
        .for_each(|inner_instruction| {
            inner_instruction
                .instructions
                .iter()
                .filter(|&inst| validate_token_program_instruction(accounts, inst.program_id_index as usize))
                .for_each(|inst| {
                    if let Some(mint) = process_inner_instruction_mint(&trx_hash, timestamp, accounts, inst) {
                        if mint.to.eq(context_account) {
                            mints.push(mint);
                        }
                    }
                });
        });
    return mints;
}

fn process_inner_instruction_mint(
    trx_hash: &String,
    timestamp: i64,
    accounts: &Vec<String>,
    inst: &InnerInstruction,
) -> Option<Mint> {
    let account_to = fetch_account_to(&accounts, inst.accounts[1]);
    let instruction = TokenInstruction::unpack(&inst.data).unwrap();
    match instruction {
        TokenInstruction::MintTo { amount: amt }
        | TokenInstruction::MintToChecked {
            amount: amt,
            decimals: _,
        } => {
            return Some(Mint {
                trx_hash: trx_hash.to_owned(),
                timestamp,
                to: account_to,
                amount: amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64),
            });
        }
        _ => {}
    }
    return None;
}

fn process_inner_instruction_transfer() {}

fn amount_to_decimals(amount: f64, decimal: f64) -> f64 {
    let base: f64 = 10.0;
    return amount.div(&(base.powf(decimal)));
}

fn fetch_account_to(account_keys: &Vec<String>, position: u8) -> String {
    // Instruction account will contain the list of accounts to fetch in the accounts list
    // inst account pos 0 -> mint_info
    // inst account pos 1 -> destination_account_info
    // inst account pos 2 -> owner_info
    return account_keys[position as usize].to_owned();
}

fn validate_token_program_instruction(accounts: &Vec<String>, program_id_index: usize) -> bool {
    return &accounts[program_id_index] == constants::TOKEN_PROGRAM;
}

fn valid_honey_token_transfer(pre_token_balances: &Vec<TokenBalance>, account: &String) -> bool {
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
