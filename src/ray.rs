use crate::matrix::M;
use crate::vectors::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + t * self.direction
    }

    pub fn transform(&self, m: &M) -> Ray {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::transformations::{scaling, translation};
    use crate::utils::float_compare;
    use crate::vectors::Tuple;

    #[test]
    fn test_position() {
        let r = Ray::new(Tuple::point(2., 3., 4.), Tuple::vector(1., 0., 0.));

        assert_eq!(r.position(0.), Tuple::point(2., 3., 4.));

        assert_eq!(r.position(1.), Tuple::point(3., 3., 4.));

        assert_eq!(r.position(-1.), Tuple::point(1., 3., 4.));

        assert_eq!(r.position(2.5), Tuple::point(4.5, 3., 4.));
    }

    #[test]
    fn test_translation() {
        let r1 = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));

        let r1_result =
            Ray::new(Tuple::point(4., 6., 8.), Tuple::vector(0., 1., 0.));

        let r1_t = r1.transform(&translation(5., 5., 5.));
    }

    #[test]
    fn test_scaling() {
        let r1 = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));

        let r1_result =
            Ray::new(Tuple::point(2., 6., 12.), Tuple::vector(0., 3., 0.));

        let r1_t = r1.transform(&scaling(2., 3., 4.));
    }
}
