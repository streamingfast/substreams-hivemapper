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

    for trx in block.transactions {
        if let Some(meta) = trx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            if let Some(transaction) = trx.transaction {
                if let Some(msg) = transaction.message {
                    for inst in msg.instructions {
                        let program_id = &msg.account_keys[inst.program_id_index as usize];
                        if bs58::encode(program_id).into_string()
                            != "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                        {
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
                                from_account_addr = bs58::encode(
                                    &msg.account_keys[inst.accounts[0] as usize].to_vec(),
                                )
                                .into_string();
                                to_account_addr = bs58::encode(
                                    &msg.account_keys[inst.accounts[1] as usize].to_vec(),
                                )
                                .into_string();
                            }
                            TokenInstruction::TransferChecked {
                                amount,
                                decimals: _,
                            } => {
                                native_amount = amount;
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
                            }
                            _ => {}
                        }

                        if mint_addr.eq(HONEY_CONTRACT_ADDRESS) {
                            log::info!("native amount {} from_account_addr {} to_account_addr {} mint_addr {}", native_amount, from_account_addr, to_account_addr, mint_addr);
                        }

                        //todo: instruction data: seems that I have to check the mint and that it is the same as the
                        // HONEY_CONTRACT_ADDRESS
                        // also, julien said something about the transfer checked event -> are we interested in the Transfer event?
                        // as in if we have a transferChecked, does it also emit a transfer event? or there are 2 completely
                        // different things??
                    }
                }
            }
        }
    }

    return Ok(holders);
}
