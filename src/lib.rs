mod constants;
mod event;
mod pb;

use std::ops::Div;
use crate::pb::hivemapper::types::v1::{AiTrainerPayment, Burn, InitializedAccount, MapConsumptionReward, MapCreate, Mint, NoSplitPayment, OperationalPayment, Output, RegularDriverPayment, RewardPayment, TokenSplittingPayment};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_solana::Address;
use substreams::log;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::{ConfirmedTransaction, TokenBalance, TransactionStatusMeta};
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

use crate::event::{Event, Type};
use crate::pb::sol::transactions::v1::Transactions;

#[substreams::handlers::map]

// pub fn map_outputs(clock: Clock, transactions: Transactions) -> Result<Output, Error> {
pub fn map_outputs(transactions: Transactions) -> Result<Output, Error> {
    let mut output = Output::default();
    // let timestamp = clock.timestamp.as_ref().unwrap();

    for confirmed_trx in transactions_owned(transactions) {
        for instruction in confirmed_trx.compiled_instructions() {
            process_instruction(
                &mut output,
                0,
                &instruction,
            );
        }
    }

    Ok(output)
}

/// Iterates over successful transactions in given block and take ownership.
pub fn transactions_owned(transactions: Transactions) -> impl Iterator<Item=ConfirmedTransaction> {
    transactions.transactions.into_iter().filter(|trx| -> bool {
        if let Some(meta) = &trx.meta {
            return meta.err.is_none();
        }
        false
    })
}


