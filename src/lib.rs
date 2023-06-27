mod pb;
mod utils;

use crate::pb::hivemapper::types::v1::{Payment, TokenSplittingContractPayment, TokenSplittingContractPayments};
use substreams::errors::Error;
use substreams::log;
use substreams_solana::instruction::TokenInstruction;
use substreams_solana::pb::sol::v1::Block;

const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const HONEY_CONTRACT_ADDRESS: &str = "4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy";
const HONEY_TOKEN_SPLITTING_CONTRACT: &str = "EEjwuvCMVYjgHUeX1BM9qmUog59Pft88c3jbt2ATwcJw";
const HONEY_TOKEN_DECIMALS: u8 = 9;

#[substreams::handlers::map]
pub fn map_token_splitting_payments(block: Block) -> Result<TokenSplittingContractPayments, Error> {
    let mut rewards = TokenSplittingContractPayments::default();
    let mut payments = vec![];

    for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = &confirmed_trx.transaction {
            let trx_id = bs58::encode(&trx.signatures[0]).into_string();
            if let Some(msg) = &trx.message {
                let mut honey_contract_implication = false;
                let mut honey_token_splitting_contract_implication = false;

                for addr in msg.account_keys.iter() {
                    if bs58::encode(addr).into_string() == HONEY_CONTRACT_ADDRESS {
                        // still don't know for sure that we have a token splitting payment here
                        honey_contract_implication = true;
                    }

                    if bs58::encode(addr).into_string() == HONEY_TOKEN_SPLITTING_CONTRACT {
                        // the token splitting contact is part of the programs involved
                        honey_token_splitting_contract_implication = true;
                    }
                }

                if !honey_contract_implication || !honey_token_splitting_contract_implication {
                    continue;
                }

                if let Some(_) = msg.header {
                    let meta = confirmed_trx.meta.as_ref().unwrap();

                    if meta.inner_instructions.len() != 1 as usize {
                        // seems to work for now, not sure if its good for all transactions
                        continue;
                    }
                    if meta.inner_instructions[0].instructions.len() != 4 as usize {
                        // seems to work for now, not sure if its good for all transactions
                        // what about a splitting contract where we have an owner with 100%
                        continue;
                    }

                    let mut account_1 = Payment::default();
                    let mut account_2 = Payment::default();
                    let mut account_1_set = false;

                    for inner_inst in meta.inner_instructions.iter() {
                        for inst in inner_inst.instructions.iter() {
                            log::info!("len of instructions {}", msg.instructions.len());
                            let program_id = &msg.account_keys[inst.program_id_index as usize];
                            let account_id = bs58::encode(program_id).into_string();
                            log::info!("account_id {} program_id_index: {}", account_id, inst.program_id_index);

                            if account_id != TOKEN_PROGRAM {
                                continue;
                            }

                            // inst account 0 -> mint_info
                            // inst account 1 -> destination_account_info
                            // inst account 2 -> owner_info
                            let account_to = bs58::encode(&msg.account_keys[inst.accounts[1] as usize]).into_string();

                            let instruction = TokenInstruction::unpack(&inst.data)?;
                            match instruction {
                                TokenInstruction::MintTo { amount } => {
                                    if !account_1_set {
                                        account_1.transaction_id = trx_id.clone();
                                        account_1.to = account_to;
                                        account_1.amount = utils::amount_to_decimals(amount as f64, 9 as f64);
                                        account_1_set = true;
                                        continue;
                                    }

                                    account_2.transaction_id = trx_id.clone();
                                    account_2.to = account_to;
                                    account_2.amount = utils::amount_to_decimals(amount as f64, 9 as f64);
                                }
                                _ => {}
                            }
                        }
                    }

                    let payment = TokenSplittingContractPayment {
                        account_one: Some(account_1),
                        account_two: Some(account_2),
                    };

                    payments.push(payment);
                }
            }
        }
    }

    rewards.payments = payments;
    return Ok(rewards);
}
