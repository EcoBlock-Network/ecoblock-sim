use ecoblock_core::{SensorData, TangleBlockData};
use ecoblock_crypto::keys::keypair::CryptoKeypair;
use ecoblock_sim::simulation::{engine::SimulationEngine, scenario::SimulationScenario};
use ecoblock_storage::tangle::block::TangleBlock;

/// 🔁 Crée une topologie linéaire A → B → C
fn build_linear_topology() -> SimulationScenario {
    let mut scenario = SimulationScenario::new("Linear topology");
    scenario.add_node("A");
    scenario.add_node("B");
    scenario.add_node("C");
    scenario.connect("A", "B");
    scenario.connect("B", "C");
    scenario
}

/// 🔁 Crée une topologie en anneau : A → B → C → A
fn build_ring_topology() -> SimulationScenario {
    let mut scenario = SimulationScenario::new("Ring topology");
    scenario.add_node("A");
    scenario.add_node("B");
    scenario.add_node("C");
    scenario.connect("A", "B");
    scenario.connect("B", "C");
    scenario.connect("C", "A");
    scenario
}

/// ⭐ Crée une topologie étoile : A au centre
fn build_star_topology() -> SimulationScenario {
    let mut scenario = SimulationScenario::new("Star topology");
    scenario.add_node("A");
    scenario.add_node("B");
    scenario.add_node("C");
    scenario.add_node("D");
    scenario.connect("A", "B");
    scenario.connect("A", "C");
    scenario.connect("A", "D");
    scenario
}

/// 🌐 Crée une topologie en mesh complet : tous connectés à tous
fn build_full_mesh_topology() -> SimulationScenario {
    let nodes = ["A", "B", "C", "D"];
    let mut scenario = SimulationScenario::new("Full mesh topology");

    for &node in &nodes {
        scenario.add_node(node);
    }

    for &from in &nodes {
        for &to in &nodes {
            if from != to {
                scenario.connect(from, to);
            }
        }
    }

    scenario
}

/// 🌲 Crée une topologie arborescente simple
fn build_tree_topology() -> SimulationScenario {
    let mut scenario = SimulationScenario::new("Tree topology");
    scenario.add_node("Root");
    scenario.add_node("Child1");
    scenario.add_node("Child2");
    scenario.add_node("Grandchild1");
    scenario.connect("Root", "Child1");
    scenario.connect("Root", "Child2");
    scenario.connect("Child1", "Grandchild1");
    scenario
}

fn main() {
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

    let mut scenarios = vec![
        build_linear_topology(),
        build_ring_topology(),
        build_star_topology(),
        build_full_mesh_topology(),
        build_tree_topology(),
    ];

    for scenario in scenarios.iter_mut() {
        println!("🎬 Running simulation: {}", scenario.name);
        SimulationEngine::run(scenario, block.clone(), "A");
        println!("✅ Simulation complete: {}", scenario.name);
        println!("---------------------------\\n");
    }
}