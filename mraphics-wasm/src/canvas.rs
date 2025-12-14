use mraphics::{Color, LogicalTimeline, PerspectiveCamera, Renderer, Scene, Timeline};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen::prelude::*;
use winit::{
    event::WindowEvent, event_loop::EventLoop, platform::web::WindowAttributesExtWebSys,
    window::Window,
};

#[wasm_bindgen]
pub struct Canvas {
    canvas_id: String,

    window: Option<Arc<Window>>,
    camera: PerspectiveCamera,
    renderer: Option<Renderer<'static>>,
    scene: Rc<RefCell<Scene>>,

    proxy: Option<winit::event_loop::EventLoopProxy<Renderer<'static>>>,

    timeline: Rc<RefCell<Box<dyn Timeline>>>,
    playhead: f32,

    clear_color: Color<f64>,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Self {
        Self {
            canvas_id: canvas_id.to_string(),

            window: None,
            camera: PerspectiveCamera::default(),
            renderer: None,
            scene: Rc::new(RefCell::new(Scene::new())),

            proxy: None,

            timeline: Rc::new(RefCell::new(Box::new(LogicalTimeline::new()))),
            playhead: 0.0,

            clear_color: mraphics::constants::GRAY_E,
        }
    }

    #[wasm_bindgen]
    pub fn run(&mut self) {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        self.proxy = Some(event_loop.create_proxy());

        event_loop.run_app(self).unwrap();
    }
}

impl winit::application::ApplicationHandler<Renderer<'static>> for Canvas {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let web_window = wgpu::web_sys::window().unwrap_throw();
        let document = web_window.document().unwrap_throw();
        let canvas_el = document.get_element_by_id(&self.canvas_id).unwrap_throw();

        self.window = Some(Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes().with_canvas(Some(canvas_el.unchecked_into())),
                )
                .unwrap(),
        ));

        // SAFETY: Initialized in Canvas::new() and the function self.resumed() only runs one time
        let proxy = self.proxy.take().unwrap();

        let window_clone = Arc::clone(self.window.as_ref().unwrap());
        wasm_bindgen_futures::spawn_local(async move {
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::GL,
                ..Default::default()
            });

            let surface = instance.create_surface(window_clone).unwrap();

            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    ..Default::default()
                })
                .await
                .unwrap();

            proxy
                .send_event(Renderer::new(surface, device, queue, &adapter))
                .ok()
                .unwrap()
        });
    }

    fn user_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        event: Renderer<'static>,
    ) {
        self.renderer = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let renderer = match &mut self.renderer {
            Some(renderer) => renderer,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.camera
                    .set_aspect(size.width as f32 / size.height as f32);

                renderer.resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                self.timeline.borrow_mut().forward();

                renderer
                    .render(
                        &mut self.scene.borrow_mut(),
                        &self.camera,
                        &self.clear_color,
                    )
                    .unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}
