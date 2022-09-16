use crate::intersections::Intersection;
use crate::matrix::M;
use crate::ray::Ray;
use crate::vectors::Tuple;
use uuid::Uuid;

pub struct Sphere {
    pub id: Uuid,
    transform: M,
    transform_inv: M,
    transform_inv_t: M
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            id: Uuid::new_v4(),
            transform: M::ident(4),
            transform_inv: M::ident(4),
            transform_inv_t: M::ident(4),
        }
    }

    pub fn set_transform(&mut self, m: M) {
        self.transform = m.clone();
        self.transform_inv = m.inverse();
        self.transform_inv_t = m.inverse().transpose();
    }

    fn transformed_ray(&self, ray: Ray) -> Ray {
        ray.transform(&self.transform_inv)
    }

    pub fn intersects(&self, ray_original: Ray) -> Vec<Intersection> {
        let ray = self.transformed_ray(ray_original);

        let sphere_to_ray = ray.origin - Tuple::point(0., 0., 0.);
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;
        if discriminant < 0. {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        vec![
            Intersection {
                object_id: self.id,
                t: t1,
            },
            Intersection {
                object_id: self.id,
                t: t2,
            },
        ]
    }

    pub fn normal(&self, p: Tuple) -> Tuple {
        let ob_point = &self.transform_inv * p;
        let ob_normal = ob_point - Tuple::point(0., 0., 0.);
        let mut world_normal = &self.transform_inv_t * ob_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometries::Sphere;
    use crate::ray::Ray;
    use crate::transformations::translation;
    use crate::utils::float_compare;
    use crate::vectors::Tuple;
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn test_intersection_1() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 2);

        assert_eq!(ints[0].t, 4.0);
        assert_eq!(ints[1].t, 6.0);
    }

    #[test]
    fn test_intersection_2() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 2);

        assert_eq!(ints[0].t, 5.0);
        assert_eq!(ints[1].t, 5.0);
    }

    #[test]
    fn test_intersection_miss() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 0);
    }

    #[test]
    fn test_intersection_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 2);

        assert_eq!(ints[0].t, -1.);
        assert_eq!(ints[1].t, 1.);
    }

    #[test]
    fn test_intersection_behind() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));

        let sph = Sphere::new();
        let ints = sph.intersects(r);

        assert_eq!(ints.len(), 2);

        assert_eq!(ints[0].t, -6.);
        assert_eq!(ints[1].t, -4.);
    }

    #[test]
    fn test_point_calc() {
        let r = Ray::new(Tuple::point(1., 1., 1.), Tuple::vector(0., 0., 1.));

        let sphere_to_ray = r.origin - Tuple::point(0., 0., 0.);

        assert_eq!(sphere_to_ray, Tuple::vector(1., 1., 1.))
    }

    #[test]
    fn test_normals() {
        let sph = Sphere::new();

        assert_eq!(
            sph.normal(Tuple::point(1., 0., 0.)),
            Tuple::vector(1., 0., 0.)
        );

        assert_eq!(
            sph.normal(Tuple::point(0., 1., 0.)),
            Tuple::vector(0., 1., 0.)
        );

        assert_eq!(
            sph.normal(Tuple::point(0., 0., 1.)),
            Tuple::vector(0., 0., 1.)
        );

        // nonaxical point:
        let t: f64 = (3_f64).sqrt() / 3.;

        assert_eq!(sph.normal(Tuple::point(t, t, t)), Tuple::vector(t, t, t));
    }

    #[test]
    fn test_normals_translated() {
        let mut sph = Sphere::new();

        sph.set_transform(translation(0., 1., 0.));

        assert_eq!(
            sph.normal(Tuple::point(0., 1. + FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
            Tuple::vector(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );
    }
}
