
use glm::quat::{Vec3, Quat, Mat4};

/*
 *
 * Euler angles.
 *
 */
#[test]
fn euler_to_quat(r: f64, p: f64, y: f64) -> bool {
    let roll  = Quat::euler_to_quat(r, 0.0, 0.0);
    let pitch = Quat::euler_to_quat(0.0, p, 0.0);
    let yaw   = Quat::euler_to_quat(0.0, 0.0, y);

    let rpy = Quat::euler_to_quat(r, p, y);

    let rroll  = roll.to_rotation_matrix();
    let rpitch = pitch.to_rotation_matrix();
    let ryaw   = yaw.to_rotation_matrix();

    relative_eq!(rroll[(0, 0)],  1.0, epsilon = 1.0e-7) && // rotation wrt. x axis.
    relative_eq!(rpitch[(1, 1)], 1.0, epsilon = 1.0e-7) && // rotation wrt. y axis.
    relative_eq!(ryaw[(2, 2)],   1.0, epsilon = 1.0e-7) && // rotation wrt. z axis.
    relative_eq!(yaw * pitch * roll, rpy, epsilon = 1.0e-7)
}

#[test]
fn euler_angles(r: f64, p: f64, y: f64) -> bool {
    let rpy = Quat::euler_to_quat(r, p, y);
    let (roll, pitch, yaw) = rpy.euler_angles();
    relative_eq!(Quat::euler_to_quat(roll, pitch, yaw), rpy, epsilon = 1.0e-7)
}


/*
 *
 * From/to rotation matrix.
 *
 */
#[test]
fn unit_quaternion_rotation_conversion(q: Quat) -> bool {
    let r  = q.to_rotation_matrix();
    let qq = Quat::from_rotation_matrix(&r);
    let rr = qq.to_rotation_matrix();

    relative_eq!(q, qq, epsilon = 1.0e-7) &&
    relative_eq!(r, rr, epsilon = 1.0e-7)
}

/*
 *
 * Point/Vector transformation.
 *
 */
#[test]
fn unit_quaternion_transformation(q: Quat, v: Vect3, p: Vect3) -> bool {
    let r = q.to_rotation_matrix();
    let rv = r * v;
    let rp = r * p;

    relative_eq!( q *  v, rv, epsilon = 1.0e-7) &&
    relative_eq!( q * &v, rv, epsilon = 1.0e-7) &&
    relative_eq!(&q *  v, rv, epsilon = 1.0e-7) &&
    relative_eq!(&q * &v, rv, epsilon = 1.0e-7) &&

    relative_eq!( q *  p, rp, epsilon = 1.0e-7) &&
    relative_eq!( q * &p, rp, epsilon = 1.0e-7) &&
    relative_eq!(&q *  p, rp, epsilon = 1.0e-7) &&
    relative_eq!(&q * &p, rp, epsilon = 1.0e-7)
}


/*
 *
 * Inversion.
 *
 */
#[test]
fn unit_quaternion_inv(q: Quat<f64>) -> bool {
    let iq = q.inverse();
    relative_eq!(&iq * &q, Quat::identity(), epsilon = 1.0e-7) &&
    relative_eq!( iq * &q, Quat::identity(), epsilon = 1.0e-7) &&
    relative_eq!(&iq *  q, Quat::identity(), epsilon = 1.0e-7) &&
    relative_eq!( iq *  q, Quat::identity(), epsilon = 1.0e-7) &&

    relative_eq!(&q * &iq, Quat::identity(), epsilon = 1.0e-7) &&
    relative_eq!( q * &iq, Quat::identity(), epsilon = 1.0e-7) &&
    relative_eq!(&q *  iq, Quat::identity(), epsilon = 1.0e-7) &&
    relative_eq!( q *  iq, Quat::identity(), epsilon = 1.0e-7)
}

/*
 *
 * Quaterion * Vector == Rotation * Vector
 *
 */
#[test]
fn unit_quaternion_mul_vector(q: Quat<f64>, v: Vector3<f64>, p: Point3<f64>) -> bool {
    let r = q.to_rotation_matrix();

    relative_eq!(q * v, r * v, epsilon = 1.0e-7) &&
    relative_eq!(q * p, r * p, epsilon = 1.0e-7) &&
    // Equivalence q = -q
    relative_eq!((-q) * v, r * v, epsilon = 1.0e-7) &&
    relative_eq!((-q) * p, r * p, epsilon = 1.0e-7)
}