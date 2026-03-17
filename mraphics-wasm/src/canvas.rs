use mraphics::{
    Camera, Color, LogicalTimeline, MeshPool, MraphicsID, OrbitControl, PerspectiveCamera,
    Renderer, Scene, Timeline,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};
use wasm_bindgen::prelude::*;
use wgpu::{Surface, SurfaceConfiguration};
use winit::{
    event::WindowEvent, event_loop::EventLoop, platform::web::WindowAttributesExtWebSys,
    window::Window,
};

struct WindowContext {
    pub window: Arc<Window>,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
}

#[wasm_bindgen]
pub struct Canvas {
    canvas_id: String,

    window_ctx: Option<WindowContext>,

    size: (u32, u32),

    #[wasm_bindgen(skip)]
    pub camera: PerspectiveCamera,
    #[wasm_bindgen(skip)]
    pub renderer: Option<Renderer>,

    #[wasm_bindgen(skip)]
    pub scene: Rc<RefCell<Scene>>,
    #[wasm_bindgen(skip)]
    pub mesh_pool: Rc<RefCell<MeshPool>>,

    #[wasm_bindgen(skip)]
    pub update_flags: HashMap<MraphicsID, bool>,

    proxy: Option<winit::event_loop::EventLoopProxy<(WindowContext, Renderer)>>,

    #[wasm_bindgen(skip)]
    pub timeline: Rc<RefCell<Box<dyn Timeline<'static>>>>,

    pub playhead: f32,

    clear_color: Color<f64>,

    on_window_event:
        Box<dyn FnMut(&winit::event_loop::ActiveEventLoop, &WindowEvent, &mut PerspectiveCamera)>,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Self {
        Self {
            canvas_id: canvas_id.to_string(),

            window_ctx: None,

            size: (960, 540),

            camera: PerspectiveCamera::default(),
            renderer: None,
            scene: Rc::new(RefCell::new(Scene::new())),
            mesh_pool: Rc::new(RefCell::new(MeshPool::new())),

            update_flags: HashMap::new(),

            proxy: None,

            timeline: Rc::new(RefCell::new(Box::new(LogicalTimeline::new()))),
            playhead: 0.0,

            clear_color: Color::from_hex_str(mraphics::constants::GRAY_E).unwrap(),

            on_window_event: Box::new(|_, _, _| {}),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        self.proxy = Some(event_loop.create_proxy());

        event_loop.run_app(self).unwrap();
    }

    #[wasm_bindgen(js_name = "enableOrbitControl")]
    pub fn enable_orbit_control(&mut self) {
        let mut controller = OrbitControl::new();
        self.on_window_event = Box::new(move |_, event, camera| {
            controller.handle_window_event(event);
            controller.update(camera);
        });
    }

    fn resize_surface(&mut self, width: u32, height: u32) {
        let mut window_ctx = self.window_ctx.take().unwrap();
        let surface_config = &mut window_ctx.surface_config;

        surface_config.width = width;
        surface_config.height = height;

        window_ctx
            .surface
            .configure(&self.renderer.as_ref().unwrap().device, surface_config);

        self.window_ctx = Some(window_ctx);

        self.size = (width, height);
    }
}

impl winit::application::ApplicationHandler<(WindowContext, Renderer)> for Canvas {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let web_window = wgpu::web_sys::window().unwrap_throw();
        let document = web_window.document().unwrap_throw();
        let canvas_el = document.get_element_by_id(&self.canvas_id).unwrap_throw();

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes().with_canvas(Some(canvas_el.unchecked_into())),
                )
                .unwrap(),
        );

        let size = self.size;

        // SAFETY: Initialized in Canvas::new() and the function self.resumed() only runs one time
        let proxy = self.proxy.take().unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::BROWSER_WEBGPU,
                ..Default::default()
            });

            let surface = instance.create_surface(Arc::clone(&window)).unwrap();

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

            let surface_caps = surface.get_capabilities(&adapter);
            let surface_config = wgpu::SurfaceConfiguration {
                width: size.0,
                height: size.1,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Rgba8Unorm,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            surface.configure(&device, &surface_config);

            let window_ctx = WindowContext {
                window,
                surface,
                surface_config,
            };

            proxy
                .send_event((window_ctx, Renderer::new(device, queue)))
                .ok()
                .unwrap()
        });
    }

    fn user_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        event: (WindowContext, Renderer),
    ) {
        self.window_ctx = Some(event.0);
        self.renderer = Some(event.1);
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
                self.resize_surface(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                let window_ctx = self.window_ctx.take().unwrap();

                self.timeline.borrow_mut().forward();

                let texture = match window_ctx.surface.get_current_texture() {
                    Ok(texture) => texture,
                    Err(_) => {
                        // ignores this error
                        return;
                    }
                };

                renderer.render(
                    &texture.texture,
                    window_ctx.surface_config.format,
                    &mut self.scene.borrow_mut().instances,
                    &self.camera,
                    &self.clear_color,
                );

                texture.present();

                window_ctx.window.request_redraw();

                self.window_ctx = Some(window_ctx);
            }
            _ => {}
        }

        (self.on_window_event)(event_loop, &event, &mut self.camera);
    }
}
