mod constants;
mod event;
mod payment;
mod pb;
mod utils;

use crate::pb::hivemapper::types::v1::{
    entity, Burn, Entity, InitializedAccount, MapCreate, Mint, Output, Payment, PaymentType, Transaction,
};
use std::ops::Div;
use substreams::errors::Error;
use substreams::log;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::{ConfirmedTransaction, TokenBalance, TransactionStatusMeta};
use substreams_solana::Address;
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

use crate::event::{Event, Type};
use crate::payment::{
    process_pay_imagery_qa_invoice, process_pay_map_consumption_reward, process_pay_operational_reward,
    process_regular_driver_payment,
};
use crate::pb::sol::transactions::v1::Transactions;
use crate::utils::transactions_owned;

#[substreams::handlers::map]
pub fn map_outputs(transactions: Transactions) -> Result<Output, Error> {
    let mut output = Output::default();

    for confirmed_trx in transactions_owned(transactions) {
        let mut transaction: Transaction =
            Transaction::new(bs58::encode(confirmed_trx.transaction.as_ref().unwrap().hash()).into_string());

        for instruction in confirmed_trx.compiled_instructions() {
            process_instruction(&mut transaction, 0, &instruction);
        }

        output.transactions.push(transaction)
    }

    Ok(output)
}

pub fn process_instruction(transaction: &mut Transaction, timestamp: i64, compile_instruction: &InstructionView) {
    match compile_instruction.program_id().to_string().as_ref() {
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM => {
            process_honey_program_instruction(compile_instruction, transaction, timestamp, compile_instruction.meta());
        }

        constants::HONEY_TOKEN_INSTRUCTION_LIB => {
            let instruction_count = compile_instruction.inner_instructions().count();
            if instruction_count == 0 {
                return;
            }

            if instruction_count != 1 {
                panic!("expecting 1 instructions trx {}", transaction.hash);
            }

            process_honey_token_lib(
                compile_instruction,
                &compile_instruction.inner_instructions().nth(0).unwrap(),
                transaction,
                timestamp,
                compile_instruction.meta(),
            );
        }

        constants::SOLANA_TOKEN_PROGRAM => {
            match process_token_instruction(compile_instruction, timestamp, compile_instruction.meta()) {
                Err(err) => {
                    panic!("trx_hash {} process token instructions: {}", transaction.hash, err);
                }
                Ok(ev_option) => {
                    if let Some(ev) = ev_option {
                        match ev.r#type {
                            Type::Mint(mint) => transaction.entities.push(Entity::new_mint(mint)),
                            Type::Burn(burn) => transaction.entities.push(Entity::new_burn(burn)),
                            Type::Transfer(transfer) => transaction.entities.push(Entity::new_transfer(transfer)),
                            Type::InitializeAccount(initialize_account) => transaction
                                .entities
                                .push(Entity::new_initialized_account(initialize_account)),
                        }
                    }
                }
            }
        }
        _ => {
            process_default_inner_instruction(compile_instruction, transaction, timestamp, compile_instruction.meta());
        }
    }
}

pub fn process_honey_token_lib(
    instruction: &InstructionView,
    second_instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    if instruction.program_id().to_string().as_str() != constants::HONEY_TOKEN_INSTRUCTION_LIB {
        panic!(
            "expected instruction of program HONEY_TOKEN_INSTRUCTION_PROGRAM_LIB got {}",
            instruction.program_id().to_string().as_str()
        )
    }

    match instruction.data()[0] {
        constants::HONEY_TOKEN_LIB_INITIALIZE_GLOBAL_STATE => {}
        constants::HONEY_LIB_MAP_CREATE => {
            process_map_create(second_instruction, trx, timestamp, meta);
        }
        constants::HONEY_LIB_MINT_TO => process_mint_to(second_instruction, trx, timestamp, meta),
        constants::HONEY_LIB_BURN => {
            let burn = extract_burn(second_instruction, &trx.hash, timestamp, meta);
            trx.entities.push(Entity::new_burn(burn));
        }
        constants::HONEY_LIB_BURN_AND_ADD_ADDITIONAL_HONEY_SUPPLY => {
            let burn = process_burns(second_instruction, &trx.hash, timestamp, meta);
            trx.entities.push(Entity::new_burn(burn));
        }

        constants::HONEY_LIB_INITIALIZE_CONSUMPTION_REWARD_META => {}

        constants::HONEY_LIB_REINITIALIZE_GLOBAL_STATE => {}
        _ => {
            panic!("instruction program account HONEY_TOKEN_SPLITTING_CONTRACT but found no match trx_hash: {} inst.data: {}", trx.hash, instruction.data()[0]);
        }
    }
}

