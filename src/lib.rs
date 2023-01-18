mod pb;
mod instruction;
mod utils;
mod option;

use substreams::errors::Error;
use substreams::log;
use crate::instruction::TokenInstruction;
use crate::pb::hivemapper::Holders;
use crate::pb::{hivemapper, spl};

const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const _HONEY_CONTRACT_ADDRESS: &str = "4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy";

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
                        if bs58::encode(program_id).into_string() != "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" {
                            continue;
                        }
                        log::info!("program_id {}", bs58::encode(program_id).into_string());
                    }
                }
            }
        }
    }

    return Ok(holders);
}