pub fn process_instruction(
    output: &mut Output,
    timestamp: i64,
    compile_instruction: &InstructionView,
) {
    let trx_hash = &bs58::encode(compile_instruction.transaction().hash()).into_string();
    match compile_instruction.program_id().to_string().as_ref() {
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM => {
            process_honey_program_instruction(compile_instruction, trx_hash, timestamp, compile_instruction.meta(), output);
        }
        constants::HONEY_TOKEN_INSTRUCTION_LIB => {
            let instruction_count = compile_instruction.inner_instructions().count();
            if instruction_count == 0 {
                return;
            }
            if instruction_count != 1 {
                panic!("expecting 1 instructions trx {}", trx_hash);
            }
            process_honey_token_lib(
                compile_instruction,
                &compile_instruction.inner_instructions().nth(0).unwrap(),
                trx_hash,
                timestamp,
                compile_instruction.meta(),
                output,
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
    if instruction.program_id().to_string().as_str() != constants::HONEY_TOKEN_INSTRUCTION_LIB {
        panic!("expected instruction of program HONEY_TOKEN_INSTRUCTION_PROGRAM_LIB got {}", instruction.program_id().to_string().as_str())
    }

    match instruction.data()[0] {
        constants::HONEY_TOKEN_LIB_INITIALIZE_GLOBAL_STATE => {}
        constants::HONEY_LIB_MAP_CREATE => {
            process_map_create(secondinstruction, trx_hash, timestamp, meta, output);
        }
        constants::HONEY_LIB_MINT_TO => {
            process_mint_to(secondinstruction, trx_hash, timestamp, meta, output)
        }
        constants::HONEY_LIB_BURN => {
            let burn = extract_burn(secondinstruction, trx_hash, timestamp, meta);
            output.burns.push(burn);
        }
        constants::HONEY_LIB_BURN_AND_ADD_ADDITIONAL_HONEY_SUPPLY => {
            let burn = process_burns(secondinstruction, trx_hash, timestamp, meta);
            output.burns.push(burn);
        }

        constants::HONEY_LIB_INITIALIZE_CONSUMPTION_REWARD_META => {}

        constants::HONEY_LIB_REINITIALIZE_GLOBAL_STATE => {}
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

pub fn process_honey_program_instruction(
    compile_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    match compile_instruction.data()[0] {
        constants::HONEY_TOKEN_INSTRUCTION_PAY_TO => {
            process_regular_driver_payment(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                meta,
                output,
            )
        }

        constants::HONEY_TOKEN_INSTRUCTION_CREATE_PAYMENT_INVOICE => {}
        constants::HONEY_TOKEN_INSTRUCTION_INITIALIZE_DEFAULT_PERIOD => {}
        constants::HONEY_TOKEN_INSTRUCTION_INITIALIZE_PAYMENT_PERIOD => {
            if compile_instruction.inner_instructions().count() <= 2 {
                return; //nothing to do
            }
            if compile_instruction.inner_instructions().count() == 3 {
                process_mint_to(
                    &compile_instruction.inner_instructions().nth(2).unwrap(),
                    trx_hash,
                    timestamp,
                    meta,
                    output,
                );
                return
            }
            panic!("expecting lest than 3 instructions got {} trx {}", compile_instruction.inner_instructions().count(), trx_hash)
        }
        constants::HONEY_TOKEN_INSTRUCTION_UPDATE_MAP_PROGRESS => {}
        constants::HONEY_TOKEN_INSTRUCTION_CREATE_IMAGERY_QA_INVOICE => {}

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_IMAGERY_QA_INVOICE => {
            process_pay_imagery_qa_invoice(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                meta,
                output,
            )
        }

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_OPERATIOANL_REWARD => {
            process_pay_operational_reward(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                compile_instruction.meta(),
                output,
            );
        }

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_AND_FORWARD_REWARD_AC => {
            if compile_instruction.inner_instructions().count() == 4 {
                process_token_splitting_fleet_ac(compile_instruction, trx_hash, timestamp, meta, output);
            }

            if compile_instruction.inner_instructions().count() == 2 {
                process_no_splitting_payments_ac(compile_instruction, trx_hash, timestamp, meta, output);
            }
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_AND_FORWARD_REWARD_SPLIT_E9 => {
            if compile_instruction.inner_instructions().count() == 4 {
                process_token_splitting_fleet_e9(compile_instruction, trx_hash, timestamp, meta, output);
            }

            if compile_instruction.inner_instructions().count() == 2 {
                process_no_splitting_payments_e9(compile_instruction, trx_hash, timestamp, meta, output);
            }
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_REMOVE_INVOICE => {}
        constants::HONEY_TOKEN_INSTRUCTION_PAY_MAP_COMSUMPTION_REWARD => {
            process_pay_map_consumption_reward(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                compile_instruction.meta(),
                output,
            )
        }

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_REWARD => {
            process_regular_driver_payment(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx_hash,
                timestamp,
                meta,
                output,
            )
        }
        constants::HONEY_TOKEN_INSTRUCTION_PAY_AND_FORWARD_CONSUMPTION_REWARD => {
            if compile_instruction.inner_instructions().count() == 6 {
                process_token_splitting_fleet_e9(compile_instruction, trx_hash, timestamp, meta, output);
                let burn = extract_burn(
                    &compile_instruction.inner_instructions().nth(5).unwrap(),
                    trx_hash,
                    timestamp,
                    meta,
                );
                output.burns.push(burn);
                return;
            }

            if compile_instruction.inner_instructions().count() == 4 {
                let firstInstuction = &compile_instruction.inner_instructions().nth(0).unwrap();
                let thirdInstuction = &compile_instruction.inner_instructions().nth(2).unwrap();

                if firstInstuction.data()[0] == constants::HONEY_LIB_MINT_TO && thirdInstuction.data()[0] == constants::HONEY_LIB_MINT_TO ||
                    firstInstuction.data()[0] == constants::HONEY_LIB_MINT_TO_6C && thirdInstuction.data()[0] == constants::HONEY_LIB_MINT_TO_6C {
                    process_token_splitting_fleet_e9(compile_instruction, trx_hash, timestamp, meta, output);
                    return;
                } else if firstInstuction.data()[0] == constants::HONEY_LIB_MINT_TO && thirdInstuction.data()[0] == constants::HONEY_LIB_BURN ||
                    firstInstuction.data()[0] == constants::HONEY_LIB_MINT_TO_6C && thirdInstuction.data()[0] == constants::HONEY_LIB_BURN {
                    process_no_splitting_payments_e9(compile_instruction, trx_hash, timestamp, meta, output);
                    let burn = extract_burn(
                        &compile_instruction.inner_instructions().nth(3).unwrap(),
                        trx_hash,
                        timestamp,
                        meta,
                    );
                    output.burns.push(burn);
                    return;
                } else {
                    panic!("unknown instruction pairing trx {}", trx_hash);
                }
            }

            if compile_instruction.inner_instructions().count() == 2 {
                process_no_splitting_payments_e9(compile_instruction, trx_hash, timestamp, meta, output);
                return;
            }

            panic!("expecting 2 or 4 or 6 instructions got {} trx {}", compile_instruction.inner_instructions().count(), trx_hash)
        }

        constants::HONEY_TOKEN_INSTRUCTION_PAY_CONSUMPTION_REWARD => {
            if compile_instruction.inner_instructions().count() == 4 {
                process_pay_map_consumption_reward(
                    &compile_instruction.inner_instructions().nth(1).unwrap(),
                    trx_hash,
                    timestamp,
                    meta,
                    output,
                );
                let burn = extract_burn(
                    &compile_instruction.inner_instructions().nth(3).unwrap(),
                    trx_hash,
                    timestamp,
                    meta,
                );
                output.burns.push(burn);
                return;
            }
            if compile_instruction.inner_instructions().count() == 2 {
                process_pay_map_consumption_reward(
                    &compile_instruction.inner_instructions().nth(1).unwrap(),
                    trx_hash,
                    timestamp,
                    meta,
                    output,
                );
                return;
            }
            panic!("expecting 2 or 4 instructions got {} trx {}", compile_instruction.inner_instructions().count(), trx_hash)
        }

        constants::HONEY_TOKEN_INSTRUCTION_PAY_BURST_REWARD => {
            if compile_instruction.inner_instructions().count() == 4 {
                process_regular_driver_payment(
                    &compile_instruction.inner_instructions().nth(1).unwrap(),
                    trx_hash,
                    timestamp,
                    meta,
                    output,
                );
                let burn = extract_burn(
                    &compile_instruction.inner_instructions().nth(3).unwrap(),
                    trx_hash,
                    timestamp,
                    meta,
                );
                output.burns.push(burn);
                return;
            }
            panic!("expecting 4 instructions got {} trx {}", compile_instruction.inner_instructions().count(), trx_hash)
        }

        constants::HONEY_TOKEN_INSTRUCTION_PAY_AND_FORWARD_BURST_REWARD => {
            if compile_instruction.inner_instructions().count() == 6 {
                process_token_splitting_fleet_e9(compile_instruction, trx_hash, timestamp, meta, output);
                let burn = extract_burn(
                    &compile_instruction.inner_instructions().nth(5).unwrap(),
                    trx_hash,
                    timestamp,
                    meta,
                );
                output.burns.push(burn);
                return;
            }

            if compile_instruction.inner_instructions().count() == 4 {
                let firstInstuction = &compile_instruction.inner_instructions().nth(0).unwrap();
                let thirdInstuction = &compile_instruction.inner_instructions().nth(2).unwrap();

                if firstInstuction.data()[0] == constants::HONEY_LIB_MINT_TO && thirdInstuction.data()[0] == constants::HONEY_LIB_MINT_TO {
                    process_token_splitting_fleet_e9(compile_instruction, trx_hash, timestamp, meta, output);
                    return;
                } else if firstInstuction.data()[0] == constants::HONEY_LIB_MINT_TO && thirdInstuction.data()[0] == constants::HONEY_LIB_BURN {
                    process_no_splitting_payments_e9(compile_instruction, trx_hash, timestamp, meta, output);
                    let burn = extract_burn(
                        &compile_instruction.inner_instructions().nth(3).unwrap(),
                        trx_hash,
                        timestamp,
                        meta,
                    );
                    output.burns.push(burn);
                    return;
                } else {
                    panic!("unknown instruction pairing");
                }
            }

            if compile_instruction.inner_instructions().count() == 2 {
                process_no_splitting_payments_e9(compile_instruction, trx_hash, timestamp, meta, output);
                return;
            }

            panic!("expecting 2 or 4 or 6 instructions got {} trx {}", compile_instruction.inner_instructions().count(), trx_hash)
        }

        _ => {
            panic!("instruction program account HONEY_TOKEN_INSTRUCTION_PROGRAM but found no match trx_hash: {} inst.data: {}", trx_hash, compile_instruction.data()[0]);
        }
    }
}


fn process_pay_operational_reward(
    instruction_view: &InstructionView,
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
fn process_pay_map_consumption_reward(
    instruction_view: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let mint = extract_mint_to(&instruction_view, trx_hash, timestamp, meta);

    output.map_consumption_reward.push(MapConsumptionReward {
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
) -> Burn {
    let burn = extract_burn(&instruction, trx_hash, timestamp, meta);

    return burn;
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

pub fn process_map_create(
    instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let burn = process_burns(&instruction, trx_hash, timestamp, meta);

    output.map_create.push(MapCreate {
        burn: Some(burn)
    });

    return;
}
pub fn process_mint_to(
    instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let mint = extract_mint_to(&instruction, trx_hash, timestamp, meta);

    output.mints.push(mint);

    return;
}

pub fn process_token_splitting_fleet_ac(
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
pub fn process_token_splitting_fleet_e9(
    compile_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let fleet_driver_account = &compile_instruction.accounts()[4];
    let fleet_account = &compile_instruction.accounts()[5];

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
                                for a in compile_instruction.accounts() {
                                    log::info!("{}", a.to_string())
                                }
                                // return;
                                panic!("mint not found! for driver or fleet trx {}, mint to {} fleet {} driver {}", trx_hash, mint.to, fleet_account.to_string(), fleet_driver_account.to_string());
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
pub fn process_no_splitting_payments_ac(
    compile_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let driver_account = &compile_instruction.accounts()[3];
    let manager_account = &compile_instruction.accounts()[4];

    let mint_instruction = compile_instruction.inner_instructions().nth(1).unwrap();
    let mint = extract_mint_to(&mint_instruction, trx_hash, timestamp, meta);

    let mut manager_mint = Mint {
        trx_hash: trx_hash.to_owned(),
        timestamp,
        to: manager_account.to_string(),
        amount: 0.0,
    };
    let mut driver_mint = Mint {
        trx_hash: trx_hash.to_owned(),
        timestamp,
        to: driver_account.to_string(),
        amount: 0.0,
    };

    if mint.to.eq(&manager_account.to_string()) {
        manager_mint = mint;
    } else if mint.to.eq(&driver_account.to_string()) {
        driver_mint = mint;
    } else {
        panic!("mint not found! for driver or fleet trx {}", trx_hash);
    }

    output.token_splitting_payments.push(TokenSplittingPayment {
        manager_mint: Some(manager_mint),
        driver_mint: Some(driver_mint),
    })
}
pub fn process_no_splitting_payments_e9(
    compile_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    output: &mut Output,
) {
    let driver_account = &compile_instruction.accounts()[4];
    let manager_account = &compile_instruction.accounts()[5];

    let mint_instruction = compile_instruction.inner_instructions().nth(1).unwrap();
    let mint = extract_mint_to(&mint_instruction, trx_hash, timestamp, meta);

    let mut manager_mint = Mint {
        trx_hash: trx_hash.to_owned(),
        timestamp,
        to: manager_account.to_string(),
        amount: 0.0,
    };
    let mut driver_mint = Mint {
        trx_hash: trx_hash.to_owned(),
        timestamp,
        to: driver_account.to_string(),
        amount: 0.0,
    };

    if mint.to.eq(&manager_account.to_string()) {
        manager_mint = mint;
    } else if mint.to.eq(&driver_account.to_string()) {
        driver_mint = mint;
    } else {
        panic!("mint not found! for driver or fleet trx {}, mint to {} fleet {} driver {}", trx_hash, mint.to, manager_account.to_string(), driver_account.to_string());
    }

    output.token_splitting_payments.push(TokenSplittingPayment {
        manager_mint: Some(manager_mint),
        driver_mint: Some(driver_mint),
    })
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