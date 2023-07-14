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

// #[substreams::handlers::map]
// pub fn map_driver_payments(block: Block) -> Result<DriverPayments, Error> {
//     let mut payments = vec![];
//     let timestamp = block.block_time.as_ref().unwrap().timestamp;
//
//     for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
//         if let Some(trx) = &confirmed_trx.transaction {
//             let trx_id = bs58::encode(&trx.signatures[0]).into_string();
//             if let Some(msg) = &trx.message {
//                 let mut honey_driver_payment = false;
//                 let mut honey_driver_no_split_payment = false;
//                 let mut driver_account = String::new();
//
//                 for inst in msg.instructions.iter() {
//                     let instruction_program_account =
//                         bs58::encode(&msg.account_keys[inst.program_id_index as usize]).into_string();
//
//                     if instruction_program_account != constants::HONEY_TOKEN_SPLITTING_INSTRUCTION_PROGRAM {
//                         continue;
//                     }
//
//                     let token_account = bs58::encode(&msg.account_keys[inst.accounts[1] as usize]).into_string();
//                     if token_account != constants::HONEY_CONTRACT_ADDRESS {
//                         continue;
//                     }
//
//                     if inst.data[0] == constants::HONEY_REGULAR_DRIVER_INSTRUCTION_BYTE {
//                         honey_driver_payment = true;
//                         driver_account = bs58::encode(&msg.account_keys[inst.accounts[2] as usize]).into_string();
//                         break;
//                     }
//
//                     if inst.data[0] == constants::HONEY_NO_TOKEN_SPLITTING_INSTRUCTION_BYTE {
//                         honey_driver_no_split_payment = true;
//                         driver_account = bs58::encode(&msg.account_keys[inst.accounts[2] as usize]).into_string();
//                         break;
//                     }
//                 }
//
//                 if !honey_driver_payment && !honey_driver_no_split_payment {
//                     continue;
//                 }
//
//                 if let Some(_) = msg.header {
//                     let meta = confirmed_trx.meta.as_ref().unwrap();
//                     let mut driver_payment = Payment::default();
//
//                     for inner_inst in meta.inner_instructions.iter() {
//                         for inst in inner_inst.instructions.iter() {
//                             let program_id = &msg.account_keys[inst.program_id_index as usize];
//                             let account_id = bs58::encode(program_id).into_string();
//
//                             if account_id != constants::TOKEN_PROGRAM {
//                                 continue;
//                             }
//
//                             let account_to = utils::fetch_account_to(&msg.account_keys, inst.accounts[1]);
//
//                             let instruction = TokenInstruction::unpack(&inst.data)?;
//                             match instruction {
//                                 TokenInstruction::MintTo { amount } => {
//                                     if account_to == driver_account {
//                                         driver_payment.timestamp = timestamp;
//                                         driver_payment.transaction_id = trx_id.clone();
//                                         driver_payment.address = account_to;
//                                         driver_payment.amount = utils::amount_to_decimals(
//                                             amount as f64,
//                                             constants::HONEY_TOKEN_DECIMALS as f64,
//                                         );
//                                     }
//                                 }
//                                 _ => {}
//                             }
//                         }
//                     }
//
//                     let mut dr = DriverPayment {
//                         payment: Some(driver_payment),
//                         r#type: 0,
//                     };
//
//                     if honey_driver_payment {
//                         dr.r#type = i32::from(Regular);
//                     }
//
//                     if honey_driver_no_split_payment {
//                         dr.r#type = i32::from(NoSplit);
//                     }
//
//                     payments.push(dr);
//                 }
//             }
//         }
//     }
//
//     Ok(DriverPayments { rewards: payments })
// }
//
// #[substreams::handlers::map]
// pub fn map_ai_trainer_payments(block: Block) -> Result<AiTrainerPayments, Error> {
//     let mut payments = vec![];
//     let timestamp = block.block_time.as_ref().unwrap().timestamp;
//
//     for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
//         if let Some(trx) = &confirmed_trx.transaction {
//             let trx_id = bs58::encode(&trx.signatures[0]).into_string();
//             if let Some(msg) = &trx.message {
//                 let mut honey_ai_trainer_payment = false;
//                 let mut driver_account = String::new();
//
//                 for inst in msg.instructions.iter() {
//                     let instruction_program_account =
//                         bs58::encode(&msg.account_keys[inst.program_id_index as usize]).into_string();
//
//                     if instruction_program_account != constants::HONEY_TOKEN_SPLITTING_CONTRACT {
//                         continue;
//                     }
//
//                     let token_account = bs58::encode(&msg.account_keys[inst.accounts[1] as usize]).into_string();
//                     if token_account != constants::HONEY_CONTRACT_ADDRESS {
//                         continue;
//                     }
//
//                     if inst.data[0] == constants::HONEY_AI_TRAINER_INSTRUCTION_BYTE {
//                         honey_ai_trainer_payment = true;
//                         driver_account = bs58::encode(&msg.account_keys[inst.accounts[2] as usize]).into_string();
//                         break;
//                     }
//                 }
//
//                 if !honey_ai_trainer_payment {
//                     continue;
//                 }
//
//                 if let Some(_) = msg.header {
//                     let meta = confirmed_trx.meta.as_ref().unwrap();
//                     let mut driver_payment = Payment::default();
//
//                     for inner_inst in meta.inner_instructions.iter() {
//                         for inst in inner_inst.instructions.iter() {
//                             let program_id = &msg.account_keys[inst.program_id_index as usize];
//                             let account_id = bs58::encode(program_id).into_string();
//
//                             if account_id != constants::TOKEN_PROGRAM {
//                                 continue;
//                             }
//
//                             let account_to = utils::fetch_account_to(&msg.account_keys, inst.accounts[1]);
//
//                             let instruction = TokenInstruction::unpack(&inst.data)?;
//                             match instruction {
//                                 TokenInstruction::MintTo { amount } => {
//                                     if account_to == driver_account {
//                                         driver_payment.timestamp = timestamp;
//                                         driver_payment.transaction_id = trx_id.clone();
//                                         driver_payment.address = account_to;
//                                         driver_payment.amount = utils::amount_to_decimals(
//                                             amount as f64,
//                                             constants::HONEY_TOKEN_DECIMALS as f64,
//                                         );
//                                     }
//                                 }
//                                 _ => {}
//                             }
//                         }
//                     }
//
//                     payments.push(AiTrainerPayment {
//                         payment: Some(driver_payment),
//                     });
//                 }
//             }
//         }
//     }
//
//     Ok(AiTrainerPayments { payments })
// }
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
