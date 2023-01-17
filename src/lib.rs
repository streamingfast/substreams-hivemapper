mod pb;

use substreams::errors::Error;
use crate::pb::hivemapper::Holders;
use crate::pb::spl;

const _HONEY_CONTRACT_ADDRESS: &str = "4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy";

#[substreams::handlers::map]
pub fn map_holders(block: spl::Block) -> Result<Holders, Error> {
    let holders = Holders::default();

    for trx in block.transactions {
        if let Some(meta) = trx.meta {
            if let Some(_) = meta.err {
                continue;
            }
        }
    }

    return Ok(holders);
}