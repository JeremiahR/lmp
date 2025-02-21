use lightning::bitcoin::Network;
use lightning::routing::gossip::NetworkGraph;
use lightning::routing::gossip::P2PGossipSync;
use lightning::routing::router::DefaultRouter;
use logger::YourLogger;
use rand::RngCore;
use utxo_lookup::UtxoLookupImpl;

mod fee_estimator;
mod logger;
mod utxo_lookup;

fn main() {
    let mut ephemeral_bytes = [0; 32];
    rand::rng().fill_bytes(&mut ephemeral_bytes);

    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let fee_estimator = fee_estimator::YourFeeEstimator::new();
    let logger = logger::YourLogger::new();
    let network_graph = NetworkGraph::new(Network::Bitcoin, &logger);

    let gossip_sync: P2PGossipSync<&NetworkGraph<&YourLogger>, &UtxoLookupImpl, &YourLogger> =
        P2PGossipSync::new(&network_graph, None, &logger);

    // let router = DefaultRouter::new(
    //   network_graph.clone(),
    //   logger.clone(),
    //   keys_manager.get_secure_random_bytes(),
    //   scorer.clone(),
    //   ProbabilisticScoringFeeParameters::default()
    // )

    // let channel_manager = ChannelManager::new(
    //     fee_estimator,
    //     chain_monitor,
    //     tx_broadcaster,
    //     router,
    //     message_router,
    //     logger,
    //     entropy_source,
    //     node_signer,
    //     signer_provider,
    //     default_config,
    //     params,
    //     current_timestamp,
    // );

    // let lightning_msg_handler = MessageHandler {
    //     chan_handler: channel_manager,
    //     route_handler: gossip_sync,
    //     onion_message_handler: onion_messenger,
    //     custom_message_handler: IgnoringMessageHandler {},
    // };

    // let peer_manager = PeerManager::new(
    //     lightning_msg_handler,
    //     cur_time.as_secs().try_into().map_err(|e| {
    //         log_error!(logger, "Failed to get current time: {}", e);
    //         BuildError::InvalidSystemTime
    //     })?,
    //     &ephemeral_bytes,
    //     &logger,
    //     &keys_manager,
    // );
    println!("Hello, world!");
}
