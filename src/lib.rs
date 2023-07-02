mod pb;
mod utils;

use crate::pb::hivemapper::types::v1::driver_payment::DriverType::{NoSplit, Regular};
use crate::pb::hivemapper::types::v1::{
    AiTrainerPayment, AiTrainerPayments, DriverPayment, DriverPayments, Payment, TokenSplittingPayment,
    TokenSplittingPayments, Transfer, Transfers,
};
use substreams::errors::Error;
use substreams::log;
use substreams_solana::instruction::TokenInstruction;
use substreams_solana::pb::sol::v1::Block;

const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const HONEY_CONTRACT_ADDRESS: &str = "4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy";
const HONEY_TOKEN_SPLITTING_CONTRACT: &str = "EEjwuvCMVYjgHUeX1BM9qmUog59Pft88c3jbt2ATwcJw";
const HONEY_TOKEN_SPLITTING_INSTRUCTION_PROGRAM: &str = "BNH1dUp3ExFbgo3YctSqQbJXRFn3ffkwbcmSas8azfaW";
const HONEY_TOKEN_SPLITTING_INSTRUCTION_BYTE: u8 = 172; // ac
const HONEY_NO_TOKEN_SPLITTING_INSTRUCTION_BYTE: u8 = 179; // b3
const HONEY_REGULAR_DRIVER_INSTRUCTION_BYTE: u8 = 151; // 97
const HONEY_AI_TRAINER_INSTRUCTION_BYTE: u8 = 241; // f1
const HONEY_TOKEN_DECIMALS: u8 = 9;

