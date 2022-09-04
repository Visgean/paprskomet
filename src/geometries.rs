use crate::ray::Ray;
use uuid::Uuid;

struct Sphere {
    id: Uuid,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { id: Uuid::new_v4() }
    }

    pub fn intersects(&self, ray: Ray) -> Vec<f64> {
        return vec![1., 2.];
    }
}

#[cfg(test)]
mod tests {
    use crate::geometries::Sphere;
    use crate::ray::Ray;
    use crate::utils::float_compare;
    use crate::vectors::Tuple;

    #[test]
    fn test_intersection_1() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints[0], 4.0);
        assert_eq!(ints[0], 6.0);
    }
}
