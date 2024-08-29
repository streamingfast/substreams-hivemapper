mod constants;
mod event;
mod pb;

use std::ops::Div;
use crate::pb::hivemapper::types::v1::{AiTrainerPayment, Burn, InitializedAccount, Mint, NoSplitPayment, OperationalPayment, Output, RegularDriverPayment, RewardPayment, TokenSplittingPayment};
use substreams::errors::Error;
use substreams_solana::Address;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, TokenBalance, TransactionStatusMeta};
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;
use crate::event::{Event, Type};

#[substreams::handlers::map]
pub fn map_outputs(block: Block) -> Result<Output, Error> {
    let mut output = Output::default();
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    for confirmed_trx in block.transactions_owned() {
        for instruction in confirmed_trx.compiled_instructions() {
            process_instruction(
                &mut output,
                timestamp,
                &instruction,
            );
        }
    }

    Ok(output)
}


pub fn process_instruction(
    output: &mut Output,
    timestamp: i64,
    compile_instruction: &InstructionView,
) {
    let trx_hash = &bs58::encode(compile_instruction.transaction().hash()).into_string();
    match compile_instruction.program_id().to_string().as_ref() {
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM => {
            process_honey_program_inner_instruction(compile_instruction, trx_hash, timestamp, compile_instruction.meta(), output);
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_LIB => {
            if compile_instruction.inner_instructions().count() != 1 {
                panic!("expecting 1 instructions trx {}", trx_hash );
            }
            process_honey_token_lib(
                compile_instruction,
                &compile_instruction.inner_instructions().nth(0).unwrap(),
                trx_hash,
                timestamp,
                compile_instruction.meta(),
                output
            );
        }
        constants::SOLANA_TOKEN_PROGRAM => {
            match process_token_instruction(compile_instruction, trx_hash, timestamp, compile_instruction.meta()) {
                Err(err) => {
                    panic!("trx_hash {} process token instructions: {}", trx_hash, err);
                }
                Ok(ev_option) => {
                    if let Some(ev) = ev_option {
                        match ev.r#type {
                            Type::Mint(mint) => output.mints.push(mint),
                            Type::Burn(burn) => output.burns.push(burn),
                            Type::Transfer(transfer) => {
                                output.transfers.push(transfer);
                            }
                            Type::InitializeAccount(initialize_account) => {
                                output.initialized_account.push(initialize_account);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            process_default_inner_instruction(compile_instruction, trx_hash, timestamp, compile_instruction.meta(), output);
        }
    }
}

pub fn process_honey_token_lib(
    instruction: &InstructionView,
    secondinstruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    
    if instruction.program_id().to_string().as_str() != constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_LIB {
        panic!("expected instruction of program HONEY_TOKEN_INSTRUCTION_PROGRAM_LIB")
    }
    
    match instruction.data()[0] {
        constants::HONEY_LIB_AI_TRAINER_INSTRUCTION_BYTE => {
            process_pay_ai_trainer_payment(secondinstruction, trx_hash, timestamp, meta, output);
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_LIB_BURN => {
            process_burns(secondinstruction, trx_hash, timestamp, meta, output);
        }
        constants::HONEY_LIB_BURN_AND_ADD_ADDITIONAL_HONEY_SUPPLY => {
            panic!("fix me trx {}", trx_hash);
        }
        constants::HONEY_LIB_BURN => {
            process_burns(secondinstruction, trx_hash, timestamp, meta, output);
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_LIB_CREATE_ACCOUNT => {}
        constants::HONEY_LIB_BURN_MAP_CREDIT => {}
        constants::HONEY_LIB_UPDATE_CREDIT_TO_HONEY_RATE => {}
        _ => {
            panic!("instruction program account HONEY_TOKEN_SPLITTING_CONTRACT but found no match trx_hash: {} inst.data: {}", trx_hash, instruction.data()[0]);
        }
    }
}

pub fn process_default_inner_instruction(
    compile_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    for inner in compile_instruction.inner_instructions() {
        match inner.program_id().to_string().as_ref() {
            constants::SOLANA_TOKEN_PROGRAM => {
                match process_token_instruction(&inner, trx_hash, timestamp, meta) {
                    Err(err) => {
                        panic!("trx_hash {} process token instructions {}", trx_hash, err);
                    }
                    Ok(ev_option) => {
                        if let Some(ev) = ev_option {
                            match ev.r#type {
                                Type::Mint(mint) => output.mints.push(mint),
                                Type::Burn(burn) => output.burns.push(burn),
                                Type::Transfer(transfer) => {
                                    output.transfers.push(transfer);
                                }
                                Type::InitializeAccount(initialize_account) => {
                                    output.initialized_account.push(initialize_account);
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                // log::info!("inner not match {} {:?} -- {:?} {}", inner.program_id(), inner.program_id().0, constants::SOLANA_TOKEN_PROGRAM, bs58::encode(constants::SOLANA_TOKEN_PROGRAM).into_string());
            }
        }
    }
}

pub fn process_honey_program_inner_instruction(
    compile_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    match compile_instruction.data()[0] {
        constants::HONEY_LIB_TOKEN_SPLITTING_INSTRUCTION_BYTE => {
            process_token_splitting_fleet(compile_instruction, trx_hash, timestamp, meta, output);
            return;
        }
        constants::HONEY_LIB_REGULAR_DRIVER_INSTRUCTION_BYTE => {
            process_regular_driver_payment(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                meta,
                output
            )
        }

        constants::HONEY_LIB_NO_TOKEN_SPLITTING_INSTRUCTION_BYTE => {
            process_no_token_splitting_payment(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                meta,
                output
            )
        }

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_IMAGERY_QA_INVOICE => {
            process_pay_imagery_qa_invoice(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                meta,
                output
            )
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_OPERATIOANL_REWARD => {
            panic!("double check if it need is own type or should we handle it like HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_REWARD");
            process_pay_operational_reward(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                meta,
                output
            )
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_REWARD => {
            process_honey_token_lib(
                &compile_instruction.inner_instructions().nth(0).unwrap(),
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                compile_instruction.meta(),
                output
            );

        }
        constants::HONEY_LIB_PAY_AND_FORWARD_REWARD => {
            process_honey_token_lib(
                &compile_instruction.inner_instructions().nth(0).unwrap(),
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                compile_instruction.meta(),
                output
            );
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_CREATE_ACCOUNT => {}
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_CREATE_ACCOUNT_2 => {}
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_CREATE_ACCOUNT_3 => {}
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_CREATE_ACCOUNT_4 => {}
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_REMOVE_INVOICE => {}
        constants::HONEY_TOKEN_INSTRUCTION_UPDATE_MAP_PROGRESS => {}

        _ => {
            panic!("instruction program account HONEY_TOKEN_INSTRUCTION_PROGRAM but found no match trx_hash: {} inst.data: {}", trx_hash, compile_instruction.data()[0]);
        }
    }
}


fn process_pay_operational_reward(
    instruction_view:  &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let mint = extract_mint_to(&instruction_view, trx_hash, timestamp, meta);

    output.operational_payments.push(OperationalPayment {
        mint: Some(mint)
    });
}


fn process_pay_imagery_qa_invoice(
    instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {

    let mint = extract_mint_to(&instruction, trx_hash, timestamp, meta);

    output.ai_trainer_payments.push(AiTrainerPayment {
        mint: Some(mint)
    });
}
fn process_pay_ai_trainer_payment(
    instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let mint = extract_mint_to(&instruction, trx_hash, timestamp, meta);

    output.ai_trainer_payments.push(AiTrainerPayment {
        mint: Some(mint)
    });
}

fn extract_mint_to(
    mint_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) -> Mint {
    match crate::process_token_instruction(&mint_instruction, trx_hash, timestamp, meta) {
        Err(err) => {
            panic!("trx_hash {} token splitting fleet: {}", trx_hash, err);
        }
        Ok(ev_option) => {
            if let Some(ev) = ev_option {
                match ev.r#type {
                    Type::Mint(mint) => {
                        return mint;
                    }
                    _ => {
                        panic!("expecting only mint trx {}", trx_hash)
                    }
                }
            }
        }
    }
    panic!("no mint found")
}

fn process_burns(
    instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let burn = extract_burn(&instruction, trx_hash, timestamp, meta);

    output.burns.push(burn);
}


fn extract_burn(
    burn_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) -> Burn {
    match crate::process_token_instruction(&burn_instruction, trx_hash, timestamp, meta) {
        Err(err) => {
            panic!("trx_hash {} token splitting fleet: {} trx {}", trx_hash, err, trx_hash);
        }
        Ok(ev_option) => {
            if let Some(ev) = ev_option {
                match ev.r#type {
                    Type::Burn(burn) => {
                        return burn;
                    }
                    _ => {
                        panic!("expecting only mint trx {}", trx_hash)
                    }
                }
            }
        }
    }
    panic!("no burn found, trx_hash trx {}", trx_hash)
}

fn process_no_token_splitting_payment(
    instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {

    let mint = extract_mint_to(&instruction, trx_hash, timestamp, meta);

    output.no_split_payments.push(NoSplitPayment {
        mint: Some(mint)
    });
}

pub fn process_regular_driver_payment(
    instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {

    let mint = extract_mint_to(&instruction, trx_hash, timestamp, meta);

    output.regular_driver_payments.push(RegularDriverPayment {
        mint: Some(mint)
    });

    return;
}

pub fn process_token_splitting_fleet(
    compile_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let fleet_driver_account = &compile_instruction.accounts()[3];
    let fleet_account = &compile_instruction.accounts()[4];

    let mut manager_mint = None;
    let mut driver_mint = None;

    for inner_instruction in compile_instruction.inner_instructions() {
        if inner_instruction.program_id().to_string().as_str() != constants::SOLANA_TOKEN_PROGRAM {
            continue;
        }
        match process_token_instruction(&inner_instruction, trx_hash, timestamp, meta) {
            Err(err) => {
                panic!("trx_hash {} token splitting fleet: {}", trx_hash, err);
            }
            Ok(ev_option) => {
                if let Some(ev) = ev_option {
                    match ev.r#type {
                        Type::Mint(mint) => {
                            if mint.to.eq(&fleet_account.to_string()) {
                                manager_mint = Some(mint);
                            } else if mint.to.eq(&fleet_driver_account.to_string()) {
                                driver_mint = Some(mint);
                            } else {
                                panic!("mint not found! for driver or fleet trx {}", trx_hash);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    if manager_mint.is_some() && driver_mint.is_some() {
        output.token_splitting_payments.push(TokenSplittingPayment {
            manager_mint,
            driver_mint,
        })
    } else {
        panic!("Missing a mints {} {} trx {}", manager_mint.is_some(), driver_mint.is_some(), trx_hash);
    }
}


pub fn process_token_instruction(
    instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) -> Result<Option<Event>, Error> {
    match TokenInstruction::unpack(&instruction.data()) {
        Err(err) => {
            return Err(anyhow::anyhow!("unpacking token instruction: {}", err));
        }
        Ok(token_instruction) => match token_instruction {
            TokenInstruction::Transfer { amount: amt } => {
                let authority = &instruction.accounts()[2];

                // let authority = &accounts[inst_accounts[2] as usize];
                if is_honey_token_transfer(&meta.pre_token_balances, authority) {
                    let source = &instruction.accounts()[0];
                    // let source = &accounts[inst_accounts[0] as usize];
                    let destination = &instruction.accounts()[1];
                    // let destination = &accounts[inst_accounts[1] as usize];
                    return Ok(Some(Event {
                        r#type: (Type::Transfer(crate::pb::hivemapper::types::v1::Transfer {
                            trx_hash: trx_hash.to_owned(),
                            timestamp,
                            from: source.to_string(),
                            to: destination.to_string(),
                            amount: amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64),
                        })),
                    }));
                }
            }
            TokenInstruction::TransferChecked { amount: amt, .. } => {
                let mint = &instruction.accounts()[1];
                // let mint = &accounts[inst_accounts[1] as usize];
                if mint.to_string() == constants::HONEY_CONTRACT_ADDRESS {
                    let source = &instruction.accounts()[0];
                    // let source = &accounts[inst_accounts[0] as usize];
                    let destination = &instruction.accounts()[2];
                    // let destination = &accounts[inst_accounts[2] as usize];
                    return Ok(Some(Event {
                        r#type: (Type::Transfer(crate::pb::hivemapper::types::v1::Transfer {
                            trx_hash: trx_hash.to_owned(),
                            timestamp,
                            from: source.to_string(),
                            to: destination.to_string(),
                            amount: amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64),
                        })),
                    }));
                }
            }
            TokenInstruction::MintTo { amount: amt } | TokenInstruction::MintToChecked { amount: amt, .. } => {
                let mint = &instruction.accounts()[0];
                if mint.to_string().as_str() != constants::HONEY_CONTRACT_ADDRESS {
                    return Ok(None);
                }

                let account_to = &instruction.accounts()[1];
                return Ok(Some(Event {
                    r#type: (Type::Mint(Mint {
                        trx_hash: trx_hash.to_owned(),
                        timestamp,
                        to: account_to.to_string(),
                        amount: amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64),
                    })),
                }));
            }
            TokenInstruction::Burn { amount: amt } | TokenInstruction::BurnChecked { amount: amt, .. } => {
                let mint = &instruction.accounts()[1];
                if mint.to_string().as_str() != constants::HONEY_CONTRACT_ADDRESS {
                    return Ok(None);
                }

                let account_from = &instruction.accounts()[0];
                return Ok(Some(Event {
                    r#type: (Type::Burn(Burn {
                        trx_hash: trx_hash.to_owned(),
                        timestamp,
                        from: account_from.to_string(),
                        amount: amount_to_decimals(amt as f64, constants::HONEY_TOKEN_DECIMALS as f64),
                    })),
                }));
            }
            TokenInstruction::InitializeAccount {} => {
                let mint = &instruction.accounts()[1];
                if mint.to_string().as_str() != constants::HONEY_CONTRACT_ADDRESS {
                    return Ok(None);
                }

                let account = &instruction.accounts()[0];
                let owner = &instruction.accounts()[2];
                return Ok(Some(Event {
                    r#type: (Type::InitializeAccount(InitializedAccount {
                        trx_hash: trx_hash.to_owned(),
                        account: account.to_string(),
                        mint: mint.to_string(),
                        owner: owner.to_string(),
                    })),
                }));
            }
            TokenInstruction::InitializeAccount2 { owner: ow } | TokenInstruction::InitializeAccount3 { owner: ow } => {
                let mint = &instruction.accounts()[1];
                if mint.to_string().as_str() != constants::HONEY_CONTRACT_ADDRESS {
                    return Ok(None);
                }

                let account = &instruction.accounts()[0];
                return Ok(Some(Event {
                    r#type: (Type::InitializeAccount(InitializedAccount {
                        trx_hash: trx_hash.to_owned(),
                        account: account.to_string(),
                        mint: mint.to_string(),
                        owner: bs58::encode(ow).into_string(),
                    })),
                }));
            }
            _ => {}
        },
    }

    return Ok(None);
}


fn amount_to_decimals(amount: f64, decimal: f64) -> f64 {
    let base: f64 = 10.0;
    return amount.div(&(base.powf(decimal)));
}

pub fn is_honey_token_transfer(pre_token_balances: &Vec<TokenBalance>, account: &Address) -> bool {
    for token_balance in pre_token_balances.iter() {
        if token_balance.owner.eq(account.to_string().as_str()) && token_balance.mint.eq(constants::HONEY_CONTRACT_ADDRESS) {
            return true;
        }
    }
    return false;
}