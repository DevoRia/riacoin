mod chain;
mod wallet;
mod p2p;
mod vm;

use crate::chain::{Blockchain, Transaction};
use crate::vm::SmartContractCall;
use crate::wallet::Wallet;
use crate::p2p::{AppBehaviour, AppEvent, NetworkMessage, KEYS, PEER_ID, TOPIC_TXS};
use colored::*;
use libp2p::{Swarm, swarm::SwarmEvent};
use std::io::{self, Write};
use tokio::io::AsyncBufReadExt;
use libp2p::floodsub::FloodsubEvent;

#[tokio::main]
async fn main() {
    print!("\x1B[2J\x1B[1;1H"); // Clear
    println!("{}", "=== RIACOIN P2P NODE ===".bright_green().bold());
    println!("Node ID: {}", PEER_ID.to_string().cyan());

    // 1. Setup Swarm
    let transport = libp2p::development_transport(KEYS.clone()).await.unwrap();
    let behaviour = AppBehaviour::new().await;
    let mut swarm = Swarm::with_tokio_executor(transport, behaviour, *PEER_ID);
    
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // 2. Local Chain State
    let mut riacoin = Blockchain::new();
    let my_wallet = Wallet::new();
    println!("My Address: {}", my_wallet.get_address().yellow());

    // 3. User Input
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    println!("{}", "[*] Listening for P2P events and User Input...".magenta());
    println!("Type 't' for transfer, 's' for smart contract, 'b' for balance.");

    loop {
        tokio::select! {
            // A. Network Event
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("[NET] Listening on {}", address);
                    }
                    SwarmEvent::Behaviour(AppEvent::Mdns(event)) => {
                         println!("[NET] mDNS Discovery: {:?}", event);
                         match event {
                             libp2p::mdns::Event::Discovered(list) => {
                                 for (peer, _) in list {
                                     swarm.behaviour_mut().floodsub.add_node_to_partial_view(peer);
                                     println!("[NET] Added peer to Floodsub: {}", peer);
                                 }
                             }
                             libp2p::mdns::Event::Expired(list) => {
                                 for (peer, _) in list {
                                     swarm.behaviour_mut().floodsub.remove_node_from_partial_view(&peer);
                                 }
                             }
                         }
                    }
                    SwarmEvent::Behaviour(AppEvent::Floodsub(FloodsubEvent::Message(msg))) => {
                         if let Ok(net_msg) = serde_json::from_slice::<NetworkMessage>(&msg.data) {
                             match net_msg {
                                NetworkMessage::Transaction(tx) => {
                                    println!("\n[NET] Received Tx: {} -> {} ({} RIA)", 
                                        &tx.sender[..8], &tx.recipient[..8], tx.amount);
                                    riacoin.add_transaction(tx);
                                },
                                NetworkMessage::Block(block) => {
                                    println!("\n[NET] Received Block #{}", block.index);
                                    riacoin.add_block(block);
                                }
                             }
                         }
                    }
                    _ => {}
                }
            },

            // B. User Input
            Ok(Some(line)) = stdin.next_line() => {
                match line.trim() {
                    "t" => {
                        println!("Broadcasting Simulated Tx...");
                        // Create Tx
                        let tx = Transaction::new(&my_wallet, my_wallet.get_address(), 10.0, 1.0, None);
                        
                        // Add Locally
                        riacoin.add_transaction(tx.clone());
                        
                        // Broadcast
                        let json = serde_json::to_vec(&NetworkMessage::Transaction(tx)).unwrap();
                        swarm.behaviour_mut().floodsub.publish(TOPIC_TXS.clone(), json);
                    },
                    "s" => {
                        println!("Broadcasting Smart Contract Call (Mint NFT)...");
                        let smart_call = SmartContractCall {
                            contract: "nft_registry".to_string(),
                            function: "mint".to_string(),
                            args: vec!["Monkey_#88".to_string(), my_wallet.get_address()],
                        };
                        
                        let tx = Transaction::new(&my_wallet, my_wallet.get_address(), 0.0, 0.1, Some(smart_call));
                        
                        riacoin.add_transaction(tx.clone());
                        let json = serde_json::to_vec(&NetworkMessage::Transaction(tx)).unwrap();
                        swarm.behaviour_mut().floodsub.publish(TOPIC_TXS.clone(), json);
                        println!("{}", "Smart Contract Tx Sent!".cyan());
                    },
                    "b" => {
                         println!("Current Balance: {} RIA", riacoin.get_balance(&my_wallet.get_address()));
                         println!("Pending Txs: {}", riacoin.pending_transactions.len());
                    },
                    "q" => break,
                     _ => {}
                }
                print!("> ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
