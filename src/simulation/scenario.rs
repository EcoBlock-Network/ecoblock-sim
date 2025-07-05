use ecoblock_gossip::node::GossipNode;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SimulationScenario {
    pub name: String,
    pub nodes: HashMap<String, Arc<Mutex<GossipNode>>>,
    pub connections: Vec<(String, String)>,
}

impl SimulationScenario {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            nodes: HashMap::new(),
            connections: vec![],
        }
    }

    pub fn add_node(&mut self, id: &str) {
        self.nodes.insert(id.to_string(), Arc::new(Mutex::new(GossipNode::new(id))));
    }

    pub fn connect(&mut self, from: &str, to: &str) {
        let from_node = self.nodes.get(from).unwrap();
        let to_node = self.nodes.get(to).unwrap();
        from_node.lock().unwrap().add_peer(Arc::clone(to_node));
        self.connections.push((from.to_string(), to.to_string()));
    }

    pub fn all_nodes(&self) -> Vec<Arc<Mutex<GossipNode>>> {
        self.nodes.values().cloned().collect()
    }
}

pub struct Scenario {
    pub name: String,
    pub build: fn() -> SimulationScenario,
}

pub struct ScenarioRunner {
    scenarios: Vec<Scenario>,
}

impl ScenarioRunner {
    pub fn new() -> Self {
        Self { scenarios: vec![] }
    }

    pub fn add_scenario(mut self, name: &str, build_fn: fn() -> SimulationScenario) -> Self {
        self.scenarios.push(Scenario {
            name: name.to_string(),
            build: build_fn,
        });
        self
    }

    pub fn run_all(&self) {
        println!("🎯 Running {} scenarios", self.scenarios.len());
        for scenario in &self.scenarios {
            println!("\n🔁 Scenario: {}", scenario.name);
            let sim = (scenario.build)();
            Self::run_simulation(&sim);
        }
    }

    fn run_simulation(sim: &SimulationScenario) {
        use ecoblock_crypto::keys::keypair::CryptoKeypair;
        use ecoblock_core::{SensorData, TangleBlockData};
        use ecoblock_storage::tangle::block::TangleBlock;

        let keypair = CryptoKeypair::generate();
        let data = TangleBlockData {
            parents: vec![],
            data: SensorData {
                pm25: 10.0,
                co2: 400.0,
                temperature: 22.0,
                humidity: 50.0,
                timestamp: 0,
            },
        };

        let block = TangleBlock::new(data, &keypair);
        let first_node = sim.nodes.values().next().unwrap();

        let mut visited = std::collections::HashSet::new();
        first_node.lock().unwrap().receive_block(block, &mut visited);

        println!("✅ Propagation done: {} nodes reached", visited.len());
    }
}
