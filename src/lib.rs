mod constants;
mod context;
mod event;
mod pb;
mod utils;

use crate::pb::hivemapper::types::v1::Output;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
pub fn map_outputs(block: Block) -> Result<Output, Error> {
    let mut output = Output::default();
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    for confirmed_trx in block.transactions_owned() {
        if let Some(trx) = confirmed_trx.transaction {
            let trx_hash = bs58::encode(&trx.signatures[0]).into_string();
            let msg = trx.message.unwrap();
            let mut accounts = vec![];
            msg.account_keys
                .into_iter()
                .for_each(|addr| accounts.push(bs58::encode(addr).into_string()));

            let meta = confirmed_trx.meta.as_ref().unwrap();
            for (i, compiled_instruction) in msg.instructions.iter().enumerate() {
                utils::process_compiled_instruction(
                    &mut output,
                    timestamp,
                    &trx_hash,
                    meta,
                    i as u32,
                    compiled_instruction,
                    &accounts,
                );
            }
        }
    }

    Ok(output)
}
