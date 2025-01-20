// src/tests.rs
use ic_cdk::{api, export};
use ic_cdk_macros::{heartbeat, init, post_upgrade, pre_upgrade, update};

// Define a test for sending tokens
#[test]
fn test_send_tokens() {
    let mut wallet = TokenWallet::new();
    wallet.send_tokens("recipient".to_string(), 10).unwrap();
    assert_eq!(wallet.get_balance(), 90);
}

// Define a test for receiving tokens
#[test]
fn test_receive_tokens() {
    let mut wallet = TokenWallet::new();
    wallet.receive_tokens(10);
    assert_eq!(wallet.get_balance(), 10);
}

// Define a test for getting the token balance
#[test]
fn test_get_balance() {
    let wallet = TokenWallet::new();
    assert_eq!(wallet.get_balance(), 0);
}
