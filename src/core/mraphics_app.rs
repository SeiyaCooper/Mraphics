use crate::{
    Scene,
    animation::{LogicalTimeline, Timeline},
    math::PerspectiveCamera,
    render::Renderer,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
    rc::Rc,
    sync::Arc,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{self, Window},
};

pub struct MraphicsAppHandle {
    app: Rc<RefCell<MraphicsApp>>,
}

impl MraphicsAppHandle {
    pub fn new() -> Self {
        Self {
            app: Rc::new(RefCell::new(MraphicsApp::new())),
        }
    }

    pub fn with_handle_clone<F: FnMut(MraphicsAppHandle)>(&self, mut closure: F) {
        closure(self.clone());
    }
}

impl Clone for MraphicsAppHandle {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
        }
    }
}

impl Deref for MraphicsAppHandle {
    type Target = RefCell<MraphicsApp>;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

pub struct MraphicsApp {
    pub scene: RefCell<Scene>,
    pub camera: RefCell<PerspectiveCamera>,
    pub renderer: RefCell<Option<Renderer<'static>>>,
}

impl MraphicsApp {
    pub fn new() -> Self {
        Self {
            camera: RefCell::new(PerspectiveCamera::default()),
            renderer: RefCell::new(None),
            scene: RefCell::new(Scene::new()),
        }
    }

    pub fn scene_mut(&self) -> RefMut<'_, Scene> {
        self.scene.borrow_mut()
    }

    pub fn scene(&self) -> Ref<'_, Scene> {
        self.scene.borrow()
    }

    pub fn camera_mut(&self) -> RefMut<'_, PerspectiveCamera> {
        self.camera.borrow_mut()
    }

    pub fn camera(&self) -> Ref<'_, PerspectiveCamera> {
        self.camera.borrow()
    }

    fn resumed(&mut self, window: Arc<Window>) {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

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

            self.renderer = RefCell::new(Some(Renderer::new(surface, device, queue, &adapter)));
        });
    }
}

pub struct WindowHandler {
    pub window: Option<Arc<Window>>,
    pub app_handle: MraphicsAppHandle,

    pub timeline: RefCell<Box<dyn Timeline>>,
}

impl WindowHandler {
    pub fn new(app_handle: MraphicsAppHandle) -> Self {
        Self {
            window: None,
            app_handle,
            timeline: RefCell::new(Box::new(LogicalTimeline::new())),
        }
    }
}

impl WindowHandler {
    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(self).unwrap();
    }
}

impl ApplicationHandler for WindowHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.window = Some(Arc::new(window));
        self.app_handle
            .borrow_mut()
            .resumed(Arc::clone(self.window.as_ref().unwrap()));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.app_handle
                    .borrow()
                    .camera
                    .borrow_mut()
                    .set_aspect(size.width as f32 / size.height as f32);

                self.app_handle
                    .borrow()
                    .renderer
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                self.timeline.borrow_mut().forward();

                self.app_handle
                    .borrow()
                    .renderer
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .render(
                        &mut (*self.app_handle.borrow().scene.borrow_mut()),
                        &(*self.app_handle.borrow().camera.borrow()),
                    )
                    .unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}
