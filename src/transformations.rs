use crate::matrix::M;

pub fn translation(x: f64, y: f64, z: f64) -> M {
    M::new(vec![
        vec![1.0, 0.0, 0.0, x],
        vec![0.0, 1.0, 0.0, y],
        vec![0.0, 0.0, 1.0, z],
        vec![0.0, 0.0, 0.0, 1.0],
    ])
    .unwrap()
}

pub fn scaling(x: f64, y: f64, z: f64) -> M {
    M::new(vec![
        vec![x, 0.0, 0.0, 0.0],
        vec![0.0, y, 0.0, 0.0],
        vec![0.0, 0.0, z, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ])
    .unwrap()
}

pub fn rotation_x(r: f64) -> M {
    M::new(vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, r.cos(), -r.sin(), 0.0],
        vec![0.0, r.sin(), r.cos(), 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ])
    .unwrap()
}

pub fn rotation_y(r: f64) -> M {
    M::new(vec![
        vec![r.cos(), 0.0, r.sin(), 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![-r.sin(), 0.0, r.cos(), 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ])
    .unwrap()
}

pub fn rotation_z(r: f64) -> M {
    M::new(vec![
        vec![r.cos(), -r.sin(), 0.0, 0.0],
        vec![r.sin(), r.cos(), 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ])
    .unwrap()
}

pub fn shearing(
    x_y: f64,
    x_z: f64,
    y_x: f64,
    y_z: f64,
    z_x: f64,
    z_y: f64,
) -> M {
    M::new(vec![
        vec![1.0, x_y, x_z, 0.0],
        vec![y_x, 1.0, y_z, 0.0],
        vec![z_x, z_y, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ])
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::transformations::{
        rotation_x, rotation_y, rotation_z, scaling,
        shearing, translation,
    };
    use std::f64::consts::PI;

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

        assert_eq!(scaling(2.0, 3.0, 4.0) * v_a, v_b)
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

        assert_eq!(scaling(-1.0, 1.0, 1.0) * v_a, v_b)
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

    #[test]
    fn test_rotation_y() {
        let p = Tuple::point(0.0, 0.0, 1.0);

        let half_q = rotation_y(PI / 4.0);
        let full_q = rotation_y(PI / 2.0);

        let sq2 = (2.0 as f64).sqrt();

        assert_eq!(
            half_q * p,
            Tuple::point(sq2 / 2.0, 0.0, sq2 / 2.0)
        );

        assert_eq!(
            full_q * p,
            Tuple::point(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_rotation_z() {
        let p = Tuple::point(0.0, 1.0, 0.0);

        let half_q = rotation_z(PI / 4.0);
        let full_q = rotation_z(PI / 2.0);

        let sq2 = (2.0 as f64).sqrt();

        assert_eq!(
            half_q * p,
            Tuple::point(-sq2 / 2.0, sq2 / 2.0, 0.0)
        );

        assert_eq!(
            full_q * p,
            Tuple::point(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_shearing_x_y() {
        let v_a = Tuple::vector(2.0, 3.0, 4.0);
        let v_b = Tuple::vector(5.0, 3.0, 4.0);

        assert_eq!(
            shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * v_a,
            v_b
        )
    }

    #[test]
    fn test_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let m_a = rotation_x(PI / 2.0);
        let m_b = scaling(5., 5., 5.);
        let m_c = translation(10., 5., 7.);

        let p2 = m_a.clone() * p;
        assert_eq!(p2, Tuple::point(1., -1., 0.));

        let p3 = m_b.clone() * p2;
        assert_eq!(p3, Tuple::point(5., -5., 0.));

        let p4 = m_c.clone() * p3;
        assert_eq!(p4, Tuple::point(15., 0., 7.));

        // should be same as:
        assert_eq!(m_c * m_b * m_a * p, p4);
    }
}
