// use std::path::PathBuf;
use bs58::{encode, decode};
use solana_client::nonblocking::rpc_client::{self, RpcClient};
use solana_sdk::{signature::{read_keypair_file, Keypair}, signer::Signer, system_instruction::{create_account, transfer}, transaction::{Transaction, VersionedTransaction}};
// use solana_lite_rpc_services::{
//     transaction_service::{TransactionService, TransactionServiceBuilder},
//     tx_sender::TxSender,
//     tpu_utils::tpu_service::{TpuService, TpuServiceConfig},
// };

// use solana_lite_rpc_cluster_endpoints::json_rpc_subscription::create_json_rpc_polling_subscription;


#[tokio::main]
async fn main() {
    let rpc_client = RpcClient::new("http://127.0.0.1:8890".to_string());
    let slot = rpc_client.get_slot().await.unwrap();

    let blockhash = rpc_client.get_latest_blockhash().await.unwrap();
    let keypair = read_keypair_file("/Users/harshpatel/.config/solana/id.json").unwrap_or_else(|err| {
        println!("------------------------------------------------------------------------------------------------");
        println!("Failed to read `swap_example.json`: {err}");
        println!();
        println!("An ephemeral keypair will be used instead. For a more realistic example, create a new keypair at");
        println!("that location and fund it with a small amount of SOL.");
        println!("------------------------------------------------------------------------------------------------");
        println!();
        Keypair::new()
    });
    let user2 = Keypair::new();
    // let _ = rpc_client.request_airdrop(&user2.pubkey(), 1000000000).await.unwrap();
    let backrun_tx = VersionedTransaction::from(Transaction::new_signed_with_payer(
        &[
            create_account(&keypair.pubkey(), &user2.pubkey(), 2000000, 0, &solana_sdk::system_program::id()),
            transfer(&keypair.pubkey(), &user2.pubkey(), 10_000),
        ],
        Some(&keypair.pubkey()),
        &[&keypair, &user2],
        blockhash,
    ));

    let x = rpc_client.send_and_confirm_transaction(&backrun_tx).await.unwrap();
    print!("Current slot: {}", x);


}
    

//     let (subscriptions, cluster_endpoint_tasks) =  create_json_rpc_polling_subscription(rpc_client.clone())?;
//     let EndpointStreaming {
//         blocks_notifier,
//         cluster_info_notifier,
//         slot_notifier,
//         vote_account_notifier,
//     } = subscriptions;
//     let finalized_block =
//         get_latest_block(blocks_notifier.resubscribe(), CommitmentConfig::finalized()).await;

//     let block_information_store =
//         BlockInformationStore::new(BlockInformation::from_block(&finalized_block));

//     let data_cache = DataCache {
//         block_information_store,
//         cluster_info: ClusterInfo::default(),
//         identity_stakes: IdentityStakes::new(validator_identity.pubkey()),
//         slot_cache: SlotCache::new(finalized_block.slot),
//         tx_subs: SubscriptionStore::default(),
//         txs: TxStore::default(),
//     };

//     let leader_schedule = Arc::new(JsonRpcLeaderGetter::new(rpc_client.clone(), 1024, 128));

//     let tpu_service = TpuService::new(config, identity, leader_schedule, data_cache);
// }


// async fn start_tx_service(
//     lite_rpc_ws_addr: String,
//     lite_rpc_http_addr: String,
//     fanout_size: usize,
//     identity_kp: &Keypair,
// ) -> Result<()> {



//     Ok(())
// }
