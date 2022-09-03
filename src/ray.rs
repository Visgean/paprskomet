use crate::vectors::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {

    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + t * self.direction
    }

}




#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::utils::float_compare;
    use crate::vectors::Tuple;

    #[test]
    fn test_position() {
        let r = Ray::new(
            Tuple::point(2., 3., 4.),
            Tuple::vector(1., 0., 0.),
        );

        assert_eq!(
            r.position(0.),
            Tuple::point(2., 3., 4.)
        );

        assert_eq!(
            r.position(1.),
            Tuple::point(3., 3., 4.)
        );


        assert_eq!(
            r.position(-1.),
            Tuple::point(1., 3., 4.)
        );

        assert_eq!(
            r.position(2.5),
            Tuple::point(4.5, 3., 4.)
        );



    }

}