use crate::computer::KeyPress;
use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{ElementState, Event, StartCause, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    Surface,
};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, Notify};

const HEIGHT: f64 = 160.0;
const WIDTH: f64 = 240.0;

const VERTEX_SHADER_DARW_POINTS_SRC: &str = r#"
        #version 140
        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
const FRAGMENT_SHADER_DRAW_POINTS_SRC: &str = r#"
        #version 140
        out vec4 color;
        uniform vec4 in_color;

        void main() {
            color = in_color;
        }
    "#;

pub fn glfw_run(
    mut screen_receiver: mpsc::Receiver<[[u8; 240]; 160]>,
    mut key_press_sender: mpsc::Sender<KeyPress>,
    quit: Arc<Notify>,
) {
    println!("Starting glfw");

    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_title("Testing");
    let cb = ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut draw_parameters = glium::DrawParameters::default();
    draw_parameters.point_size = Some(10.0);
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    let darw_points = Arc::new(Mutex::new(Vec::<Vertex>::new()));
    let (key_press_local_sender, key_press_local_receiver) = std::sync::mpsc::sync_channel(1);
    {
        let darw_points = darw_points.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(66)).await;
                if let Some(screen_data) = screen_receiver.recv().await {
                    println!("Received screen data");
                    for x in 0..screen_data.len() {
                        for y in 0..screen_data[x].len() {
                            if screen_data[x][y] > 0 {
                                let x_len = HEIGHT / 2.0;
                                let y_len = WIDTH / 2.0;
                                // let x = (x as i32 - x_len as i32) as f32;
                                // let y = (y as i32 - y_len as i32) as f32;
                                darw_points.lock().unwrap().push(Vertex {
                                    position: [y as f32 / y_len as f32, x as f32 / x_len as f32],
                                });
                            }
                        }
                    }
                }
            }
        });

        let key_press_local_receiver = Arc::new(Mutex::new(key_press_local_receiver));
        tokio::spawn(async move {
            loop {
                let key_press = key_press_local_receiver.lock().unwrap().recv().unwrap();
                key_press_sender.send(key_press).await;
            }
        });

        tokio::spawn(async move {
            quit.notified().await;
            std::process::exit(0);
        });
    }

    //TODO:Clear canvas

    let draw_points = glium::Program::from_source(
        &display,
        VERTEX_SHADER_DARW_POINTS_SRC,
        FRAGMENT_SHADER_DRAW_POINTS_SRC,
        None,
    )
    .unwrap();

    let mut cnt = 0;
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_millis(33);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target
            .draw(
                &glium::VertexBuffer::new(&display, &darw_points.lock().unwrap()).unwrap(),
                &glium::index::NoIndices(glium::index::PrimitiveType::Points),
                &draw_points,
                &uniform! { in_color: [1.0, 0.0, 0.0, 0.0f32] },
                &draw_parameters,
            )
            .unwrap();

        target.finish().unwrap();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    if input.virtual_keycode.unwrap() == VirtualKeyCode::Escape {
                        std::process::exit(0);
                    } else if (input.virtual_keycode.unwrap() >= VirtualKeyCode::A
                        && input.virtual_keycode.unwrap() <= VirtualKeyCode::Z)
                        || (input.virtual_keycode.unwrap() >= VirtualKeyCode::Key1
                            && input.virtual_keycode.unwrap() <= VirtualKeyCode::Key0)
                    {
                        // Send keypress
                        match input.state {
                            ElementState::Pressed => {
                                key_press_local_sender.send(KeyPress {
                                    value: input.virtual_keycode.unwrap() as i32,
                                    is_down: true,
                                });
                            }
                            ElementState::Released => {
                                key_press_local_sender.send(KeyPress {
                                    value: input.virtual_keycode.unwrap() as i32,
                                    is_down: false,
                                });
                            }
                        }
                    }
                }
                _ => return,
            },
            Event::RedrawEventsCleared => {
                let mut target = display.draw();
                target.clear_color(0.5, 0.5, 0.5, 1.0);
                target.finish().unwrap();
                display.swap_buffers().unwrap();
            }
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },
            Event::LoopDestroyed => {
                println!("Destroyed");
            }
            _ => return,
        }
    });
}
