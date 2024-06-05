use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::util::DeviceExt;

async fn run() {
    // Create an event loop
    let event_loop = EventLoop::new();
    // Create a window
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Initialize WGPU
    let backend = wgpu::Backends::VULKAN;
    let instance = wgpu::Instance::new(backend);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.unwrap();
    let (_device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        label: None,
    }, None).await.unwrap();

    // More initialization code here...

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn main() {
    pollster::block_on(run());
}