use glam::vec3;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Instance {
    position: glam::Vec3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum TestEnum {
    A(f32, f32, f32, f32),
    B(Vec<Instance>),
    C(Instance),
    D(bool),
}

fn main() {

    let instance = Instance { position: vec3(2.0, 2.0, 2.0) };
    let test_enum = TestEnum::B(vec![instance.clone(), instance.clone(), instance.clone(), instance.clone(), instance.clone(), instance.clone()]);

    println!("bincode 2.0");
    b2(test_enum.clone());

    println!("");

    println!("bincode 1.3.3");
    b133(test_enum.clone());
}


fn b2(test_enum: TestEnum) {

    let m = bincode::serde::encode_to_vec(&test_enum, bincode::config::standard()).unwrap();
    println!("{}", m.len());

    let test_enum2: TestEnum = bincode::serde::decode_from_slice(&m, bincode::config::standard()).unwrap().0;


    println!("{:?}", test_enum2);
}

fn b133(test_enum: TestEnum) {

    let m = bincode133::serialize(&test_enum).unwrap();
    println!("{}", m.len());

    let test_enum2: TestEnum = bincode133::deserialize(&m).unwrap();


    println!("{:?}", test_enum2);
}