pub use utilities::*;
pub use point::*;
pub use vector::*;
pub use matrix::*;
pub use bezier::*;

mod utilities;
mod point;
mod vector;
mod matrix;
mod bezier;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testDet4() {
        let m4 = Mat4::new(16.0, 12.0, 5.0, 2.0, 5.0, 26.0, 7.0, 8.0, 9.0, 114.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);

        assert_eq!(m4.det(), 3760.0);
    }

    #[test]
    fn testSwap() {
        let mut m = Mat2::new(1.0,2.0,3.0,4.0);
        m.swapRows(0,1);
        assert_eq!(m, Mat2::new(3.0,4.0,1.0,2.0));
    }

    #[test]
    fn testVAngle() {
        let v1 = Vec2::new(1.0, 0.0);
        let v2 = Vec2::new(0.0, 1.0);

        assert_eq!(vAngle(&v1,&v2), (std::f64::consts::PI as f32)/2.0);
    }

    #[test]
    fn testTranspose() {
        let matrix2 = Mat2::new(1.0,2.0,3.0,4.0);

        assert_eq!(matrix2.transpose(), Mat2::new(1.0,3.0,2.0,4.0));
    }

    #[test]
    fn test_lerp() {
        let a = 2.0;
        let b = 4.0;
        assert_eq!(lerp(a,b,0.5), 3.0);
    }

    #[test]
    fn test_inverseLerp() {
        let a = 2.0;
        let b = 4.0;
        assert_eq!(invLerp(a,b,3.0), 0.5);
    }

    #[test]
    fn test_clamp() {
        let start = 0.0;
        let end = 10.0;
        assert_eq!(clamp(start, end, -10002022020.0), 0.0);
    }

    #[test]
    fn test_vec2_dot_product() {
        let v1 = Vec2::new(1.0, 1.0);
        let v2 = Vec2::new(1.0, 1.0);
        assert_eq!(dot(&v1, &v2), 2.0);
    }

    #[test]
    fn test_vec3_dot_product() {
        let v3 = Vec3::new(1.0, 2.0, 3.0);
        let v4 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(dot(&v3, &v4), 32.0);
    }

    #[test]
    fn test_vec4_dot_product() {
        let v5 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v6 = Vec4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(dot(&v5, &v6), 70.0);
    }
}
