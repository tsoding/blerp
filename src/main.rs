#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;

use glium::Surface;
use glium_sdl2::DisplayBuild;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("blerp", 800, 600)
        .position_centered()
        .opengl()
        .build_glium()
        .unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let (mut x, mut y): (f32, f32) = (0.0, 0.0);
    let t: f32 = 0.002;

    let vertex_shader_src = r#"
        #version 130

        in vec2 position;

        uniform float x;
        uniform float y;

        void main() {
            vec2 pos = position;
            pos.x += x;
            pos.y += y;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 130

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&window, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    while running {
        let mut target = window.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform!{ x: x, y: y },
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..  } => {
                    running = false;
                },

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    y -= t;
                }

                _ => ()
            }
        }
    }
}
