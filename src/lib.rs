mod constants;
mod context;
mod event;
mod pb;
mod utils;

use crate::pb::hivemapper::types::v1::Output;
use substreams::errors::Error;
use substreams::store::{StoreGet, StoreGetArray};
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
pub fn map_outputs(block: Block, address_lookup_table_store: StoreGetArray<String>) -> Result<Output, Error> {
    let mut output = Output::default();
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    for confirmed_trx in block.transactions_owned().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = confirmed_trx.transaction {
            let trx_hash = bs58::encode(&trx.signatures[0]).into_string();
            let msg = trx.message.unwrap();
            let mut accounts = vec![];
            let mut writable_accounts = vec![];
            let mut readable_accounts = vec![];
            msg.account_keys
                .into_iter()
                .for_each(|addr| accounts.push(bs58::encode(addr).into_string()));
            msg.address_table_lookups.into_iter().for_each(|addr| {
                let acc = bs58::encode(&addr.account_key).into_string();
                match address_lookup_table_store.get_last(format!("table:{acc}")) {
                    None => panic!("Address Lookup Table Account {} does not exist", acc),
                    Some(accs) => {
                        addr.writable_indexes.into_iter().for_each(|idx| {
                            writable_accounts.push(accs[idx as usize].clone());
                        });
                        addr.readonly_indexes.into_iter().for_each(|idx| {
                            readable_accounts.push(accs[idx as usize].clone());
                        })
                    }
                }
            });

            accounts.append(&mut writable_accounts);
            accounts.append(&mut readable_accounts);

            for (i, compiled_instruction) in msg.instructions.iter().enumerate() {
                utils::process_compiled_instruction(
                    &mut output,
                    timestamp,
                    &trx_hash,
                    confirmed_trx.meta.as_ref().unwrap(),
                    i as u32,
                    compiled_instruction,
                    &accounts,
                )
            }
        }
    }

    Ok(output)
}
