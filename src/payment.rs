use crate::extract_mint_to;
use crate::pb::hivemapper::types::v1::{Entity, Output, Payment, PaymentType, Transaction};
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::TransactionStatusMeta;

pub fn process_no_token_splitting_payment(
    instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    add_payment_entity(instruction, trx, timestamp, meta, PaymentType::NoSplit)
}

pub fn process_regular_driver_payment(
    instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    add_payment_entity(instruction, trx, timestamp, meta, PaymentType::RegularDriver)
}

pub fn process_pay_ai_trainer_payment(
    instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    add_payment_entity(instruction, trx, timestamp, meta, PaymentType::AiTrainer)
}

pub fn process_pay_imagery_qa_invoice(
    instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    add_payment_entity(instruction, trx, timestamp, meta, PaymentType::AiTrainer)
}

pub fn process_pay_operational_reward(
    instruction_view: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    add_payment_entity(instruction_view, trx, timestamp, meta, PaymentType::Operational);
}

pub fn process_pay_map_consumption_reward(
    instruction_view: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
) {
    add_payment_entity(instruction_view, trx, timestamp, meta, PaymentType::MapConsumption);
}

fn add_payment_entity(
    instruction: &InstructionView,
    trx: &mut Transaction,
    timestamp: i64,
    meta: &TransactionStatusMeta,
    payment_type: PaymentType,
) {
    let mint = extract_mint_to(&instruction, &trx.hash, timestamp, meta);
    trx.entities.push(Entity::new_payment(Payment {
        mint: Some(mint),
        r#type: payment_type as i32,
    }))
}
