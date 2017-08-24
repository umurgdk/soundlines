extern crate cgmath;
extern crate mint;
extern crate three;
extern crate rand;
#[macro_use]
extern crate ndarray;
extern crate noise;

mod dna;
mod helpers;
mod constants;
mod plant;
mod plant_setting;
mod cell;
mod seed;
mod world;

use cgmath::prelude::*;
use cgmath::Quaternion;
use cgmath::Deg;

use world::World;
use constants::*;

fn main() {
    let shaders_path: String = format!("{}/data/shaders", env!("CARGO_MANIFEST_DIR"));
    let shaders_path_str: &str = shaders_path.as_str();
    let mut win = three::Window::new("Soundlines", shaders_path_str).build();
    let mut cam = win.factory.perspective_camera(75.0, 1.0, 100.0);
    cam.look_at([0.0, 20.0, 20.0], [0.0, 0.0, 0.0], Some([0.0, 1.0, 0.0].into()));

	let mut point_light = win.factory.point_light(0xffffff, 0.9);
	point_light.set_position([15.0, 35.0, 35.0]);
	win.scene.add(&point_light);

    let mut world = World::new(&mut win.factory, 10, 10);
	win.scene.add(&world.group);

	world.group.set_position([-CELL_SIZE * 10.0 / 2.0, 0.0, -CELL_SIZE * 10.0 / 2.0]);
	println!("{:#?}", world.group);

    let mut angle = 0.0f32;
    while win.update() && !three::KEY_ESCAPE.is_hit(&win.input) {
        angle += win.input.get_time_delta() * 0.1;

        cam.look_at([f32::cos(angle) * 20.0, 20.0, f32::sin(angle) * 20.0], [0.0, 0.0, 0.0], Some([0.0, 1.0, 0.0].into()));
	    world.update();

        win.render(&cam);
    }
}
