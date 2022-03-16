use glium::{glutin, Surface};
use glium::{uniform, implement_vertex, program};
use glium::index::PrimitiveType;
use glium::glutin::dpi::Size;
use glium::glutin::dpi::PhysicalSize;

pub struct DrawBuffer {
    pub vertex_buffer: &'static [Vertex],
    pub index_buffer: &'static [u32],
    pub program_vert: &'static str,
    pub program_frag: &'static str,
}

pub struct ShapeBuffer {
    pub shapes: Box<Vec<Shape>>,
}

pub enum Shape {
    Circ(Circle)
}

pub struct Circle {
    pub center: [f32; 2],
    pub radius: f32,
}

pub struct Renderer {}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

impl Renderer {
    pub fn render(b: &DrawBuffer, mut sb: ShapeBuffer, w: u32, ha: bool) {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_inner_size(Size::Physical(PhysicalSize{width: w, height: w})).with_resizable(false);
        let cb = glutin::ContextBuilder::new().with_hardware_acceleration(Some(ha));
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
        // // building the vertex buffer, which contains all the vertices that we will draw
        let vertex_buffer = {
            glium::VertexBuffer::new(&display, b.vertex_buffer).unwrap()
        };
    
        let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, b.index_buffer).unwrap();
    
        let program = program!(&display, 
            140 => {
                vertex: b.program_vert,
                fragment: b.program_frag
            }
        ).unwrap();
        
        let mut pressed = false;
        let mut already_pressed = false;
        let mut mouse_pos: [f32;2] = [0.0, 0.0];
        event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    glutin::event::WindowEvent::CursorMoved {
                        position, ..
                    } => {
                        mouse_pos = [(position.x as f32 - 750f32)/750f32, -(position.y as f32 - 750f32)/750f32];
                        return;
                    },
                    glutin::event::WindowEvent::MouseInput {
                        state, ..
                    } => {
                        match state {
                            glutin::event::ElementState::Pressed => {pressed = true;},
                            glutin::event::ElementState::Released => {pressed = false;},
                        }
                        return;
                    },
                    glutin::event::WindowEvent::KeyboardInput {
                        input, ..
                    } => {
                        match input.state {
                            glutin::event::ElementState::Pressed => {
                                match input.virtual_keycode {
                                    Some(v) => {
                                        match v {
                                            glutin::event::VirtualKeyCode::N => {(*sb.shapes).push(Shape::Circ(Circle {
                                                center: [-0.5f32, -0.5f32],
                                                radius: 1.0 as f32,
                                            })); return; },
                                            _ => (),
                                        }
                                    },
                                    None => ()
                                }
                            },
                            _ => ()
                        }
                    },
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }
        
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(0);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            let mut target = display.draw();
            let mut selected_shape : Option<&mut Circle> = None;
            target.clear_color(1.0, 1.0, 1.0, 1.0);
            for shape in (*sb.shapes).iter_mut() {
                match shape {
                    Shape::Circ(c) => {
                        if (c.center[0]-mouse_pos[0]) * (c.center[0]-mouse_pos[0]) + (c.center[1]-mouse_pos[1]) * (c.center[1]-mouse_pos[1]) <= 0.01 * c.radius * c.radius {
                            if pressed {
                                if !already_pressed {
                                    selected_shape = Some(c);
                                    already_pressed = true;
                                }

                            }
                        }
                    },
                };
            }

            match selected_shape {
                Some(s) => {
                    s.center = mouse_pos;
                },
                None => (),
            }

            for shape in (*sb.shapes).iter() {
                match shape {
                    Shape::Circ(c) => {
                        target.draw(&vertex_buffer, &index_buffer, &program, &uniform! { cent: c.center, radiusScale: c.radius},
                            &Default::default()).unwrap();
                    },
                };
            }
            already_pressed = false;
            // target.draw(&vertex_buffer, &index_buffer, &program, &uniform! { cent: [0.5 as f32, 0.5f32], radiusScale: 1f32},
            //     &Default::default()).unwrap();
            target.finish().unwrap();
        });
    }
}