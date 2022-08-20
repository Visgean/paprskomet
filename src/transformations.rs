use crate::matrix::M;

pub fn translation(x: f64, y: f64, z: f64) -> M {
    M::new(vec![
        vec![1.0, 0.0, 0.0, x],
        vec![0.0, 1.0, 0.0, y],
        vec![0.0, 0.0, 1.0, z],
        vec![0.0, 0.0, 0.0, 1.0],
    ]).unwrap()
}

pub fn scaling(x: f64, y: f64, z: f64) -> M {
    M::new(vec![
        vec![x, 0.0, 0.0, 0.0],
        vec![0.0, y, 0.0, 0.0],
        vec![0.0, 0.0, z, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]).unwrap()
}

pub fn rotation_x(r: f64) -> M {
    M::new(vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, r.cos(), -r.sin(), 0.0],
        vec![0.0, r.sin(), r.cos(), 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]).unwrap()
}




#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::transformations::{scaling, translation, rotation_x};

    use crate::vectors::Tuple;

    #[test]
    fn test_translation_point() {
        let point_a = Tuple::point(-3.0, 4.0, 5.0);
        let point_b = Tuple::point(2.0, 1.0, 7.0);


        assert_eq!(
            translation(5.0, -3.0, 2.0) * point_a,
            point_b
        )
    }

    #[test]
    fn test_translation_inverse() {
        let point_a = Tuple::point(-3.0, 4.0, 5.0);
        let point_b = Tuple::point(2.0, 1.0, 7.0);

        let t = translation(5.0, -3.0, 2.0);

        let res = t.clone() * point_a;
        assert_eq!(res, point_b);

        assert_eq!(t.inverse() * res, point_a)
    }


    #[test]
    fn test_translation() {
        let point_a = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(
            translation(5.0, -3.0, 2.0) * point_a,
            point_a
        )
    }


    #[test]
    fn test_scaling_point() {
        let point_a = Tuple::point(-4.0, 6.0, 8.0);
        let point_b = Tuple::point(-8.0, 18.0, 32.0);

        assert_eq!(
            scaling(2.0, 3.0, 4.0) * point_a,
            point_b
        )
    }

    #[test]
    fn test_scaling_vector() {
        let v_a = Tuple::vector(-4.0, 6.0, 8.0);
        let v_b = Tuple::vector(-8.0, 18.0, 32.0);

        assert_eq!(
            scaling(2.0, 3.0, 4.0) * v_a,
            v_b
        )
    }

    #[test]
    fn test_scaling_vector_inverse() {
        let v_a = Tuple::vector(-4.0, 6.0, 8.0);
        let v_b = Tuple::vector(-2.0, 2.0, 2.0);

        assert_eq!(
            scaling(2.0, 3.0, 4.0).inverse() * v_a,
            v_b
        )
    }

    #[test]
    fn test_reflection() {
        let v_a = Tuple::vector(2.0, 3.0, 4.0);
        let v_b = Tuple::vector(-2.0, 3.0, 4.0);

        assert_eq!(
            scaling(-1.0, 1.0, 1.0) * v_a,
            v_b
        )
    }


    #[test]
    fn test_rotation_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);

        let half_q = rotation_x(PI / 4.0);
        let full_q = rotation_x(PI / 2.0);

        let sq2 = (2.0 as f64).sqrt();

        assert_eq!(
            half_q * p,
            Tuple::point(0.0, sq2 / 2.0, sq2 / 2.0)
        );

        assert_eq!(
            full_q * p,
            Tuple::point(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn test_rotation_x_inv() {
        let p = Tuple::point(0.0, 1.0, 0.0);

        let half_q = rotation_x(PI / 4.0);
        let sq2 = (2.0 as f64).sqrt();

        assert_eq!(
            half_q.inverse() * p,
            Tuple::point(0.0, sq2 / 2.0, -sq2 / 2.0)
        );
    }
}