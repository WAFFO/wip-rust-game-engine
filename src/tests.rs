use engine::mesh_manager::MeshManager;
use engine::mesh_manager::mesh::*;
use math::{Mat4, Vert4};


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

//#[test]
//fn test_mesh_exploder() {
//    let mesh_indexed = MeshIndexed {
//        vertices: vec![
//            0.0, 1.0, 2.0,
//            3.0, 4.0, 5.0,
//            6.0, 7.0, 8.0,
//        ],
//        colors: vec![
//            0.5, 1.5, 2.5, 1.0,
//        ],
//        indices: vec![
//            2, 1, 0,
//        ],
//    };
//
//    let mesh = Mesh {
//        vertices: vec![
//            6.0, 7.0, 8.0,
//            3.0, 4.0, 5.0,
//            0.0, 1.0, 2.0,
//        ],
//        colors: vec![
//            0.5, 1.5, 2.5, 1.0,
//            0.5, 1.5, 2.5, 1.0,
//            0.5, 1.5, 2.5, 1.0,
//        ],
//    };
//
//    assert_eq!(mesh_indexed.explode(), mesh);
//
//    let mesh_indexed = MeshIndexed {
//        vertices: vec![
//            0.0, 1.0, 2.0,
//            3.0, 4.0, 5.0,
//            6.0, 7.0, 8.0,
//        ],
//        colors: vec![
//            0.1, 1.1, 2.1, 1.1,
//            0.2, 1.2, 2.2, 1.2,
//        ],
//        indices: vec![
//            2, 1, 0,
//            0, 1, 2,
//        ],
//    };
//
//    let mesh = Mesh {
//        vertices: vec![
//            6.0, 7.0, 8.0,
//            3.0, 4.0, 5.0,
//            0.0, 1.0, 2.0,
//            0.0, 1.0, 2.0,
//            3.0, 4.0, 5.0,
//            6.0, 7.0, 8.0,
//        ],
//        colors: vec![
//            0.1, 1.1, 2.1, 1.1,
//            0.1, 1.1, 2.1, 1.1,
//            0.1, 1.1, 2.1, 1.1,
//            0.2, 1.2, 2.2, 1.2,
//            0.2, 1.2, 2.2, 1.2,
//            0.2, 1.2, 2.2, 1.2,
//        ],
//    };
//
//    assert_eq!(mesh_indexed.explode(), mesh);
//}

#[test]
fn test_math(){
    let a = Mat4::mat4([
        1.0, 2.0, 3.0, 0.0,
        0.0, 2.0, 0.0, 0.0,
        0.0, 0.0, 3.0, 0.0,
        5.0, 0.0, 0.0, 1.0,
    ]);
    let b = Mat4::mat4([
        5.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        1.0, 0.0, 2.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);
    let c = Mat4::mat4([
        1.0, 0.0, 0.0, 2.0,
        0.0, 1.0, 2.0, 0.0,
        0.0, 2.0, 1.0, 0.0,
        2.0, 0.0, 0.0, 1.0,
    ]);
    let v = Vert4::new(2.0, 3.0, 1.0, 3.0);
    let r1 = Mat4::mat4([
        5.0, 10.0, 15.0, 0.0,
        0.0, 6.0, 0.0, 0.0,
        1.0, 2.0, 9.0, 0.0,
        5.0, 0.0, 0.0, 1.0,
    ]);
    let r2 = Mat4::mat4([
        15.0, 10.0, 15.0, 2.0,
        2.0, 10.0, 18.0, 0.0,
        1.0, 14.0, 9.0, 0.0,
        15.0, 20.0, 30.0, 1.0,
    ]);
    let r3 = Vert4::new(17.0, 10.0, 9.0, 3.0);

    assert_eq!(a*b,r1);
    assert_eq!(a*b*c,r2);
    assert_eq!(a*v, r3);
}