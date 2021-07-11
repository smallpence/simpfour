extern crate glium;

use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::ContextBuilder;
use glium::{Display, Surface};
use glium::glutin::event_loop::ControlFlow::{Wait, Exit};
use glium::glutin::event::Event;
use glium::glutin::event::WindowEvent;

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024,768))
        .with_title("Hello World!");
    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id
            } => *control_flow = Exit,
            _ => {}
        }

        let mut frame = display.draw();
        frame.clear_color(0.0,0.0,1.0,1.0);
        frame.finish().unwrap();
    })
}
