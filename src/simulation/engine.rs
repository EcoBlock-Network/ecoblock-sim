use std::collections::HashSet;
use ecoblock_storage::tangle::block::TangleBlock;

use crate::simulation::scenario::SimulationScenario;

pub struct SimulationEngine;

impl SimulationEngine {
    pub fn run(scenario: &mut SimulationScenario, block: TangleBlock, entry_node: &str) {
        println!("🎬 Running simulation: {}", scenario.name);
        let mut visited = HashSet::new();
        let entry = scenario.nodes.get(entry_node).unwrap();
        entry.lock().unwrap().receive_block(block, &mut visited);
    }
}
