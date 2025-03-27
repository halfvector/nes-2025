use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("WGPU Framebuffer")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(WINDOW_WIDTH, WINDOW_HEIGHT, &window);
    let mut pixels = Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture).unwrap();
    
    // Create a test pattern (a simple gradient)
    let frame = pixels.frame_mut();
    for y in 0..WINDOW_HEIGHT {
        for x in 0..WINDOW_WIDTH {
            let index = ((y * WINDOW_WIDTH + x) * 4) as usize;
            let scaled_x = ((x as f32 * 255.0 / WINDOW_WIDTH as f32) as u32);
            let scaled_y = ((y as f32 * 255.0 / WINDOW_HEIGHT as f32) as u32);
            frame[index] = u8::try_from(scaled_x).expect("scaled x should be < 256");     // R
            frame[index + 1] = u8::try_from(scaled_y).expect("scaled y should be < 256"); // G
            frame[index + 2] = 255;     // B
            frame[index + 3] = 255;     // A
        }
    }

    let _ = event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
            } => target.exit(),
            Event::AboutToWait => {
                window.request_redraw();
                target.set_control_flow(winit::event_loop::ControlFlow::Poll);
            }
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::RedrawRequested,
            } => {
                if pixels.render().is_err() {
                    target.exit();
                }
            }
            _ => (),
        }
    });
}
