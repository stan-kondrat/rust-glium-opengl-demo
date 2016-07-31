#[macro_use]
extern crate glium;

mod triangle;
mod text;
mod fps;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let mut triangle = triangle::Triangle::new(&display);
    let mut logs = text::Text::new(&display);
    let mut fps = fps::FPS::new();

    loop {
        let mut target = display.draw();

        target.clear_color(0.9, 0.9, 0.9, 1.0);

        // rotate triangle
        triangle.direction += 0.5;
        if triangle.direction >= 360.0 {
            triangle.direction = 0.0;
        }

        triangle.set_target(&mut target);

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
