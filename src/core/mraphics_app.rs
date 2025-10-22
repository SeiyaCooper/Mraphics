use crate::{Scene, render::Renderer};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

pub struct MraphicsApp<'window> {
    pub scene: Scene,

    renderer: Option<Renderer<'window>>,
}

impl<'window> ApplicationHandler for MraphicsApp<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        pollster::block_on((async || {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default())
                .await
                .unwrap();

            self.renderer = Some(Renderer::new(
                surface,
                device,
                queue,
                include_str!("D:/WorkSpace/Mraphics/src/render/shaders/shader.wgsl"),
                &adapter,
            ));
        })());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.renderer
                    .as_mut()
                    .unwrap()
                    .resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                self.renderer.as_ref().unwrap().render().unwrap();
            }
            _ => {}
        }
    }
}

impl<'window> MraphicsApp<'window> {
    pub fn new() -> Self {
        Self {
            renderer: None,
            scene: Scene::new(),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(self).unwrap();
    }
}