pub fn process_default_inner_instruction(
    compile_instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    for inner in compile_instruction.inner_instructions() {
        match inner.program_id().to_string().as_ref() {
            constants::SOLANA_TOKEN_PROGRAM => match process_token_instruction(&inner, timestamp, meta) {
                Err(err) => {
                    panic!("trx_hash {} process token instructions {}", trx.hash, err);
                }
                Ok(ev_option) => {
                    if let Some(ev) = ev_option {
                        match ev.r#type {
                            Type::Mint(mint) => trx.entities.push(Entity::new_mint(mint)),
                            Type::Burn(burn) => trx.entities.push(Entity::new_burn(burn)),
                            Type::Transfer(transfer) => trx.entities.push(Entity::new_transfer(transfer)),
                            Type::InitializeAccount(initialize_account) => {
                                trx.entities.push(Entity::new_initialized_account(initialize_account))
                            }
                        }
                    }
                }
            },
            _ => {
                // log::info!("inner not match {} {:?} -- {:?} {}", inner.program_id(), inner.program_id().0, constants::SOLANA_TOKEN_PROGRAM, bs58::encode(constants::SOLANA_TOKEN_PROGRAM).into_string());
            }
        }
    }
}

pub fn process_honey_program_instruction(
    compile_instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    match compile_instruction.data()[0] {
        constants::HONEY_TOKEN_INSTRUCTION_PAY_TO => process_regular_driver_payment(
            &compile_instruction.inner_instructions().nth(1).unwrap(),
            trx,
            timestamp,
            meta,
        ),

        constants::HONEY_TOKEN_INSTRUCTION_CREATE_PAYMENT_INVOICE => {}
        constants::HONEY_TOKEN_INSTRUCTION_INITIALIZE_DEFAULT_PERIOD => {}
        constants::HONEY_TOKEN_INSTRUCTION_INITIALIZE_PAYMENT_PERIOD => {
            if compile_instruction.inner_instructions().count() <= 2 {
                return; //nothing to do
            }
            if compile_instruction.inner_instructions().count() == 3 {
                process_mint_to(
                    &compile_instruction.inner_instructions().nth(2).unwrap(),
                    trx,
                    timestamp,
                    meta,
                );
                return;
            }
            panic!(
                "expecting lest than 3 instructions got {} trx {}",
                compile_instruction.inner_instructions().count(),
                trx.hash
            )
        }
        constants::HONEY_TOKEN_INSTRUCTION_UPDATE_MAP_PROGRESS => {}
        constants::HONEY_TOKEN_INSTRUCTION_CREATE_IMAGERY_QA_INVOICE => {}

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_IMAGERY_QA_INVOICE => process_pay_imagery_qa_invoice(
            &compile_instruction.inner_instructions().nth(1).unwrap(),
            trx,
            timestamp,
            meta,
        ),

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_OPERATIOANL_REWARD => {
            process_pay_operational_reward(
                &compile_instruction.inner_instructions().nth(1).unwrap(),
                trx,
                timestamp,
                compile_instruction.meta(),
            );
        }

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_AND_FORWARD_REWARD_AC => {
            if compile_instruction.inner_instructions().count() == 4 {
                process_token_splitting_fleet_ac(compile_instruction, trx, timestamp, meta);
            }

            if compile_instruction.inner_instructions().count() == 2 {
                process_no_splitting_payments_ac(compile_instruction, trx, timestamp, meta);
            }
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_AND_FORWARD_REWARD_SPLIT_E9 => {
            if compile_instruction.inner_instructions().count() == 4 {
                process_token_splitting_fleet_e9(compile_instruction, trx, timestamp, meta);
            }

            if compile_instruction.inner_instructions().count() == 2 {
                process_no_splitting_payments_e9(compile_instruction, trx, timestamp, meta);
            }
        }
        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_REMOVE_INVOICE => {}
        constants::HONEY_TOKEN_INSTRUCTION_PAY_MAP_COMSUMPTION_REWARD => process_pay_map_consumption_reward(
            &compile_instruction.inner_instructions().nth(1).unwrap(),
            trx,
            timestamp,
            compile_instruction.meta(),
        ),

        constants::HONEY_TOKEN_INSTRUCTION_PROGRAM_PAY_REWARD => process_regular_driver_payment(
            &compile_instruction.inner_instructions().nth(1).unwrap(),
            trx,
            timestamp,
            meta,
        ),
        constants::HONEY_TOKEN_INSTRUCTION_PAY_AND_FORWARD_CONSUMPTION_REWARD => {
            if compile_instruction.inner_instructions().count() == 6 {
                process_token_splitting_fleet_e9(compile_instruction, trx, timestamp, meta);
                let burn = extract_burn(
                    &compile_instruction.inner_instructions().nth(5).unwrap(),
                    &trx.hash,
                    timestamp,
                    meta,
                );
                trx.entities.push(Entity::new_burn(burn));
                return;
            }

            if compile_instruction.inner_instructions().count() == 4 {
                let first_instruction = &compile_instruction.inner_instructions().nth(0).unwrap();
                let third_instruction = &compile_instruction.inner_instructions().nth(2).unwrap();

                if first_instruction.data()[0] == constants::HONEY_LIB_MINT_TO
                    && third_instruction.data()[0] == constants::HONEY_LIB_MINT_TO
                    || first_instruction.data()[0] == constants::HONEY_LIB_MINT_TO_6C
                        && third_instruction.data()[0] == constants::HONEY_LIB_MINT_TO_6C
                {
                    process_token_splitting_fleet_e9(compile_instruction, trx, timestamp, meta);
                    return;
                } else if first_instruction.data()[0] == constants::HONEY_LIB_MINT_TO
                    && third_instruction.data()[0] == constants::HONEY_LIB_BURN
                    || first_instruction.data()[0] == constants::HONEY_LIB_MINT_TO_6C
                        && third_instruction.data()[0] == constants::HONEY_LIB_BURN
                {
                    process_no_splitting_payments_e9(compile_instruction, trx, timestamp, meta);
                    let burn = extract_burn(
                        &compile_instruction.inner_instructions().nth(3).unwrap(),
                        &trx.hash,
                        timestamp,
                        meta,
                    );
                    trx.entities.push(Entity::new_burn(burn));
                    return;
                } else {
                    panic!("unknown instruction pairing trx {}", trx.hash);
                }
            }

            if compile_instruction.inner_instructions().count() == 2 {
                process_no_splitting_payments_e9(compile_instruction, trx, timestamp, meta);
                return;
            }

            panic!(
                "expecting 2 or 4 or 6 instructions got {} trx {}",
                compile_instruction.inner_instructions().count(),
                trx.hash
            )
        }

        constants::HONEY_TOKEN_INSTRUCTION_PAY_CONSUMPTION_REWARD => {
            if compile_instruction.inner_instructions().count() == 4 {
                process_pay_map_consumption_reward(
                    &compile_instruction.inner_instructions().nth(1).unwrap(),
                    trx,
                    timestamp,
                    meta,
                );
                let burn = extract_burn(
                    &compile_instruction.inner_instructions().nth(3).unwrap(),
                    &trx.hash,
                    timestamp,
                    meta,
                );
                trx.entities.push(Entity::new_burn(burn));
                return;
            }
            if compile_instruction.inner_instructions().count() == 2 {
                process_pay_map_consumption_reward(
                    &compile_instruction.inner_instructions().nth(1).unwrap(),
                    trx,
                    timestamp,
                    meta,
                );
                return;
            }
            panic!(
                "expecting 2 or 4 instructions got {} trx {}",
                compile_instruction.inner_instructions().count(),
                trx.hash
            )
        }

        constants::HONEY_TOKEN_INSTRUCTION_PAY_BURST_REWARD => {
            if compile_instruction.inner_instructions().count() == 4 {
                process_regular_driver_payment(
                    &compile_instruction.inner_instructions().nth(1).unwrap(),
                    trx,
                    timestamp,
                    meta,
                );
                let burn = extract_burn(
                    &compile_instruction.inner_instructions().nth(3).unwrap(),
                    &trx.hash,
                    timestamp,
                    meta,
                );
                trx.entities.push(Entity::new_burn(burn));
                return;
            }
            panic!(
                "expecting 4 instructions got {} trx {}",
                compile_instruction.inner_instructions().count(),
                trx.hash
            )
        }

        constants::HONEY_TOKEN_INSTRUCTION_PAY_AND_FORWARD_BURST_REWARD => {
            if compile_instruction.inner_instructions().count() == 6 {
                process_token_splitting_fleet_e9(compile_instruction, trx, timestamp, meta);
                let burn = extract_burn(
                    &compile_instruction.inner_instructions().nth(5).unwrap(),
                    &trx.hash,
                    timestamp,
                    meta,
                );
                trx.entities.push(Entity::new_burn(burn));
                return;
            }

            if compile_instruction.inner_instructions().count() == 4 {
                let first_instruction = &compile_instruction.inner_instructions().nth(0).unwrap();
                let third_instruction = &compile_instruction.inner_instructions().nth(2).unwrap();

                if first_instruction.data()[0] == constants::HONEY_LIB_MINT_TO
                    && third_instruction.data()[0] == constants::HONEY_LIB_MINT_TO
                {
                    process_token_splitting_fleet_e9(compile_instruction, trx, timestamp, meta);
                    return;
                } else if first_instruction.data()[0] == constants::HONEY_LIB_MINT_TO
                    && third_instruction.data()[0] == constants::HONEY_LIB_BURN
                {
                    process_no_splitting_payments_e9(compile_instruction, trx, timestamp, meta);
                    let burn = extract_burn(
                        &compile_instruction.inner_instructions().nth(3).unwrap(),
                        &trx.hash,
                        timestamp,
                        meta,
                    );
                    trx.entities.push(Entity::new_burn(burn));
                    return;
                } else {
                    panic!("unknown instruction pairing");
                }
            }

            if compile_instruction.inner_instructions().count() == 2 {
                process_no_splitting_payments_e9(compile_instruction, trx, timestamp, meta);
                return;
            }

            panic!(
                "expecting 2 or 4 or 6 instructions got {} trx {}",
                compile_instruction.inner_instructions().count(),
                trx.hash
            )
        }

        _ => {
            panic!("instruction program account HONEY_TOKEN_INSTRUCTION_PROGRAM but found no match trx_hash: {} inst.data: {}", trx.hash, compile_instruction.data()[0]);
        }
    }
}

fn extract_mint_to(
    mint_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) -> Mint {
    match crate::process_token_instruction(&mint_instruction, timestamp, meta) {
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
    extract_burn(&instruction, trx_hash, timestamp, meta)
}

fn extract_burn(
    burn_instruction: &InstructionView,
    trx_hash: &String,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) -> Burn {
    match process_token_instruction(&burn_instruction, timestamp, meta) {
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

pub fn process_map_create(
    instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    let burn = process_burns(&instruction, &trx.hash, timestamp, meta);
    trx.entities
        .push(Entity::new_map_create(MapCreate { burn: Some(burn) }));
}
pub fn process_mint_to(
    instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    let mint = extract_mint_to(&instruction, &trx.hash, timestamp, meta);
    trx.entities.push(Entity::new_mint(mint));
}

pub fn process_token_splitting_fleet_ac(
    compile_instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    let fleet_driver_account = &compile_instruction.accounts()[3];
    let fleet_account = &compile_instruction.accounts()[4];

    let mut manager_mint = None;
    let mut driver_mint = None;

    for inner_instruction in compile_instruction.inner_instructions() {
        if inner_instruction.program_id().to_string().as_str() != constants::SOLANA_TOKEN_PROGRAM {
            continue;
        }
        match process_token_instruction(&inner_instruction, timestamp, meta) {
            Err(err) => {
                panic!("trx_hash {} token splitting fleet: {}", trx.hash, err);
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
                                panic!("mint not found! for driver or fleet trx {}", trx.hash);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    if manager_mint.is_some() && driver_mint.is_some() {
        add_token_split_payment_entity(manager_mint, driver_mint, trx);
    } else {
        panic!(
            "Missing a mints {} {} trx {}",
            manager_mint.is_some(),
            driver_mint.is_some(),
            trx.hash
        );
    }
}
pub fn process_token_splitting_fleet_e9(
    compile_instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    let fleet_driver_account = &compile_instruction.accounts()[4];
    let fleet_account = &compile_instruction.accounts()[5];

    let mut manager_mint = None;
    let mut driver_mint = None;

    for inner_instruction in compile_instruction.inner_instructions() {
        if inner_instruction.program_id().to_string().as_str() != constants::SOLANA_TOKEN_PROGRAM {
            continue;
        }
        match process_token_instruction(&inner_instruction, timestamp, meta) {
            Err(err) => {
                panic!("trx_hash {} token splitting fleet: {}", trx.hash, err);
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
                                panic!(
                                    "mint not found! for driver or fleet trx {}, mint to {} fleet {} driver {}",
                                    trx.hash,
                                    mint.to,
                                    fleet_account.to_string(),
                                    fleet_driver_account.to_string()
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if manager_mint.is_some() && driver_mint.is_some() {
        add_token_split_payment_entity(manager_mint, driver_mint, trx);
    } else {
        panic!(
            "Missing a mints {} {} trx {}",
            manager_mint.is_some(),
            driver_mint.is_some(),
            trx.hash
        );
    }
}

fn add_token_split_payment_entity(manager_mint: Option<Mint>, driver_mint: Option<Mint>, trx: &mut Transaction) {
    trx.entities.push(Entity::new_payment(Payment {
        mint: manager_mint,
        r#type: PaymentType::FleetManager as i32,
    }));
    trx.entities.push(Entity::new_payment(Payment {
        mint: driver_mint,
        r#type: PaymentType::FleetDriver as i32,
    }))
}
pub fn process_no_splitting_payments_ac(
    compile_instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    let driver_account = &compile_instruction.accounts()[3];
    let manager_account = &compile_instruction.accounts()[4];

    let mint_instruction = compile_instruction.inner_instructions().nth(1).unwrap();
    let mint = extract_mint_to(&mint_instruction, &trx.hash, timestamp, meta);

    let mut manager_mint = Mint::new(timestamp, manager_account.to_string(), 0.0);
    let mut driver_mint = Mint::new(timestamp, driver_account.to_string(), 0.0);

    if mint.to.eq(&manager_account.to_string()) {
        manager_mint = mint;
    } else if mint.to.eq(&driver_account.to_string()) {
        driver_mint = mint;
    } else {
        panic!("mint not found! for driver or fleet trx {}", trx.hash);
    }

    // FIXME: the method name is no_splitting_payments_ac but we are adding a token split payment...
    add_token_split_payment_entity(Some(manager_mint), Some(driver_mint), trx);
    // output.token_splitting_payments.push(TokenSplittingPayment {
    //     manager_mint: Some(manager_mint),
    //     driver_mint: Some(driver_mint),
    // })
}
pub fn process_no_splitting_payments_e9(
    compile_instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    let driver_account = &compile_instruction.accounts()[4];
    let manager_account = &compile_instruction.accounts()[5];

    let mint_instruction = compile_instruction.inner_instructions().nth(1).unwrap();
    let mint = extract_mint_to(&mint_instruction, &trx.hash, timestamp, meta);

    let mut manager_mint = Mint::new(timestamp, manager_account.to_string(), 0.0);
    let mut driver_mint = Mint::new(timestamp, driver_account.to_string(), 0.0);

    if mint.to.eq(&manager_account.to_string()) {
        manager_mint = mint;
    } else if mint.to.eq(&driver_account.to_string()) {
        driver_mint = mint;
    } else {
        panic!(
            "mint not found! for driver or fleet trx {}, mint to {} fleet {} driver {}",
            trx.hash,
            mint.to,
            manager_account.to_string(),
            driver_account.to_string()
        );
    }

    // FIXME: the method name is no_splitting_payments_ac but we are adding a token split payment...
    add_token_split_payment_entity(Some(manager_mint), Some(driver_mint), trx);
    // output.token_splitting_payments.push(TokenSplittingPayment {
    //     manager_mint: Some(manager_mint),
    //     driver_mint: Some(driver_mint),
    // })
}

pub fn process_token_instruction(
    instruction: &InstructionView,
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

                if is_honey_token_transfer(&meta.pre_token_balances, authority) {
                    let source = &instruction.accounts()[0];
                    let destination = &instruction.accounts()[1];
                    return Ok(Some(Event {
                        r#type: (Type::Transfer(pb::hivemapper::types::v1::Transfer {
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
                if mint.to_string() == constants::HONEY_CONTRACT_ADDRESS {
                    let source = &instruction.accounts()[0];
                    let destination = &instruction.accounts()[2];
                    return Ok(Some(Event {
                        r#type: (Type::Transfer(pb::hivemapper::types::v1::Transfer {
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
                        account: account.to_string(),
                        mint: mint.to_string(),
                        owner: bs58::encode(ow).into_string(),
                    })),
                }));
            }
            _ => {}
        },
    }

    Ok(None)
}

fn amount_to_decimals(amount: f64, decimal: f64) -> f64 {
    let base: f64 = 10.0;
    return amount.div(&(base.powf(decimal)));
}

pub fn is_honey_token_transfer(pre_token_balances: &Vec<TokenBalance>, account: &Address) -> bool {
    for token_balance in pre_token_balances.iter() {
        if token_balance.owner.eq(account.to_string().as_str())
            && token_balance.mint.eq(constants::HONEY_CONTRACT_ADDRESS)
        {
            return true;
        }
    }
    return false;
}
