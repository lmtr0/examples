mod state;
mod mgr;
use mgr::Mgr;
use glfw::{Context, Key, Action};
// use state::State;

pub async fn run() {
    env_logger::builder().default_format().filter(None, log::LevelFilter::Info).init();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    Mgr::new(window);

    window.set_key_polling(true);

    // window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            // handle_window_event(&mut window, event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },

                _ => {}
            }

        }
    }


    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();

    // let mut state = State::new(&window).await;

    // event_loop.run(move |event, _, control_flow| 
    //     match event {
    //         Event::RedrawRequested(window_id) if window_id == window.id() => {
    //             state.update();
    //             match state.render() {
    //                 Ok(_) => {}
    //                 // Reconfigure the surface if lost
    //                 Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
    //                 // The system is out of memory, we should probably quit
    //                 Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
    //                 // All other errors (Outdated, Timeout) should be resolved by the next frame
    //                 Err(e) => log::error!("{:?}", e),
    //             }
    //         }
    //         Event::MainEventsCleared => {
    //             // RedrawRequested will only trigger once, unless we manually
    //             // request it.
    //             window.request_redraw();
    //         }
    
    //         // window events
    //         Event::WindowEvent {
    //             ref event,
    //             window_id,
    //         } if window_id == window.id() => 
    //             match event {
                    
    //                 WindowEvent::CloseRequested
    //                 | WindowEvent::KeyboardInput {
    //                     input:
    //                         KeyboardInput {
    //                             state: ElementState::Pressed,
    //                             virtual_keycode: Some(VirtualKeyCode::Escape),
    //                             ..
    //                         },
    //                     ..
    //                 } => *control_flow = ControlFlow::Exit,
                    
    //                 WindowEvent::Resized(physical_size) => {
    //                     state.resize(*physical_size);
    //                 },
    //                 WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
    //                     // new_inner_size is &&mut so we have to dereference it twice
    //                     state.resize(**new_inner_size);
    //                 }
    //                 _ => {}
    //             },
    //         _ => {}
    //     }
    // );
}



fn main() {
    println!("Running....");
    pollster::block_on(run());
    println!("Finished running");
}