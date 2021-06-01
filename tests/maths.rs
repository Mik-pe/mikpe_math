use mikpe_math::{Mat4, Vec3, Vec4};

#[test]
fn test_cross() {
    use crate::Vec3;
    let x_axis = Vec3::new(1.0, 0.0, 0.0);
    let y_axis = Vec3::new(0.0, 1.0, 0.0);
    let cross_product = x_axis.cross(y_axis);
    assert_eq!(cross_product[0], 0.0);
    assert_eq!(cross_product[1], 0.0);
    assert_eq!(cross_product[2], 1.0);
}

#[test]
fn test_vec4_into() {
    let v = Vec4::from_xyz(1.0, 2.0, 3.0);
    {
        let v: [f32; 4] = v.into();
        assert_eq!(v, [1.0, 2.0, 3.0, 1.0]);
    }
}

#[test]
fn test_mat4_into() {
    let mat = Mat4::new();
    {
        let mat: [[f32; 4]; 4] = mat.into();
        assert_eq!(
            mat,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );
    }
}

// #[test]
// fn test_inverse() {
//     let orig_mat4 = Mat4([
//         Vec4([22.2, 1.1, 4.4, 11.0]),
//         Vec4([2.2, 33.1, 44.4, 11.330]),
//         Vec4([2.52, 11.1, 4.255, 151.0]),
//         Vec4([233.2, 1.1321, 5.4, 11.0]),
//     ]);
//     let inv_mat4 = orig_mat4.inverse();
//     let multiplied = orig_mat4.mul(&inv_mat4);

//     let ident = Mat4::identity();
//     //These are not equal since of approximations and whatnot, whatever...
//     assert_eq!(multiplied, ident);
// }
