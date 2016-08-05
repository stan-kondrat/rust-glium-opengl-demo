#[macro_use]
extern crate glium;
extern crate rand;

mod triangle;
mod text;
mod fps;
mod point;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use glium::{DisplayBuild, Surface};
use rand::{thread_rng, Rng};

const POINTS: usize = 100;
const EPSILON: f32 = 0.01;
const MAX_SPEED: f32 = 0.01;
const MIN_SPEED: f32 = 0.0001;

struct World {
    time: u64,
    points: Vec<point::Point>,
}

fn main() {

    let mut rng = thread_rng();

    let mut world: World = World {
        time: 0,
        points: vec![],
    };

    for _ in 0..POINTS {
        world.points.push(point::Point::new(rng.gen_range(-1.0f32, 1.0f32),
                                            rng.gen_range(-1.0f32, 1.0f32),
                                            rng.gen_range(0.0f32, 360.0f32),
                                            rng.gen_range(MIN_SPEED, MAX_SPEED)))
    }

    let world = Arc::new(Mutex::new(world));
    let world_clone = world.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        loop {
            let mut world = world_clone.lock().expect("Unable to lock");
            world.time += 1;
            for point in &mut world.points {
                point.step();
            }
        }
    });

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let mut logs = text::Text::new(&display);
    let mut fps = fps::FPS::new();

    let mut triangles: Vec<triangle::Triangle> = vec![];
    for _ in 0..POINTS {
        let mut triangle = triangle::Triangle::new(&display);
        triangle.position_x = rng.gen_range(-1.0f32, 1.0f32);
        triangle.position_y = rng.gen_range(-1.0f32, 1.0f32);
        triangles.push(triangle);
    }

    loop {
        let world = world.lock().expect("Unable to lock");
        let mut target = display.draw();
        target.clear_color(0.9, 0.9, 0.9, 1.0);

        for i in 0..POINTS {
            triangles[i].position_x = world.points[i].x;
            triangles[i].position_y = world.points[i].y;
            triangles[i].direction = world.points[i].direction;
            triangles[i].set_target(&mut target);
        }

        fps.calc();
        logs.set_text(fps.to_string());
        logs.set_target(&mut target);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
