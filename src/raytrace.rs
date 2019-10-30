pub mod raytrace {
    use image::ImageBuffer;
    use image::RgbImage;
    use std::ops;

    #[derive(Copy, Clone, Debug)]
    pub struct Vector3(f64, f64, f64);

    impl Vector3 {
        pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
            Vector3(x, y, z)
        }

        fn dot(&self, with: &Vector3) -> f64 {
            self.0 * with.0 +
            self.1 * with.1 +
            self.2 * with.2
        }

        fn normalize(&self) -> Vector3 {
            let inverse_length = self.length().recip();
            Vector3(self.0 * inverse_length, self.1 * inverse_length, self.2 * inverse_length)
        }

        fn norm(&self) -> f64 {
            self.dot(&self)
        }

        fn length(&self) -> f64 {
            self.norm().sqrt()
        }
    }

    impl ops::Add for Vector3 {
        type Output = Self;
        fn add(self, right_hand_side: Self) -> Self {
            Self(
                self.0 + right_hand_side.0,
                self.1 + right_hand_side.1,
                self.2 + right_hand_side.2,
            )
        }
    }

    impl ops::Sub for Vector3 {
        type Output = Self;
        fn sub(self, right_hand_side: Self) -> Self {
            Self(
                self.0 - right_hand_side.0,
                self.1 - right_hand_side.1,
                self.2 - right_hand_side.2,
            )
        }
    }

    impl ops::Neg for Vector3 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(-self.0, -self.1, -self.2)
        }
    }

    impl ops::Mul for Vector3 {
        type Output = Self;
        fn mul(self, right_hand_side: Self) -> Self {
            Vector3(
                self.1 * right_hand_side.2 - self.2 * right_hand_side.1,
                self.2 * right_hand_side.0 - self.0 * right_hand_side.2,
                self.0 * right_hand_side.1 - self.1 * right_hand_side.0,
            )
        }
    }

    impl ops::Mul<f64> for Vector3 {
        type Output = Self;
        fn mul(self, right_hand_side: f64) -> Self {
            Vector3(self.0 * right_hand_side, self.1 * right_hand_side, self.2 * right_hand_side)
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct ColorRGB(u8, u8, u8);

    impl ColorRGB {
        pub fn new(red: u8, green: u8, blue: u8) -> ColorRGB {
            ColorRGB(red, green, blue)
        }
        fn to_array(&self) -> [u8; 3] {
            [
                self.0,
                self.1,
                self.2,
            ]
        }
    }

    impl ops::Mul for ColorRGB {
        type Output = Self;
        fn mul(self, right_hand_side: Self) -> Self {
            Self(
                ((self.0 as u16 * right_hand_side.0 as u16) / 255) as u8,
                ((self.1 as u16 * right_hand_side.1 as u16) / 255) as u8,
                ((self.2 as u16 * right_hand_side.2 as u16) / 255) as u8,
            )
        }
    }

    impl ops::Mul<f32> for ColorRGB {
        type Output = Self;
        fn mul(self, right_hand_side: f32) -> Self {
            Self(
                ((self.0 as f32) * right_hand_side) as u8,
                ((self.1 as f32) * right_hand_side) as u8,
                ((self.2 as f32) * right_hand_side) as u8,
            )
        }
    }

    pub trait DistanceFn {
        fn distance(&self, ray: &Ray) -> f64;
    }

    pub trait SurfaceNormalFn {
        fn normal(&self, hit_point: &Vector3) -> Vector3;
    }

    #[derive(Debug)]
    pub struct Sphere {
        pub center: Vector3,
        pub radius: f64,
        pub color: ColorRGB,
        pub albedo: f32,
    }

    impl DistanceFn for Sphere {
        fn distance(&self, ray: &Ray) -> f64 {
            let ray_origin_intersect = self.center - ray.origin;

            let ray_direction_to_sphere = ray_origin_intersect.dot(&ray.direction);
            
            let distance_to_sphere_center_squared = ray_origin_intersect.dot(&ray_origin_intersect) - (ray_direction_to_sphere * ray_direction_to_sphere);

            distance_to_sphere_center_squared - (self.radius * self.radius)
        }
    }

    impl SurfaceNormalFn for Sphere {
        fn normal(&self, hit_point: &Vector3) -> Vector3 {
            (*hit_point - self.center).normalize()
        }
    }

    #[derive(Debug)]
    pub struct Plane {
        pub origin: Vector3,
        pub normal: Vector3,
        pub color: ColorRGB,
        pub albedo: f32,
    }

    impl DistanceFn for Plane {
        fn distance(&self, ray: &Ray) -> f64 {
            let normal = &self.normal;
            let denom = normal.dot(&ray.direction);
            if denom > 1e-6 {
                (self.origin - ray.origin).dot(&normal) / denom
            } else {
                1.7976931348623157e+308_f64
            }
        }
    }

    impl SurfaceNormalFn for Plane {
        fn normal(&self, _: &Vector3) -> Vector3 {
            -self.normal
        }
    }

    #[derive(Debug)]
    pub struct Intersection<'a> {
        pub distance: f64,
        pub element: &'a Element,
    }

    #[derive(Debug)]
    pub enum Element {
        Sphere(Sphere),
        Plane(Plane),
    }

    impl Element {
        pub fn color(&self) -> ColorRGB {
            match *self {
                Element::Sphere(ref s) => s.color,
                Element::Plane(ref s) => s.color,
            }
        }
        pub fn albedo(&self) -> f32 {
            match *self {
                Element::Sphere(ref s) => s.albedo,
                Element::Plane(ref s) => s.albedo,
            }
        }
    }

    impl DistanceFn for Element {
        fn distance(&self, ray: &Ray) -> f64 {
            match *self {
                Element::Sphere(ref s) => s.distance(ray),
                Element::Plane(ref s) => s.distance(ray),
            }
        }
    }

    impl SurfaceNormalFn for Element {
        fn normal(&self, hit_point: &Vector3) -> Vector3 {
            match *self {
                Element::Sphere(ref s) => s.normal(hit_point),
                Element::Plane(ref s) => s.normal(hit_point),
            }
        }
    }

    pub struct Light {
        pub direction: Vector3,
        pub color: ColorRGB,
        pub intensity: f32,
    }

    pub struct Scene {
        pub width: u32,
        pub height: u32,
        pub fov: f64,
        pub elements: Vec<Element>,
        pub light: Light,
    }

    impl Scene {
        pub fn trace(&self, ray: &Ray) -> Intersection {
            let mut min_intersection = Intersection {
                distance: 1.7976931348623157e+308_f64,
                element: &Element::Plane(Plane {
                    origin: Vector3(0.0, 0.0, 0.0),
                    normal: Vector3(0.0, 0.0, 0.0),
                    albedo: 0.0,
                    color: ColorRGB(0,0,0),
                })
            };
            let intersections = self.elements
                .iter()
                .map(|s| Intersection { distance: s.distance(&ray), element: s });

            for intersection in intersections {
                if intersection.distance < min_intersection.distance {
                    min_intersection = intersection
                }
            }
            min_intersection
        }
    }

    #[derive(Debug)]
    pub struct Ray {
        pub origin: Vector3,
        pub direction: Vector3,
    }

    impl Ray {
        pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
            let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
            let aspect_ratio = (scene.width as f64) / (scene.height as f64);
            let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
            let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

            Ray {
                origin: Vector3(0.0, 0.0, 0.0),
                direction: Vector3(sensor_x, sensor_y, -1.0).normalize(),
            }
        }
    }

    fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> ColorRGB {
        let hit_point = ray.origin + (ray.direction * intersection.distance);
        let surface_normal = intersection.element.normal(&hit_point);
        let direction_to_light = -scene.light.direction;

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * 1e-13),
            direction: direction_to_light,
        };
        let is_in_shadow = scene.trace(&shadow_ray).distance <= 0.0;

        let light_intensity = if is_in_shadow { 0.0 } else { scene.light.intensity };

        let light_power = (surface_normal.dot(&direction_to_light) as f32) * light_intensity;
        let light_reflected = intersection.element.albedo() / std::f32::consts::PI;
        
        intersection.element.color() * scene.light.color * light_power * light_reflected
    }

    pub fn render(scene: &Scene) -> RgbImage {
        ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
            let ray = Ray::create_prime(x, y, scene);

            let intersection = scene.trace(&ray);

            image::Rgb(
                if intersection.distance <= 0.0 {
                    get_color(&scene, &ray, &intersection).to_array()
                } else {
                    [140,140,255]
                }
            )
        })
    }
}