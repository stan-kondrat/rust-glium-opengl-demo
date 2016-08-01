#[macro_use]
extern crate glium;
extern crate rand;

mod triangle;
mod text;
mod fps;

fn main() {
    use glium::{DisplayBuild, Surface};
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let mut triangles: Vec<triangle::Triangle> = vec![];
    for _ in 0..100 {
        let mut triangle = triangle::Triangle::new(&display);
        triangle.position_x = rng.gen_range(-1.0f32, 1.0f32);
        triangle.position_y = rng.gen_range(-1.0f32, 1.0f32);
        triangles.push(triangle);
    }

    let mut logs = text::Text::new(&display);
    let mut fps = fps::FPS::new();

    let mut direction: f32 = 0.0;

    loop {
        let mut target = display.draw();

        target.clear_color(0.9, 0.9, 0.9, 1.0);

        // rotate triangle
        direction += 0.5;
        if direction >= 360.0 {
            direction = 0.0;
        }

        for triangle in &mut triangles {
            triangle.direction = direction;
            triangle.set_target(&mut target);
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
