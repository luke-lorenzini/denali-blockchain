use denali::{Message, vote::Votes, create_new_block, chain::{Chain, TRANSACTIONS_PER_BLOCK}, process_transactions};

fn create_transaction_pool() -> Vec<Message<Votes>> {
    let mut messages = Vec::new();
    for _ in 0..1_000_000 {
        let message = generate_test_vote_data();
        messages.push(message);
    }
    messages
}

fn generate_test_vote_data() -> Message<Votes> {
    let payload = r#"
    {
        "candidate": 0
    }"#
    .into();
    let number_of_candidates = 3;
    let votes = Votes::new(number_of_candidates);
    Message {
        payload,
        program: votes
    }
}

#[test]
fn test_add_new_block_to_chain() {
    let mut chain = Chain::new();
    let previous_block_hash = "0".into();
    let transactions = create_transaction_pool();
    let block = create_new_block(previous_block_hash, transactions);
    chain.add_next_block(block);
    assert_eq!(chain.count, 2);
}

#[test]
fn test_process_transactions() {
    let transactions = create_transaction_pool();
    let res = process_transactions(transactions);
    let expected = "1a100baed95a65f66d01cd08644b28e134783fd0c52ac7e35ad52a452e8b90b2";
    assert_eq!(res, expected);
}

#[test]
fn test_create_new_block() {
    let previous_block_hash = "0".into();
    let transactions = create_transaction_pool();
    let res = create_new_block(previous_block_hash, transactions);
    let expected = "1a100baed95a65f66d01cd08644b28e134783fd0c52ac7e35ad52a452e8b90b2";
    assert_eq!(res.header.merkle_tree_root, expected);
    assert_eq!(res.transaction_count, TRANSACTIONS_PER_BLOCK);
}
