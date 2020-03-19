use engine::mesh_manager::MeshManager;
use engine::mesh_manager::mesh::*;
use glm::{Mat4, Vec4};


#[test]
fn test_storage() {
    let mut manager = MeshManager::new();

    manager.load("debug_box");

    println!("{:?}", manager.get_storage());

    manager.load("debug_box");

    println!("{:?}", manager.get_storage());

    manager.load("debug_d20");

    println!("{:?}", manager.get_storage());
    println!("{:?}", manager.get(&"debug_box"));
    println!("{:?}", manager.get(&"debug_d20"));

}