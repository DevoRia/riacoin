use crate::vm::{VirtualMachine, SmartContractCall};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::Utc;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub sender: String, // Public Key Hex
    pub recipient: String,
    pub amount: f64,
    pub fee: f64,
    pub timestamp: i64,
    pub signature: String,
    pub smart_call: Option<SmartContractCall>, // "New Technology" - Programmability
}

impl Transaction {
    pub fn new(wallet: &Wallet, recipient: String, amount: f64, fee: f64, smart_call: Option<SmartContractCall>) -> Self {
        let timestamp = Utc::now().timestamp();
        let sender = wallet.get_address();
        
        // Payload to sign (include smart_call if present)
        let call_str = match &smart_call {
            Some(c) => format!("{:?}", c),
            None => "".to_string(),
        };

        let payload = format!("{}{}{}{}{}{}", sender, recipient, amount, fee, timestamp, call_str);
        let signature = wallet.sign(payload.as_bytes());
        let id = hex::encode(Sha256::digest(payload.as_bytes()));

        Transaction {
            id,
            sender,
            recipient,
            amount,
            fee,
            timestamp,
            signature,
            smart_call,
        }
    }

    pub fn new_coinbase(recipient: String, amount: f64) -> Self {
        let timestamp = Utc::now().timestamp();
        let sender = "NETWORK_MINT".to_string();
        let payload = format!("{}{}{}{}{}", sender, recipient, amount, 0.0, timestamp);
        let id = hex::encode(Sha256::digest(payload.as_bytes()));
        
        Transaction {
            id,
            sender,
            recipient,
            amount,
            fee: 0.0,
            timestamp,
            signature: "COINBASE".to_string(),
            smart_call: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.sender == "NETWORK_MINT" {
            return true;
        }
        let call_str = match &self.smart_call {
            Some(c) => format!("{:?}", c),
            None => "".to_string(),
        };
        let payload = format!("{}{}{}{}{}{}", self.sender, self.recipient, self.amount, self.fee, self.timestamp, call_str);
        Wallet::verify(&self.sender, payload.as_bytes(), &self.signature)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub validator: String,
    pub merkle_root: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String, validator: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let merkle_root = Self::calculate_merkle_root(&transactions);
        let hash_input = format!("{}{}{}{}{}", index, timestamp, merkle_root, previous_hash, validator);
        let hash = hex::encode(Sha256::digest(hash_input.as_bytes()));

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            validator,
            merkle_root,
        }
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        let mut hashes: Vec<String> = transactions.iter().map(|tx| tx.id.clone()).collect();
        if hashes.is_empty() {
            return "0".to_string();
        }
        
        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for chunk in hashes.chunks(2) {
                let h1 = &chunk[0];
                let h2 = if chunk.len() > 1 { &chunk[1] } else { h1 };
                let combined = format!("{}{}", h1, h2);
                new_hashes.push(hex::encode(Sha256::digest(combined.as_bytes())));
            }
            hashes = new_hashes;
        }
        hashes[0].clone()
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub balances: HashMap<String, f64>,
    pub vm: VirtualMachine, // The BRAIN of the blockchain
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            balances: HashMap::new(),
            vm: VirtualMachine::new(),
        };
        // Genesis
        let genesis_tx = Transaction::new_coinbase("GENESIS_WALLET".to_string(), 1_000_000.0);
        let genesis_block = Block::new(0, vec![genesis_tx], "0".to_string(), "GENESIS".to_string());
        blockchain.add_block(genesis_block);
        blockchain
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if !tx.is_valid() {
            println!("Invalid transaction signature from {}", tx.sender);
            return false;
        }
        
        // Check balance
        if tx.sender != "NETWORK_MINT" {
            let sender_bal = *self.balances.get(&tx.sender).unwrap_or(&0.0);
            if sender_bal < (tx.amount + tx.fee) {
                println!("Insufficient funds for {}", tx.sender);
                return false;
            }
        }

        self.pending_transactions.push(tx);
        true
    }

    pub fn mine_block(&mut self, validator_address: String) -> Option<Block> {
        if self.pending_transactions.is_empty() {
            return None;
        }

        // Add fee reward to validator
        let fees: f64 = self.pending_transactions.iter().map(|tx| tx.fee).sum();
        let reward_tx = Transaction::new_coinbase(validator_address.clone(), 10.0 + fees);
        
        let mut txs = self.pending_transactions.clone();
        txs.push(reward_tx);

        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            txs,
            last_block.hash.clone(),
            validator_address
        );

        self.add_block(new_block.clone());
        self.pending_transactions.clear();
        Some(new_block)
    }

    pub fn add_block(&mut self, block: Block) {
        // Update balances
        for tx in &block.transactions {
            // Subtract from sender
            if tx.sender != "NETWORK_MINT" {
                let sender_bal = self.balances.entry(tx.sender.clone()).or_insert(0.0);
                *sender_bal -= tx.amount + tx.fee;
            }
            // Add to recipient
            let recipient_bal = self.balances.entry(tx.recipient.clone()).or_insert(0.0);
            *recipient_bal += tx.amount;

            // NEW: Execute Smart Contract Logic
            if let Some(call) = &tx.smart_call {
                self.vm.execute(call);
            }
        }
        self.chain.push(block);
    }
    
    pub fn get_balance(&self, address: &str) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }
}
