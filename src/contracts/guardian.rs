use std::sync::Arc;

use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::Address,
    utils::keccak256,
};
use eyre::Result;

abigen!(
    IEmailGuardian,
    r#"[
        function getHash(address, bytes32) external view returns (bytes32)
    ]"#,
);

pub async fn contract(
    provider: Provider<Http>,
    guardian_address: &str,
    account: &str,
    email: &str,
) -> Result<()> {
    let client = Arc::new(provider);
    let address: Address = guardian_address
        .parse()
        .expect("parse guardian address error");
    let account: Address = account.parse().expect("parse account address error");
    let guardian = IEmailGuardian::new(address, client);

    let _ = guardian
        .get_hash(account, keccak256::<&str>(&email))
        .call()
        .await;

    Ok(())
}
