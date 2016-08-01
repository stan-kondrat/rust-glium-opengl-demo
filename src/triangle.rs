extern crate glium;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

pub struct Triangle {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    program: glium::Program,
    pub direction: f32,
    pub position_x: f32,
    pub position_y: f32,
}

impl<'a> Triangle {
    pub fn new(display: &'a glium::Display) -> Triangle {

        implement_vertex!(Vertex, position);

        let vertex1 = Vertex { position: [-0.01, -0.02] };
        let vertex2 = Vertex { position: [0.0, 0.02] };
        let vertex3 = Vertex { position: [0.01, -0.02] };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();


        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            uniform mat4 matrix;
            void main() {
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(0.5, 0.0, 0.0, 1.0);
            }
        "#;

        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        Triangle {
            vertex_buffer: vertex_buffer,
            indices: indices,
            program: program,
            direction: 0.0,
            position_x: 0.0,
            position_y: 0.0,
        }
    }

    pub fn set_target(&mut self, target: &mut glium::Frame) {
        use glium::Surface;

        let angle = self.direction.to_radians();

        let uniforms = uniform! {
            matrix: [
                [ angle.cos(), angle.sin(), 0.0, 0.0],
                [-angle.sin(), angle.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [self.position_x, self.position_y, 0.0, 1.0f32], // x: 1=left, y: 1=top
            ]
        };

        target.draw(&self.vertex_buffer,
                  &self.indices,
                  &self.program,
                  &uniforms,
                  &Default::default())
            .unwrap();
    }
}

#[test]
#[ignore]
fn test_triangle() {
    assert!(false);
}

#[test]
#[ignore]
fn test_triangle_direction() {
    assert!(false);
}

#[test]
#[ignore]
fn test_triangle_set_target() {
    assert!(false);
}
