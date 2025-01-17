use crate::cep47::{token_cfg, CasperCEP47Contract, Meta, TokenId};
use casper_types::U256;

mod meta {
    use super::Meta;
    use maplit::btreemap;

    pub fn red_dragon() -> Meta {
        btreemap! {
            "color".to_string() => "red".to_string()
        }
    }

    pub fn blue_dragon() -> Meta {
        btreemap! {
            "color".to_string() => "blue".to_string()
        }
    }

    pub fn black_dragon() -> Meta {
        btreemap! {
            "color".to_string() => "black".to_string()
        }
    }

    pub fn gold_dragon() -> Meta {
        btreemap! {
            "color".to_string() => "gold".to_string()
        }
    }
}

#[test]
fn test_deploy() {
    let contract = CasperCEP47Contract::deploy();

    assert_eq!(contract.name(), token_cfg::NAME);
    assert_eq!(contract.symbol(), token_cfg::SYMBOL);
    assert_eq!(contract.meta(), token_cfg::contract_meta());
    assert_eq!(contract.total_supply(), U256::zero());
}

#[test]
fn test_token_meta() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = String::from("custom_token_id");
    let token_meta = meta::red_dragon();

    contract.mint_one(
        &contract.ali.clone(),
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );

    let ali_token_meta = contract.token_meta(&token_id).unwrap();
    assert_eq!(ali_token_meta, token_meta);

    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);
    assert_eq!(ali_tokens, vec![token_id]);
}

#[test]
fn test_mint_one_with_random_token_id() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_meta = meta::red_dragon();
    contract.mint_one(
        &contract.ali.clone(),
        None,
        &token_meta,
        &contract.admin.clone(),
    );

    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.balance_of(&contract.ali), U256::one());

    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::one());
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(contract.ali));
}

#[test]
fn test_mint_one_with_set_token_id() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();
    contract.mint_one(
        &contract.ali.clone(),
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);
    assert_eq!(ali_tokens, vec![token_id.clone()]);
    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.balance_of(&contract.ali), U256::one());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::one());
    assert_eq!(contract.owner_of(&token_id), Some(contract.ali));
}

#[test]
#[should_panic]
fn test_mint_one_with_not_unique_token_id() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();
    contract.mint_one(
        &contract.ali.clone(),
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );
    contract.mint_one(
        &contract.ali.clone(),
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );
}

#[test]
fn test_mint_copies() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_meta = meta::gold_dragon();
    contract.mint_copies(
        &contract.ali.clone(),
        None,
        &token_meta,
        3,
        &contract.admin.clone(),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);
    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(&contract.ali), U256::from(3));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(3));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(contract.ali));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(contract.ali));
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(contract.ali));
}

#[test]
fn test_mint_many() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::red_dragon()];
    contract.mint_many(
        &contract.ali.clone(),
        None,
        &token_metas,
        &contract.admin.clone(),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(&contract.ali), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(contract.ali));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(contract.ali));
}

#[test]
fn test_burn_many() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![
        meta::gold_dragon(),
        meta::blue_dragon(),
        meta::black_dragon(),
        meta::red_dragon(),
    ];
    contract.mint_many(
        &contract.ali.clone(),
        None,
        &token_metas,
        &contract.admin.clone(),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);
    println!("{:?}", ali_tokens);
    println!("{:?}", ali_tokens.first().unwrap().clone());

    contract.burn_many(
        &contract.ali.clone(),
        &vec![
            ali_tokens.first().unwrap().clone(),
            ali_tokens.last().unwrap().clone(),
        ],
        &contract.admin.clone(),
    );
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(&contract.ali), U256::from(2));

    let ali_tokens = contract.tokens(&contract.ali);
    println!("{:?}", ali_tokens);
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
}

#[test]
fn test_burn_one() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::red_dragon()];
    contract.mint_many(
        &contract.ali.clone(),
        None,
        &token_metas,
        &contract.admin.clone(),
    );

    let ali_tokens = contract.tokens(&contract.ali);

    contract.burn_one(
        &contract.ali.clone(),
        ali_tokens.first().unwrap(),
        &contract.admin.clone(),
    );
    assert_eq!(contract.total_supply(), U256::from(1));
    assert_eq!(contract.balance_of(&contract.ali), U256::from(1));

    let ali_tokens = contract.tokens(&contract.ali);
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(1));
}

#[test]
fn test_transfer_token() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::blue_dragon()];
    contract.mint_many(
        &contract.ali.clone(),
        None,
        &token_metas,
        &contract.admin.clone(),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);

    contract.transfer_token(&contract.bob.clone(), &ali_tokens[1], &contract.ali.clone());

    assert_eq!(contract.balance_of(&contract.ali), U256::from(1));
    assert_eq!(contract.balance_of(&contract.bob), U256::from(1));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(contract.ali));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(contract.bob));
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![
        meta::gold_dragon(),
        meta::black_dragon(),
        meta::black_dragon(),
    ];
    contract.mint_many(
        &contract.ali.clone(),
        None,
        &token_metas,
        &contract.admin.clone(),
    );
    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);
    contract.transfer_many_tokens(
        &contract.bob.clone(),
        &ali_tokens[..2].to_vec(),
        &contract.ali.clone(),
    );

    assert_eq!(contract.balance_of(&contract.ali), U256::from(1));
    assert_eq!(contract.balance_of(&contract.bob), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(contract.bob));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(contract.bob));
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(contract.ali));
}

#[test]
fn test_transfer_all_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::blue_dragon()];
    contract.mint_many(
        &contract.ali.clone(),
        None,
        &token_metas,
        &contract.admin.clone(),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(&contract.ali);

    contract.transfer_all_tokens(&contract.bob.clone(), &contract.ali.clone());
    assert_eq!(contract.balance_of(&contract.ali), U256::from(0));
    assert_eq!(contract.balance_of(&contract.bob), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(2));

    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(contract.bob));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(contract.bob));
}

#[test]
fn test_token_metadata_update() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();
    contract.mint_one(
        &contract.ali.clone(),
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );

    contract.update_token_metadata(&token_id, &meta::blue_dragon(), &contract.admin.clone());
    assert_eq!(contract.token_meta(&token_id).unwrap(), meta::blue_dragon());
}
