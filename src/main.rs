#[macro_use]
extern crate glium;

mod input;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use glium::{ DisplayBuild, Surface, glutin };
use glium::backend::glutin_backend::GlutinFacade;
use input::InputHandler;

fn main() {
    let display = glutin::WindowBuilder::new().build_glium().unwrap();
    let window_size = {
        let target = display.draw();
        let (width, height) = target.get_dimensions();
        target.finish().unwrap();
        std::cmp::max(width, height)
    };


    let mut input_handler = InputHandler::new();

    let data = get_vertex_data();

    let vertex_buffer = glium::VertexBuffer::new(&display, &data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = make_program(&display);

    let mut uniforms = Uniforms { num_iterations: 100, window_size: window_size as i32, x_shift: -0.5, y_shift: 0.5, zoom: 0.3 };

    loop {
        let mut target = display.draw();
        input_handler.set_uniforms(&mut uniforms);

        target.clear_color(1.0, 1.0, 1.0, 0.0);

        target.draw(&vertex_buffer, &indices, &program,
                    &uniform!{  num_iterations: uniforms.num_iterations, zoom: uniforms.zoom,
                                window_size: uniforms.window_size,
                                x_shift: uniforms.x_shift, y_shift: uniforms.y_shift },
                    &Default::default()).unwrap();

        target.finish().unwrap();

        for e in display.poll_events() {
            use glium::glutin::Event;

            let should_close: bool = match e {
                Event::Closed => true,
                Event::KeyboardInput(state, raw, code) => input_handler.keyinput(state, raw, code),
                _ => false,
            };

            if should_close {
                return;
            }
        }
    }
}

pub struct Uniforms {
    num_iterations: i32,
    window_size: i32,
    x_shift: f32,
    y_shift: f32,
    zoom: f32,
}

fn get_vertex_data() -> Vec<Vertex> {
    let p1 = Vertex { position: [-1.0, -1.0] };
    let p2 = Vertex { position: [-1.0, 1.0] };
    let p3 = Vertex { position: [1.0, -1.0] };
    let p4 = Vertex { position: [1.0, -1.0] };
    let p5 = Vertex { position: [1.0, 1.0] };
    let p6 = Vertex { position: [-1.0, 1.0] };

    vec![p1, p2, p3, p4, p5, p6]
}

fn make_program(display: &GlutinFacade) -> glium::Program {
    let vertex_shader_src = load_shader("src/shader/vert_shader.glslv");
    let fragment_shader_src = load_shader("src/shader/frag_shader.glslf");

    glium::Program::from_source(display, &vertex_shader_src,
                                &fragment_shader_src, None).unwrap()
}


fn load_shader(file: &str) -> String {
    let path = Path::new(file);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

#[derive(Copy, Clone)]
struct Vertex {
        position: [f32; 2],
}

implement_vertex!(Vertex, position);
