mod instruction;
mod option;
mod pb;
mod utils;

use crate::instruction::TokenInstruction;
use crate::pb::hivemapper::Holders;
use crate::pb::{hivemapper, spl};
use substreams::errors::Error;
use substreams::log;

const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const HONEY_CONTRACT_ADDRESS: &str = "4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy";

#[substreams::handlers::map]
pub fn map_holders(block: spl::Block) -> Result<Holders, Error> {
    let holders = Holders::default();

    for confirmed_trx in block.transactions {
        if let Some(meta) = confirmed_trx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            if let Some(trx) = confirmed_trx.transaction {
                let trx_id = bs58::encode(&trx.signatures[0]).into_string();

                if let Some(msg) = trx.message {
                    for inst in msg.instructions {
                        let program_id = &msg.account_keys[inst.program_id_index as usize];
                        let account_id = bs58::encode(program_id).into_string();
                        // log::info!("account id {}", account_id);
                        // if account_id == HONEY_CONTRACT_ADDRESS {
                        //     log::info!("transferring honey {}", trx_id);
                        // }

                        // continue;

                        if account_id != TOKEN_PROGRAM {
                            continue;
                        }

                        let mut native_amount: u64 = 0;
                        let mut from_account_addr = "".to_string();
                        let mut to_account_addr = "".to_string();
                        let mut mint_addr = "".to_string();

                        let instruction = TokenInstruction::unpack(&inst.data)?;
                        match instruction {
                            TokenInstruction::Transfer { amount } => {
                                native_amount = amount;
                                from_account_addr =
                                    bs58::encode(&msg.account_keys[0].to_vec()).into_string();
                                to_account_addr =
                                    bs58::encode(&msg.account_keys[1].to_vec()).into_string();
                            }
                            TokenInstruction::TransferChecked {
                                amount,
                                decimals: _,
                            } => {
                                // todo: are we interested in multi signature owner/delegates?
                                log::info!("trx_id {}", trx_id);
                                native_amount = amount;
                                // from_account_addr =
                                //     bs58::encode(&msg.account_keys[0].to_vec()).into_string();
                                // mint_addr =
                                //     bs58::encode(&msg.account_keys[1].to_vec()).into_string();
                                // to_account_addr =
                                //     bs58::encode(&msg.account_keys[2].to_vec()).into_string();

                                from_account_addr = bs58::encode(
                                    &msg.account_keys[inst.accounts[0] as usize].to_vec(),
                                )
                                .into_string();
                                mint_addr = bs58::encode(
                                    &msg.account_keys[inst.accounts[1] as usize].to_vec(),
                                )
                                .into_string();
                                to_account_addr = bs58::encode(
                                    &msg.account_keys[inst.accounts[2] as usize].to_vec(),
                                )
                                .into_string();

                                // log::info!("amount transferred {}", amount);
                                // log::info!("from_account_addr {}", from_account_addr);
                                // log::info!("to_account_addr {}", to_account_addr);
                                // log::info!("mint_addr {}", mint_addr);
                                //todo: here we have a valid transfer of tokens, we need to emit an
                                // event
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    return Ok(holders);
}
