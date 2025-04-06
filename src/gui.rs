#![allow(dead_code)] // Suppress dead code warnings for this whole module

// Import necessary libraries for pixel management and window handling
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

// Constants use `const` keyword (like C/C++/C#) but type comes after name
// `u32` is unsigned 32-bit integer (equivalent to uint in C#/Go)
const WINDOW_WIDTH: u32 = 640; // Width of the window in pixels
const WINDOW_HEIGHT: u32 = 480; // Height of the window in pixels

// `pub fn` makes this function public, allowing it to be called from main.rs
pub fn run_gui() {
    // `env_logger::init()` - Module path syntax (::) like C# namespace
    // No parentheses for zero-arg functions unlike Python/JS
    env_logger::init();
    
    // `let` declares variables (immutable by default, unlike JS/Go)
    // `EventLoop::new()` - Associated function (static method) call
    // `unwrap()` handles Result type - panics on error (similar to .Unwrap() in C#)
    let event_loop = EventLoop::new().unwrap();
    
    // Builder pattern with method chaining (common in Rust vs C#'s object initializers)
    let window = WindowBuilder::new() // Constructor-like pattern
        .with_title("WGPU Framebuffer") // Fluent interface method
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop) // `&` creates reference (similar to C++ address-of)
        .unwrap(); // Handle potential error immediately

    // `SurfaceTexture::new()` - Another associated function
    // Parameters passed by value unless prefixed with `&` (explicit borrowing)
    let surface_texture = SurfaceTexture::new(WINDOW_WIDTH, WINDOW_HEIGHT, &window);
    
    // `mut` makes variable mutable (Rust variables immutable by default like F#)
    let mut pixels = Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture).unwrap();
    
    // Get mutable frame buffer reference (`mut` required for write access)
    let frame = pixels.frame_mut();
    
    // Range syntax `0..HEIGHT` similar to Python/JS, but exclusive upper bound
    // Correction: Should be WINDOW_HEIGHT and WINDOW_WIDTH
    for y in 0..WINDOW_HEIGHT { 
        for x in 0..WINDOW_WIDTH {
            // Explicit type casting with `as` (like C-style cast but checked)
            // `usize` is pointer-sized integer (like C++ size_t)
            let index = (y * WINDOW_WIDTH + x) as usize * 4; // Calculate index in the RGBA frame buffer
            // `frame` is a slice (`&[u8]`), index access like arrays in other languages
            // Set pixel color (e.g., simple gradient)
            frame[index] = x as u8;        // Red component
            frame[index + 1] = y as u8;    // Green component
            frame[index + 2] = 0;          // Blue component
            frame[index + 3] = 255;        // Alpha component (opaque)
        }
    }

    // Render the frame buffer to the window
    // `if let Err(...) = ...` pattern matching for error handling (idiomatic Rust)
    if let Err(err) = pixels.render() {
        // Use `log::error!` macro for logging errors (requires `log` crate usually)
        // Need to add `log` crate dependency if not present. Assuming env_logger implies log.
        // log::error!("pixels.render() failed: {}", err);
        eprintln!("pixels.render() failed: {}", err); // Use eprintln! for now
        // Consider returning an error instead of panicking or exiting directly
        return; // Exit the function gracefully on render error
    }

    // Start the event loop (blocking operation)
    // `move || { ... }` is a closure that captures its environment by value
    event_loop.run(move |event, elwt| {
        // Match expression (like switch in C#/Java, but more powerful)
        match event {
            // Handle window events
            Event::WindowEvent { event, .. } => match event {
                // Close requested (e.g., user clicks the 'X' button)
                WindowEvent::CloseRequested => {
                    // Control flow for the event loop (exits the loop)
                    elwt.exit(); 
                }
                // Window needs repaint
                WindowEvent::RedrawRequested => {
                    // Render again if needed (e.g., if frame changes)
                    if let Err(err) = pixels.render() {
                        // log::error!("pixels.render() failed: {}", err);
                        eprintln!("pixels.render() failed during loop: {}", err); // Use eprintln!
                        elwt.exit(); // Exit if rendering fails during loop
                    }
                }
                _ => (), // Ignore other window events
            },
            _ => (), // Ignore other event types
        }
    }).unwrap(); // Handle potential errors from event_loop.run() itself
}
