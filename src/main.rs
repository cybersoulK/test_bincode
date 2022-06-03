use glam::vec3;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Instance {
    position: glam::Vec3,
}

fn main() {
    
    let instance = Instance { position: vec3(2.0, 2.0, 2.0) };

    let m = bincode::serde::encode_to_vec(&instance, bincode::config::standard()).unwrap();

    let instance2: Instance = bincode::serde::decode_from_slice(&m, bincode::config::standard()).unwrap().0;


    println!("{:?}, {:?}", instance, instance2);
}
