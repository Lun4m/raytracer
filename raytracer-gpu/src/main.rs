use {
    algebra::Vec3,
    anyhow::{Context, Result},
    camera::Camera,
    std::{f32::consts::FRAC_PI_2, sync::Arc},
    winit::{
        application::ApplicationHandler,
        event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        keyboard::{KeyCode, PhysicalKey},
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
    left_mouse_button_pressed: bool,
    right_mouse_button_pressed: bool,
}

impl<'a> App<'a> {
    pub fn new(camera: camera::Camera) -> App<'a> {
        App {
            window: None,
            surface: None,
            renderer: None,
            camera,
            left_mouse_button_pressed: false,
            right_mouse_button_pressed: false,
        }
    }

    fn switch_pan_orbit(&mut self, dx: f32, dy: f32) {
        if self.left_mouse_button_pressed {
            self.camera.orbit(dx, dy);
            self.renderer.as_mut().unwrap().reset_samples();
        }
        if self.right_mouse_button_pressed {
            self.camera.pan(dx, dy);
            self.renderer.as_mut().unwrap().reset_samples();
        }
    }
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
                // TODO: these should not reset zoom
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => event_loop.exit(),
                    PhysicalKey::Code(KeyCode::KeyW) => {
                        self.switch_pan_orbit(0.0, 0.01);
                    }
                    PhysicalKey::Code(KeyCode::KeyS) => {
                        self.switch_pan_orbit(0.0, -0.01);
                    }
                    PhysicalKey::Code(KeyCode::KeyD) => {
                        self.switch_pan_orbit(0.01, 0.0);
                    }
                    PhysicalKey::Code(KeyCode::KeyA) => {
                        self.switch_pan_orbit(-0.01, 0.0);
                    }
                    _ => (),
                };
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    MouseScrollDelta::PixelDelta(delta) => 0.001 * delta.y as f32,
                    MouseScrollDelta::LineDelta(_, y) => 0.1 * y,
                };
                self.camera.zoom(delta);
                self.renderer.as_mut().unwrap().reset_samples();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => self.left_mouse_button_pressed = pressed,
                    MouseButton::Right => self.right_mouse_button_pressed = pressed,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    // NOTE: This is broken on WSL, deltas are not relative to the window
    // The workaround is to compile targeting windows
    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            let dx = dx as f32 * 0.001;
            let dy = dy as f32 * -0.001;

            if self.left_mouse_button_pressed {
                self.camera.orbit(dx, dy);
                self.renderer.as_mut().unwrap().reset_samples();
            }

            if self.right_mouse_button_pressed {
                self.camera.pan(dx, dy);
                self.renderer.as_mut().unwrap().reset_samples();
            }
        }
    }
}

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let camera = Camera::with_spherical_coords(
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        2.0,
        FRAC_PI_2,
        0.0,
    );
    println!("{:?}", camera);

    let mut app = App::new(camera);
    event_loop.run_app(&mut app)?;

    Ok(())
}
