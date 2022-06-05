extern crate web3;

use std::time;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::*,
};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    // Get the contract bytecode and ABI from Solidity compiler
    let bytecode = include_str!("./res/KOKU.bin");
    let abi = include_bytes!("./res/KOKU.abi");

    // Deploying a KOKU contract
    let koku_contract = Contract::deploy(web3.eth(), abi).unwrap()
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(1))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, accounts[0], accounts[0])
        .await.unwrap();

    println!("Deployed at: {}", koku_contract.address());

    // Get the contract bytecode and ABI from Solidity compiler
    let bytecode = include_str!("./res/OKOKU.bin");
    let abi = include_bytes!("./res/OKOKU.abi");

    // Deploying an OKOKU contract
    let okoku_contract = Contract::deploy(web3.eth(), abi).unwrap()
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(1))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .await.unwrap();

    println!("Deployed at: {}", okoku_contract.address());

    // // If we already have a deployed contract
    // let token_address = Address::from_str("0xdEa937d22a19AFd39Aa88DdC72fF8859a102B0C3").unwrap();
    // let contract = Contract::from_json(web3.eth(), token_address, abi).unwrap();

    // // Check owner address after deployment
    // let owner: Address = koku_contract.query("owner", (), None, Options::default(), None).await.unwrap();
    // println!("Owner: {:?}", owner);

    // Transfer some tokens from one account to another
    println!("Balance before transfer: {:?}", balance(&koku_contract, accounts[1]).await);
    let tx = transfer(&koku_contract, accounts[1], accounts[0]).await;
    println!("Balance after transfer: {:?}", balance(&koku_contract, accounts[1]).await);
    println!("Transaction hash: {:?}", tx);




    Ok(())
}

async fn balance(contract: &Contract<Http>, address: Address) -> U256 {
    contract.query("balanceOf", address, None, Options::default(), None).await.unwrap()
}

async fn transfer(contract: &Contract<Http>, to: Address, from: Address) -> H256 {
    contract.call("transfer", (to, 42_u32), from, Options::default()).await.unwrap()
}