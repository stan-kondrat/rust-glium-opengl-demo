extern crate glium_text;
extern crate glium;

pub struct Text<'a> {
    value: String,
    font: glium_text::FontTexture,
    system: glium_text::TextSystem,
    display: &'a glium::Display,
}

impl<'a> Text<'a> {
    pub fn new(display: &'a glium::Display) -> Text<'a> {
        Text {
            value: String::from("logs"),
            display: display,
            font: glium_text::FontTexture::new(display,
                                               &include_bytes!("OpenSans-Light.ttf")[..],
                                               20)
                .unwrap(),
            system: glium_text::TextSystem::new(display),
        }
    }

    pub fn set_text(&mut self, value: String) {
        self.value = value;
    }

    pub fn set_target(&mut self, target: &mut glium::Frame) {
        let (w, h) = self.display.get_framebuffer_dimensions();
        let text = glium_text::TextDisplay::new(&self.system, &self.font, &self.value);
        let text_width = text.get_width();
        let color = (0.0, 0.0, 1.0, 1.0);
        let matrix: [[f32; 4]; 4] = [[2.0 / text_width, 0.0, 0.0, 0.0],
                                     [0.0, 2.0 * (w as f32) / (h as f32) / text_width, 0.0, 0.0],
                                     [0.0, 0.0, 1.0, 0.0],
                                     [-1.0, -1.0, 0.0, 1.0f32]];
        glium_text::draw(&text, &self.system, target, matrix, color);
    }
}

#[test]
#[ignore]
fn test_text() {
    assert!(false);
}

#[test]
#[ignore]
fn test_text_set_target() {
    assert!(false);
}

#[test]
#[ignore]
fn test_text_set_text() {
    assert!(false);
}
