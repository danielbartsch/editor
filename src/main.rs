mod raytrace;
use crate::raytrace::raytrace::*;

#[path="editor/sdl_experiment.rs"]
mod sdl_experiment;
use crate::sdl_experiment::sdl2::*;

fn main() {
    run();

    render(&Scene {
        width: 600,
        height: 600,
        fov: 70.0,
        light: Light {
            direction: Vector3::new(-0.1, -1.0, -0.1),
            color: ColorRGB::new(255,255,255),
            intensity: 1.0,
        },
        elements: vec![
            Element::Sphere(Sphere {
                center: Vector3::new(-10.0, 0.0, -60.0),
                radius: 10.0,
                color: ColorRGB::new(255,255,255),
                albedo: std::f32::consts::PI * 2.0,
            }),
            Element::Sphere(Sphere {
                center: Vector3::new(-15.0, 0.0, -60.0),
                radius: 10.0,
                color: ColorRGB::new(255,0,0),
                albedo: std::f32::consts::PI * 2.0,
            }),
            Element::Sphere(Sphere {
                center: Vector3::new(-25.0, 0.0, -60.0),
                radius: 10.0,
                color: ColorRGB::new(0,255,0),
                albedo: std::f32::consts::PI * 2.0,
            }),
            Element::Sphere(Sphere {
                center: Vector3::new(-35.0, 0.0, -60.0),
                radius: 10.0,
                color: ColorRGB::new(0,0,255),
                albedo: std::f32::consts::PI * 2.0,
            }),
            Element::Sphere(Sphere {
                center: Vector3::new(-1.5, 0.0, -5.0),
                radius: 1.0,
                color: ColorRGB::new(0,0,0),
                albedo: std::f32::consts::PI * 2.0,
            }),
            Element::Plane(Plane {
                origin: Vector3::new(0.0, 0.0, 0.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
                color: ColorRGB::new(255,0,0),
                albedo: 0.1,
            })
        ]
    }).save("raytrace.png").unwrap();
}