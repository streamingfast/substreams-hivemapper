mod constants;
mod context;
mod pb;
mod utils;

use crate::pb::hivemapper::types::v1::{Output, TokenSplittingPayment, Transfer};
use substreams::errors::Error;
use substreams_solana::pb::sol::v1::Block;

#[substreams::handlers::map]
pub fn map_outputs(block: Block) -> Result<Output, Error> {
    let mut output = Output::default();
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = &confirmed_trx.transaction {
            let trx_hash = bs58::encode(&trx.signatures[0]).into_string();
            let msg = trx.message.as_ref().unwrap();
            let accounts = &msg.account_keys;

            for (i, compiled_instruction) in trx.message.as_ref().unwrap().instructions.iter().enumerate() {
                utils::process_compiled_instruction(
                    &mut output,
                    timestamp,
                    &trx_hash,
                    confirmed_trx.meta.as_ref().unwrap(),
                    i as u32,
                    compiled_instruction,
                    accounts,
                )
            }
        }
    }

    Ok(output)
}

//
// //TODO: create the address lookup table substreams and if the program index is > len of msg.accounts
// // then we need to lookup from the address loopk up table substreams
// #[substreams::handlers::map]
// pub fn map_transfers(block: Block) -> Result<Transfers, Error> {
//     let mut transfers = vec![];
//     let timestamp = block.block_time.as_ref().unwrap().timestamp;
//     let mut trx_id = String::new();
//     let mut msg_accounts = vec![];
//     let msg_accounts_set = false;
//
//     for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
//         if let Some(trx) = &confirmed_trx.transaction {
//             trx_id = bs58::encode(&trx.signatures[0]).into_string();
//             if let Some(msg) = &trx.message {
//                 if !msg_accounts_set {
//                     msg_accounts = msg.clone().account_keys;
//                 }
//
//                 for inst in msg.instructions.iter() {
//                     let instruction_program_account =
//                         bs58::encode(&msg_accounts[inst.program_id_index as usize]).into_string();
//
//                     if instruction_program_account != constants::TOKEN_PROGRAM {
//                         continue;
//                     }
//
//                     let source = bs58::encode(&msg_accounts[inst.accounts[0] as usize]).into_string();
//                     let destination = bs58::encode(&msg_accounts[inst.accounts[1] as usize]).into_string();
//                     let authority = bs58::encode(&msg_accounts[inst.accounts[2] as usize]).into_string();
//
//                     let instruction = TokenInstruction::unpack(&inst.data)?;
//                     let mut amount = 0.0;
//                     match instruction {
//                         TokenInstruction::Transfer { amount: amt } => {
//                             amount = utils::amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64);
//                         }
//                         _ => {}
//                     }
//
//                     if let Some(meta) = &confirmed_trx.meta {
//                         if utils::valid_honey_token_transfer(&meta.pre_token_balances, &authority) {
//                             transfers.push(Transfer {
//                                 trx_id: trx_id.clone(),
//                                 timestamp,
//                                 from: source,
//                                 to: destination,
//                                 amount,
//                             });
//                         }
//                     }
//                 }
//             }
//         }
//         if let Some(meta) = &confirmed_trx.meta {
//             for inner_transaction in meta.inner_instructions.iter() {
//                 for inst in inner_transaction.instructions.iter() {
//                     if trx_id
//                         == "2uYRfNxMjM4uWKHQ5xckX2F8W3skNwYcSU7nEpZxdvAFdznwzG8sFJyYSQyRs4M2S2EmLkq4Bus2v79XKNzkFywe"
//                             .to_owned()
//                     {
//                         log::info!("trx {:?}", confirmed_trx.meta);
//                     }
//                     let instruction_program_account =
//                         bs58::encode(&msg_accounts[inst.program_id_index as usize]).into_string();
//
//                     if instruction_program_account != constants::TOKEN_PROGRAM {
//                         continue;
//                     }
//
//                     let source = bs58::encode(&msg_accounts[inst.accounts[0] as usize]).into_string();
//                     let destination = bs58::encode(&msg_accounts[inst.accounts[1] as usize]).into_string();
//                     let authority = bs58::encode(&msg_accounts[inst.accounts[2] as usize]).into_string();
//
//                     let instruction = TokenInstruction::unpack(&inst.data)?;
//                     let mut amount = 0.0;
//                     match instruction {
//                         TokenInstruction::Transfer { amount: amt } => {
//                             amount = utils::amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64);
//                         }
//                         _ => continue,
//                     }
//
//                     if let Some(meta) = &confirmed_trx.meta {
//                         if utils::valid_honey_token_transfer(&meta.pre_token_balances, &authority) {
//                             transfers.push(Transfer {
//                                 trx_id: trx_id.clone(),
//                                 timestamp,
//                                 from: source,
//                                 to: destination,
//                                 amount,
//                             });
//                         }
//                     }
//                 }
//             }
//         }
//     }
//
//     Ok(Transfers { transfers })
// }
