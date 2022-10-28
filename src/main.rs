use std::str::FromStr;
use sui_sdk::{
    crypto::{AccountKeystore, FileBasedKeystore, Keystore},
    types::{
        base_types::{ObjectID, SuiAddress},
        messages::Transaction,
    },
    SuiClient,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // TODO make transaction builder for each of the dapps. Make main runner starting threads
    // in order to send the transactins over the network every X hours.
    let sui = SuiClient::new_rpc_client("https://fullnode.devnet.sui.io:443", None).await?;
    // Load keystore from ~/.sui/sui_config/sui.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let my_address = SuiAddress::from_str("0x008b758de70bf7eb57dc7b70d6a3d8d26e68c74b")?;
    let gas_object_id = ObjectID::from_str("0xe4f513ebdfc7069fa2a3305b3ee739adacf8469d")?;
    let recipient = SuiAddress::from_str("0x008b758de70bf7eb57dc7b70d6a3d8d26e68c74b")?;

    // Create a sui transfer transaction
    let transfer_tx = sui
        .transaction_builder()
        .transfer_sui(my_address, gas_object_id, 1000, recipient, Some(1))
        .await?;

    // Sign transaction
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let signature = keystore.sign(&my_address, &transfer_tx.to_bytes())?;

    // Execute the transaction
    let transaction_response = sui
        .quorum_driver()
        .execute_transaction(Transaction::new(transfer_tx, signature), None)
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}
