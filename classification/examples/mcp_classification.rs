extern crate classification;

fn main() {
    let mcp_neuron = classification::mcp::MCPNeuron::new(10, 10, 0.01);
    println!("{}", mcp_neuron.get_weight(2));
}