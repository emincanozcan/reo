use std::{fs, process::exit};

#[derive(serde::Deserialize, Debug)]
pub struct Instance {
    pub id: usize,
    pub address: String,
    pub name: String,
}

pub struct NodeConfig {
    pub nodes: Vec<Instance>,
    pub current_node: Instance,
}

// TODO: get from TOML
impl NodeConfig {
    pub fn init(node_id: usize) -> Self {
        let filename = std::env::current_dir().unwrap().to_str().unwrap().to_string() + "/NodeConfig.json";

        let contents = match fs::read_to_string(filename.clone()) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", filename);
                exit(1);
            }
        };

        let nodes: Vec<Instance> = match serde_json::from_str(&contents) {
            Ok(d) => d,
            Err(err) => {
                eprintln!("Unable to parse data from `{}`: {}", filename, err);
                exit(1);
            }
        };

        let curr_node_info = &nodes.iter().find(|node| node.id == node_id).unwrap();
        let current_node = Instance {
            id: curr_node_info.id,
            address: curr_node_info.address.clone(),
            name: curr_node_info.name.clone(),
        };

        println!("REO Init: Current Node: {:?}", current_node);
        println!("REO Init: Nodes: {:?}", nodes);

        Self {
            nodes,
            current_node,
        }
    }
}