#[substreams::handlers::map]
pub fn map_token_splitting_payments(block: Block) -> Result<TokenSplittingPayments, Error> {
    let mut payments = vec![];
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = &confirmed_trx.transaction {
            let trx_id = bs58::encode(&trx.signatures[0]).into_string();
            if let Some(msg) = &trx.message {
                let mut honey_token_splitting_contract = false;
                let mut fleet_manager_account = String::new();
                let mut fleet_driver_account = String::new();

                for inst in msg.instructions.iter() {
                    let instruction_program_account =
                        bs58::encode(&msg.account_keys[inst.program_id_index as usize]).into_string();

                    if instruction_program_account != HONEY_TOKEN_SPLITTING_INSTRUCTION_PROGRAM {
                        continue;
                    }

                    let token_account = bs58::encode(&msg.account_keys[inst.accounts[1] as usize]).into_string();
                    if token_account != HONEY_CONTRACT_ADDRESS {
                        continue;
                    }

                    if inst.data[0] == HONEY_TOKEN_SPLITTING_INSTRUCTION_BYTE {
                        // ac: instruction for token splitting
                        honey_token_splitting_contract = true;
                        fleet_manager_account =
                            bs58::encode(&msg.account_keys[inst.accounts[4] as usize]).into_string();
                        fleet_driver_account = bs58::encode(&msg.account_keys[inst.accounts[3] as usize]).into_string();

                        break;
                    }
                }

                if !honey_token_splitting_contract {
                    continue;
                }

                if let Some(_) = msg.header {
                    let meta = confirmed_trx.meta.as_ref().unwrap();
                    let mut manager_payment = Payment::default();
                    let mut driver_payment = Payment::default();

                    for inner_inst in meta.inner_instructions.iter() {
                        for inst in inner_inst.instructions.iter() {
                            let program_id = &msg.account_keys[inst.program_id_index as usize];
                            let account_id = bs58::encode(program_id).into_string();

                            if account_id != TOKEN_PROGRAM {
                                continue;
                            }

                            let account_to = utils::fetch_account_to(&msg.account_keys, inst.accounts[1]);

                            let instruction = TokenInstruction::unpack(&inst.data)?;
                            match instruction {
                                TokenInstruction::MintTo { amount } => {
                                    if account_to == fleet_manager_account {
                                        manager_payment.timestamp = timestamp;
                                        manager_payment.transaction_id = trx_id.clone();
                                        manager_payment.address = account_to;
                                        manager_payment.amount =
                                            utils::amount_to_decimals(amount as f64, HONEY_TOKEN_DECIMALS as f64);
                                        continue;
                                    }

                                    if account_to == fleet_driver_account {
                                        driver_payment.timestamp = timestamp;
                                        driver_payment.transaction_id = trx_id.clone();
                                        driver_payment.address = account_to;
                                        driver_payment.amount =
                                            utils::amount_to_decimals(amount as f64, HONEY_TOKEN_DECIMALS as f64);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    payments.push(TokenSplittingPayment {
                        manager: Some(manager_payment),
                        driver: Some(driver_payment),
                    });
                }
            }
        }
    }

    Ok(TokenSplittingPayments { payments })
}

#[substreams::handlers::map]
pub fn map_driver_payments(block: Block) -> Result<DriverPayments, Error> {
    let mut payments = vec![];
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = &confirmed_trx.transaction {
            let trx_id = bs58::encode(&trx.signatures[0]).into_string();
            if let Some(msg) = &trx.message {
                let mut honey_driver_payment = false;
                let mut honey_driver_no_split_payment = false;
                let mut driver_account = String::new();

                for inst in msg.instructions.iter() {
                    let instruction_program_account =
                        bs58::encode(&msg.account_keys[inst.program_id_index as usize]).into_string();

                    if instruction_program_account != HONEY_TOKEN_SPLITTING_INSTRUCTION_PROGRAM {
                        continue;
                    }

                    let token_account = bs58::encode(&msg.account_keys[inst.accounts[1] as usize]).into_string();
                    if token_account != HONEY_CONTRACT_ADDRESS {
                        continue;
                    }

                    if inst.data[0] == HONEY_REGULAR_DRIVER_INSTRUCTION_BYTE {
                        honey_driver_payment = true;
                        driver_account = bs58::encode(&msg.account_keys[inst.accounts[2] as usize]).into_string();
                        break;
                    }

                    if inst.data[0] == HONEY_NO_TOKEN_SPLITTING_INSTRUCTION_BYTE {
                        honey_driver_no_split_payment = true;
                        driver_account = bs58::encode(&msg.account_keys[inst.accounts[2] as usize]).into_string();
                        break;
                    }
                }

                if !honey_driver_payment && !honey_driver_no_split_payment {
                    continue;
                }

                if let Some(_) = msg.header {
                    let meta = confirmed_trx.meta.as_ref().unwrap();
                    let mut driver_payment = Payment::default();

                    for inner_inst in meta.inner_instructions.iter() {
                        for inst in inner_inst.instructions.iter() {
                            let program_id = &msg.account_keys[inst.program_id_index as usize];
                            let account_id = bs58::encode(program_id).into_string();

                            if account_id != TOKEN_PROGRAM {
                                continue;
                            }

                            let account_to = utils::fetch_account_to(&msg.account_keys, inst.accounts[1]);

                            let instruction = TokenInstruction::unpack(&inst.data)?;
                            match instruction {
                                TokenInstruction::MintTo { amount } => {
                                    if account_to == driver_account {
                                        driver_payment.timestamp = timestamp;
                                        driver_payment.transaction_id = trx_id.clone();
                                        driver_payment.address = account_to;
                                        driver_payment.amount =
                                            utils::amount_to_decimals(amount as f64, HONEY_TOKEN_DECIMALS as f64);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    let mut dr = DriverPayment {
                        payment: Some(driver_payment),
                        r#type: 0,
                    };

                    if honey_driver_payment {
                        dr.r#type = i32::from(Regular);
                    }

                    if honey_driver_no_split_payment {
                        dr.r#type = i32::from(NoSplit);
                    }

                    payments.push(dr);
                }
            }
        }
    }

    Ok(DriverPayments { rewards: payments })
}

#[substreams::handlers::map]
pub fn map_ai_trainer_payments(block: Block) -> Result<AiTrainerPayments, Error> {
    let mut payments = vec![];
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = &confirmed_trx.transaction {
            let trx_id = bs58::encode(&trx.signatures[0]).into_string();
            if let Some(msg) = &trx.message {
                let mut honey_ai_trainer_payment = false;
                let mut driver_account = String::new();

                for inst in msg.instructions.iter() {
                    let instruction_program_account =
                        bs58::encode(&msg.account_keys[inst.program_id_index as usize]).into_string();

                    if instruction_program_account != HONEY_TOKEN_SPLITTING_CONTRACT {
                        continue;
                    }

                    let token_account = bs58::encode(&msg.account_keys[inst.accounts[1] as usize]).into_string();
                    if token_account != HONEY_CONTRACT_ADDRESS {
                        continue;
                    }

                    if inst.data[0] == HONEY_AI_TRAINER_INSTRUCTION_BYTE {
                        honey_ai_trainer_payment = true;
                        driver_account = bs58::encode(&msg.account_keys[inst.accounts[2] as usize]).into_string();
                        break;
                    }
                }

                if !honey_ai_trainer_payment {
                    continue;
                }

                if let Some(_) = msg.header {
                    let meta = confirmed_trx.meta.as_ref().unwrap();
                    let mut driver_payment = Payment::default();

                    for inner_inst in meta.inner_instructions.iter() {
                        for inst in inner_inst.instructions.iter() {
                            let program_id = &msg.account_keys[inst.program_id_index as usize];
                            let account_id = bs58::encode(program_id).into_string();

                            if account_id != TOKEN_PROGRAM {
                                continue;
                            }

                            let account_to = utils::fetch_account_to(&msg.account_keys, inst.accounts[1]);

                            let instruction = TokenInstruction::unpack(&inst.data)?;
                            match instruction {
                                TokenInstruction::MintTo { amount } => {
                                    if account_to == driver_account {
                                        driver_payment.timestamp = timestamp;
                                        driver_payment.transaction_id = trx_id.clone();
                                        driver_payment.address = account_to;
                                        driver_payment.amount =
                                            utils::amount_to_decimals(amount as f64, HONEY_TOKEN_DECIMALS as f64);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    payments.push(AiTrainerPayment {
                        payment: Some(driver_payment),
                    });
                }
            }
        }
    }

    Ok(AiTrainerPayments { payments })
}

//TODO: create the address lookup table substreams and if the program index is > len of msg.accounts
// then we need to lookup from the address loopk up table substreams
#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<Transfers, Error> {
    let mut transfers = vec![];
    let timestamp = block.block_time.as_ref().unwrap().timestamp;
    let mut trx_id = String::new();
    let mut msg_accounts = vec![];
    let msg_accounts_set = false;

    for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = &confirmed_trx.transaction {
            trx_id = bs58::encode(&trx.signatures[0]).into_string();
            if let Some(msg) = &trx.message {
                if !msg_accounts_set {
                    msg_accounts = msg.clone().account_keys;
                }

                for inst in msg.instructions.iter() {
                    let instruction_program_account =
                        bs58::encode(&msg_accounts[inst.program_id_index as usize]).into_string();

                    if instruction_program_account != TOKEN_PROGRAM {
                        continue;
                    }

                    let source = bs58::encode(&msg_accounts[inst.accounts[0] as usize]).into_string();
                    let destination = bs58::encode(&msg_accounts[inst.accounts[1] as usize]).into_string();
                    let authority = bs58::encode(&msg_accounts[inst.accounts[2] as usize]).into_string();

                    let instruction = TokenInstruction::unpack(&inst.data)?;
                    let mut amount = 0.0;
                    match instruction {
                        TokenInstruction::Transfer { amount: amt } => {
                            amount = utils::amount_to_decimals(amt as f64, HONEY_TOKEN_DECIMALS as f64);
                        }
                        _ => {}
                    }

                    if let Some(meta) = &confirmed_trx.meta {
                        if utils::valid_honey_token_transfer(&meta.pre_token_balances, &authority) {
                            transfers.push(Transfer {
                                trx_id: trx_id.clone(),
                                timestamp,
                                from: source,
                                to: destination,
                                amount,
                            });
                        }
                    }
                }
            }
        }
        // if let Some(meta) = &confirmed_trx.meta {
        //     for inner_transaction in meta.inner_instructions.iter() {
        //         for inst in inner_transaction.instructions.iter() {
        //             let instruction_program_account =
        //                 bs58::encode(&msg_accounts[inst.program_id_index as usize]).into_string();
        //
        //             if instruction_program_account != TOKEN_PROGRAM {
        //                 continue;
        //             }
        //
        //             let source = bs58::encode(&msg_accounts[inst.accounts[0] as usize]).into_string();
        //             let destination = bs58::encode(&msg_accounts[inst.accounts[1] as usize]).into_string();
        //             let authority = bs58::encode(&msg_accounts[inst.accounts[2] as usize]).into_string();
        //
        //             let instruction = TokenInstruction::unpack(&inst.data)?;
        //             let mut amount = 0.0;
        //             match instruction {
        //                 TokenInstruction::Transfer { amount: amt } => {
        //                     amount = utils::amount_to_decimals(amt as f64, HONEY_TOKEN_DECIMALS as f64);
        //                 }
        //                 _ => continue,
        //             }
        //
        //             if let Some(meta) = &confirmed_trx.meta {
        //                 if utils::valid_honey_token_transfer(&meta.pre_token_balances, &authority) {
        //                     transfers.push(Transfer {
        //                         trx_id: trx_id.clone(),
        //                         timestamp,
        //                         from: source,
        //                         to: destination,
        //                         amount,
        //                     });
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    Ok(Transfers { transfers })
}
