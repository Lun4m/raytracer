use {
    anyhow::{Context, Result},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::Window,
    },
};

mod render;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

async fn connect_to_gpu(window: &Window) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface)> {
    use wgpu::TextureFormat::{Bgra8Unorm, Rgba8Unorm};

    // Create wgpu API entry point
    let instance = wgpu::Instance::default();

    // Create a drawable surface, associated with the window
    let surface = instance.create_surface(window)?;

    // Request GPU
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("failed to find adapter")?;

    // Connect to the GPU
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .context("failed to connect to the GPU")?;

    // Configure texture memory
    let caps = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .into_iter()
        .find(|it| matches!(it, Rgba8Unorm | Bgra8Unorm))
        .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 3,
    };

    surface.configure(&device, &config);

    Ok((device, queue, surface))
}

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
    let window_attrs = Window::default_attributes()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("GPU Path Tracer".to_string());

    // TODO: replace with non deprecated code we the mantainers update the docs
    let window = event_loop.create_window(window_attrs)?;

    let (device, queue, surface) = connect_to_gpu(&window).await?;
    let renderer = render::PathTracer::new(device, queue, WIDTH, HEIGHT);

    // TODO: replace with non deprecated code we the mantainers update the docs
    event_loop.run(|event, control_handle| {
        control_handle.set_control_flow(ControlFlow::Poll);
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => control_handle.exit(),
                WindowEvent::RedrawRequested => {
                    let frame: wgpu::SurfaceTexture = surface
                        .get_current_texture()
                        .expect("should be able to get current texture");

                    let render_target = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    renderer.render_frame(&render_target);

                    frame.present();
                    window.request_redraw();
                }
                _ => (),
            }
        }
    })?;

    Ok(())
}
