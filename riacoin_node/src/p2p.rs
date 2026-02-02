use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, Event as MdnsEvent},
    swarm::NetworkBehaviour,
    PeerId,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::chain::Block;
use crate::chain::Transaction;

pub static KEYS: Lazy<identity::Keypair> = Lazy::new(identity::Keypair::generate_ed25519);
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
pub static TOPIC_BLOCKS: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));
pub static TOPIC_TXS: Lazy<Topic> = Lazy::new(|| Topic::new("transactions"));

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    Block(Block),
    Transaction(Transaction),
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "AppEvent")]
pub struct AppBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
}

#[derive(Debug)]
pub enum AppEvent {
    Floodsub(FloodsubEvent),
    Mdns(MdnsEvent),
}

impl From<FloodsubEvent> for AppEvent {
    fn from(v: FloodsubEvent) -> Self {
        Self::Floodsub(v)
    }
}

impl From<MdnsEvent> for AppEvent {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

impl AppBehaviour {
    pub async fn new() -> Self {
        let floodsub = Floodsub::new(*PEER_ID);
        let mdns = Mdns::new(Default::default()).await.expect("can create mdns");
        
        let mut behaviour = Self {
            floodsub,
            mdns,
        };
        behaviour.floodsub.subscribe(TOPIC_BLOCKS.clone());
        behaviour.floodsub.subscribe(TOPIC_TXS.clone());
        behaviour
    }
}
