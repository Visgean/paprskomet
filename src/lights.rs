use crate::colors::Color;
use crate::materials::Material;
use crate::vectors::Tuple;

pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

pub fn lighting(m: Material, light: PointLight, position: Tuple, eye: Tuple, normal: Tuple) -> Color {
    let effective_color = m.color * light.intensity;

    let lightv = (light.position - position).normalize();
    let ambient = effective_color * m.ambient;
    let mut diffuse = Color::black();
    let mut specular = Color::black();

    let light_dot_normal = lightv.dot(&normal);
    if light_dot_normal >= 0. {
        diffuse = effective_color * m.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(&normal);
        let reflect_dot_eye = reflectv.dot(&eye);

        if reflect_dot_eye > 0. {
            specular = light.intensity * m.specular * (reflect_dot_eye.powf(m.shininess))
        }
    }
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use crate::colors::Color;
    use crate::lights::{lighting, PointLight};
    use crate::materials::Material;
    use crate::vectors::Tuple;

    #[test]
    fn test_light_between() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);
        let normal = Tuple::vector(0., 0., -1.);
        let eyev = Tuple::vector(0., 0., -1.);
        let light = PointLight {
            intensity: Color::new(1., 1., 1.),
            position: Tuple::point(0., 0., -10.),
        };

        let r = lighting(m, light, position, eyev, normal);
        assert_eq!(r, Color::new(1.9, 1.9, 1.9))
    }

    #[test]
    fn test_light_up() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);
        let normal = Tuple::vector(0., 0., -1.);

        let sq2_2 = (2_f64).sqrt() / 2. as f64;
        let eyev = Tuple::vector(0., sq2_2, sq2_2);
        let light = PointLight {
            intensity: Color::new(1., 1., 1.),
            position: Tuple::point(0., 0., -10.),
        };

        let r = lighting(m, light, position, eyev, normal);
        assert_eq!(r, Color::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn test_light_down() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);
        let normal = Tuple::vector(0., 0., -1.);
        let eyev = Tuple::vector(0., 0., -1.);
        let light = PointLight {
            intensity: Color::new(1., 1., 1.),
            position: Tuple::point(0., 10., -10.),
        };

        let r = lighting(m, light, position, eyev, normal);
        assert_eq!(r, Color::new(0.7364, 0.7364, 0.7364))
    }

    #[test]
    fn test_light_pong() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);
        let normal = Tuple::vector(0., 0., -1.);

        let sq2_2 = (2_f64).sqrt() / 2. as f64;
        let eyev = Tuple::vector(0., -sq2_2, -sq2_2);
        let light = PointLight {
            intensity: Color::new(1., 1., 1.),
            position: Tuple::point(0., 10., -10.),
        };

        let r = lighting(m, light, position, eyev, normal);
        assert_eq!(r, Color::new(1.6364, 1.6364, 1.6364))
    }

    #[test]
    fn test_light_behind() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);
        let normal = Tuple::vector(0., 0., -1.);

        let eyev = Tuple::vector(0., 0., -1.);
        let light = PointLight {
            intensity: Color::new(1., 1., 1.),
            position: Tuple::point(0., 0., 10.),
        };

        let r = lighting(m, light, position, eyev, normal);
        assert_eq!(r, Color::new(0.1, 0.1, 0.1))
    }
}
