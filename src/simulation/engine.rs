use std::collections::{HashSet, VecDeque};
use ecoblock_storage::tangle::block::TangleBlock;
use std::sync::{Arc, Mutex};

use crate::simulation::scenario::SimulationScenario;

pub struct SimulationEngine;

impl SimulationEngine {
    pub fn run(scenario: &mut SimulationScenario, block: TangleBlock, entry_node: &str) {
        // Use a breadth-first approach to avoid mutex deadlocks
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with the entry node
        if let Some(entry) = scenario.nodes.get(entry_node) {
            queue.push_back((Arc::clone(entry), block.clone()));
        }
        
        // Process nodes in breadth-first order
        while let Some((node, block)) = queue.pop_front() {
            let node_id = {
                let node_lock = node.lock().unwrap();
                node_lock.id.clone()
            };
            
            // Skip if already visited
            if visited.contains(&node_id) {
                continue;
            }
            
            // Process the node
            {
                let mut node_lock = node.lock().unwrap();
                if !node_lock.engine.has_received(&block.id) {
                    node_lock.engine.propagate_block(&block);
                    visited.insert(node_id.clone());
                    println!("   NODE {}: Received block", node_id);
                    
                    // Add peers to queue
                    let peers = node_lock.peers.clone();
                    for peer in peers {
                        let peer_id = {
                            let peer_lock = peer.lock().unwrap();
                            peer_lock.id.clone()
                        };
                        
                        if !visited.contains(&peer_id) {
                            queue.push_back((Arc::clone(&peer), block.clone()));
                        }
                    }
                } else {
                    println!("   NODE {}: Already has block", node_id);
                }
            }
        }
        
        // Add summary information
        println!("   Nodes reached: {}/{}", visited.len(), scenario.nodes.len());
    }
}
