use crate::{
    Scene,
    animation::{LogicalTimeline, Timeline},
    geometry::Mesh,
    math::PerspectiveCamera,
    render::Renderer,
};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

pub struct Canvas {
    pub window: Option<Arc<Window>>,
    pub camera: PerspectiveCamera,
    pub renderer: Option<Renderer<'static>>,
    pub scene: Rc<RefCell<Scene>>,
    pub timeline: Rc<RefCell<Box<dyn Timeline>>>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            window: None,
            camera: PerspectiveCamera::default(),
            renderer: None,
            scene: Rc::new(RefCell::new(Scene::new())),
            timeline: Rc::new(RefCell::new(Box::new(LogicalTimeline::new()))),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(self).unwrap();
    }

    pub fn add_mesh(&self, mesh: Mesh) -> usize {
        self.scene.borrow_mut().add_mesh(mesh)
    }

    pub fn with_scene_timeline_handle<
        F: FnMut(Rc<RefCell<Scene>>, Rc<RefCell<Box<dyn Timeline>>>),
    >(
        &self,
        mut closure: F,
    ) {
        closure(self.scene.clone(), self.timeline.clone())
    }
}

impl winit::application::ApplicationHandler for Canvas {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.window = Some(Arc::new(window));

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance
            .create_surface(Arc::clone(self.window.as_ref().unwrap()))
            .unwrap();

        pollster::block_on(async {
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

            self.renderer = Some(Renderer::new(surface, device, queue, &adapter));
        });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.camera
                    .set_aspect(size.width as f32 / size.height as f32);

                self.renderer
                    .as_mut()
                    .unwrap()
                    .resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                self.timeline.borrow_mut().forward();

                self.renderer
                    .as_mut()
                    .unwrap()
                    .render(&mut self.scene.borrow_mut(), &self.camera)
                    .unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}
