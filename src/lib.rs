pub use vector::*;

mod vector;

#[cfg(test)]
mod tests {
    use super::*;

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
