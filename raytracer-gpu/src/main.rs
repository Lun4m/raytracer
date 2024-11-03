use {
    algebra::Vec3,
    anyhow::{Context, Result},
    camera::Camera,
    std::sync::Arc,
    winit::{
        application::ApplicationHandler,
        event::{DeviceEvent, MouseScrollDelta, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        keyboard::Key,
        window::Window,
    },
};

mod algebra;
mod camera;
mod render;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

async fn connect_to_gpu<'a>(
    window: Arc<Window>,
) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface<'a>)> {
    use wgpu::TextureFormat::{Bgra8UnormSrgb, Rgba8UnormSrgb};

    // Create wgpu API entry point
    let instance = wgpu::Instance::default();

    // Create a drawable surface, associated with the window
    let surface = instance.create_surface(window.clone())?;

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
        .find(|it| matches!(it, Rgba8UnormSrgb | Bgra8UnormSrgb))
        .context("could not find preferred texture format (Rgba9Unorm or Bgra8Unorm)")?;
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

#[derive(Default)]
struct App<'a> {
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'a>>,
    renderer: Option<render::PathTracer>,
    camera: camera::Camera,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
        let window_attrs = Window::default_attributes()
            .with_inner_size(window_size)
            .with_resizable(false)
            .with_title("GPU Path Tracer".to_string());

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        self.window = Some(window.clone());

        let (device, queue, surface) = pollster::block_on(connect_to_gpu(window)).unwrap();
        let renderer = render::PathTracer::new(device, queue, WIDTH, HEIGHT);

        self.camera = Camera::look_at(
            Vec3::new(0.0, 0.75, 1.0),
            Vec3::new(0.0, -0.5, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        self.surface = Some(surface);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let frame: wgpu::SurfaceTexture = self
                    .surface
                    .as_ref()
                    .unwrap()
                    .get_current_texture()
                    .expect("should be able to get current texture");

                let render_target = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                self.renderer
                    .as_mut()
                    .unwrap()
                    .render_frame(&self.camera, &render_target);

                frame.present();
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Key::Named(winit::keyboard::NamedKey::Escape) = event.logical_key {
                    event_loop.exit()
                }
            }
            _ => (),
        }
    }
    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let DeviceEvent::MouseWheel { delta } = event {
            let delta = match delta {
                MouseScrollDelta::PixelDelta(delta) => 0.001 * delta.y as f32,
                MouseScrollDelta::LineDelta(_, y) => 0.1 * y,
            };
            self.camera.zoom(delta);
            self.renderer.as_mut().unwrap().reset_samples();
        }
    }
}

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
