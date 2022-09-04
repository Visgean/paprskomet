use crate::ray::Ray;
use crate::vectors::Tuple;
use uuid::Uuid;

struct Sphere {
    id: Uuid,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { id: Uuid::new_v4() }
    }

    pub fn intersects(&self, ray: Ray) -> Vec<f64> {
        let sphere_to_ray =
            ray.origin - Tuple::point(0., 0., 0.);
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;
        if discriminant < 0. {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        return vec![t1, t2];
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
        let r = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.),
        );

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 2);

        assert_eq!(ints[0], 4.0);
        assert_eq!(ints[1], 6.0);
    }

    #[test]
    fn test_intersection_2() {
        let r = Ray::new(
            Tuple::point(0., 1., -5.),
            Tuple::vector(0., 0., 1.),
        );

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 2);

        assert_eq!(ints[0], 5.0);
        assert_eq!(ints[1], 5.0);
    }

    #[test]
    fn test_intersection_miss() {
        let r = Ray::new(
            Tuple::point(0., 2., -5.),
            Tuple::vector(0., 0., 1.),
        );

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 0);
    }

    #[test]
    fn test_intersection_inside() {
        let r = Ray::new(
            Tuple::point(0., 0., 0.),
            Tuple::vector(0., 0., 1.),
        );

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 2);

        assert_eq!(ints[0], -1.);
        assert_eq!(ints[1], 1.);
    }

    #[test]
    fn test_intersection_behind() {
        let r = Ray::new(
            Tuple::point(0., 0., 5.),
            Tuple::vector(0., 0., 1.),
        );

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 2);

        assert_eq!(ints[0], -6.);
        assert_eq!(ints[1], -4.);
    }

    #[test]
    fn test_point_calc() {
        let r = Ray::new(
            Tuple::point(1., 1., 1.),
            Tuple::vector(0., 0., 1.),
        );

        let sphere_to_ray =
            r.origin - Tuple::point(0., 0., 0.);

        assert_eq!(
            sphere_to_ray,
            Tuple::vector(1., 1., 1.)
        )
    }
}
