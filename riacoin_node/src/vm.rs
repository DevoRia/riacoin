use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// A "Smart Contract" in RiaCoin is just a function identifier and params.
// In a real modern chain (like Solana), this would be BPF bytecode.
// Here we simulate the "World Computer" aspect.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartContractCall {
    pub contract: String, // e.g., "nft_registry"
    pub function: String, // e.g., "mint"
    pub args: Vec<String>, // e.g., ["Rare_Monkey_Image_IPFS_Link"]
}

pub struct VirtualMachine {
    pub state: HashMap<String, String>, // Contract Storage
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            state: HashMap::new(),
        }
    }

    pub fn execute(&mut self, call: &SmartContractCall) -> bool {
        match call.contract.as_str() {
            "nft_registry" => self.run_nft(call),
            "governance" => self.run_governance(call),
            _ => {
                println!("[VM] Unknown contract: {}", call.contract);
                false
            }
        }
    }

    fn run_nft(&mut self, call: &SmartContractCall) -> bool {
        if call.function == "mint" {
            if call.args.len() < 2 { return false; }
            let token_id = &call.args[0];
            let owner = &call.args[1];
            
            // Storage key: nft_registry:token_id -> owner
            let key = format!("nft:{}:owner", token_id);
            if self.state.contains_key(&key) {
                println!("[VM] Error: NFT {} already exists!", token_id);
                return false;
            }
            
            self.state.insert(key, owner.clone());
            println!("[VM] ✨ SUCCESS: Minted NFT #{} for {}", token_id, owner);
            return true;
        }
        false
    }
    
    fn run_governance(&mut self, call: &SmartContractCall) -> bool {
        // Vote logic...
        true
    }
}
