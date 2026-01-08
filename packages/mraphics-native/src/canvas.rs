use mraphics_core::{
    Animation, Color, MeshHandle, MeshLike, MeshPool, PerspectiveCamera, RenderInstance, Renderer,
    Scene, Timeline,
};
use std::{cell::RefCell, marker::PhantomData, rc::Rc, sync::Arc, time::Duration};
use wgpu::{Surface, SurfaceConfiguration};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

pub struct Canvas<'res, T: Timeline<'res>> {
    pub window: Option<Arc<Window>>,
    pub camera: PerspectiveCamera,

    pub renderer: Option<Renderer>,
    surface: Option<Surface<'static>>,
    surface_config: Option<SurfaceConfiguration>,

    pub scene: Rc<RefCell<Scene>>,
    pub mesh_pool: Rc<RefCell<MeshPool>>,

    pub timeline: Rc<RefCell<T>>,
    pub playhead: f32,

    pub clear_color: Color<f64>,

    pub on_window_event:
        Box<dyn FnMut(&winit::event_loop::ActiveEventLoop, &WindowEvent, &mut PerspectiveCamera)>,

    _marker: PhantomData<&'res ()>,
}

impl<'res, T: Timeline<'res>> Canvas<'res, T> {
    pub fn new(timeline: T) -> Self {
        Self {
            window: None,
            camera: PerspectiveCamera::default(),

            renderer: None,
            surface: None,
            surface_config: None,

            scene: Rc::new(RefCell::new(Scene::new())),
            mesh_pool: Rc::new(RefCell::new(MeshPool::new())),

            timeline: Rc::new(RefCell::new(timeline)),
            playhead: 0.0,

            clear_color: Color::from_hex_str(mraphics_core::constants::GRAY_E).unwrap(),

            on_window_event: Box::new(|_, _, _| {}),

            _marker: PhantomData,
        }
    }

    pub fn add_mesh<Mesh: MeshLike + 'static>(&self, mut mesh: Mesh) -> MeshHandle<Mesh> {
        self.scene.borrow_mut().add_renderable(&mut mesh);

        self.scene
            .borrow_mut()
            .acquire_instance_mut_unchecked(mesh.identifier())
            .sync_matrix_data();

        self.mesh_pool.borrow_mut().add_mesh(mesh)
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }

    pub fn queue_animation<Ani: Animation<'res>>(&mut self, animation: Ani, duration: &Duration) {
        let mut action = animation.into_action(self.mesh_pool.clone(), self.scene.clone());
        action.duration = duration.as_secs_f32();
        action.start_time = self.playhead;

        self.playhead += action.duration;

        self.timeline.borrow_mut().add_action(action);
    }

    pub fn advance_playhead(&mut self, step: &Duration) {
        self.playhead += step.as_secs_f32();
    }

    pub fn with_instance<F: FnMut(Option<&mut RenderInstance>), Mesh: MeshLike>(
        &self,
        mesh_handle: &MeshHandle<Mesh>,
        mut closure: F,
    ) {
        closure(self.scene.borrow_mut().acquire_instance_mut(mesh_handle.id));
    }

    pub fn with_instance_unchecked<F: FnMut(&mut RenderInstance), Mesh: MeshLike>(
        &self,
        mesh_handle: &MeshHandle<Mesh>,
        mut closure: F,
    ) {
        closure(
            self.scene
                .borrow_mut()
                .acquire_instance_mut_unchecked(mesh_handle.id),
        );
    }

    pub fn with_scene_timeline_handle<F: FnMut(Rc<RefCell<Scene>>, Rc<RefCell<T>>)>(
        &self,
        mut closure: F,
    ) {
        closure(self.scene.clone(), self.timeline.clone())
    }

    fn resize(&mut self, width: u32, height: u32) {
        let surface_config = self.surface_config.as_mut().unwrap();

        surface_config.width = width;
        surface_config.height = height;

        self.surface
            .as_mut()
            .unwrap()
            .configure(&self.renderer.as_ref().unwrap().device, surface_config);
    }
}

impl<'res, T: Timeline<'res>> winit::application::ApplicationHandler for Canvas<'res, T> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("mraphics window"))
            .unwrap();

        self.window = Some(Arc::new(window));

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        self.surface = Some(
            instance
                .create_surface(Arc::clone(self.window.as_ref().unwrap()))
                .unwrap(),
        );

        pollster::block_on(async {
            let surface = self.surface.as_ref().unwrap();

            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    force_fallback_adapter: false,
                    compatible_surface: Some(surface),
                    ..Default::default()
                })
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default())
                .await
                .unwrap();

            let surface_caps = surface.get_capabilities(&adapter);
            let surface_config = wgpu::SurfaceConfiguration {
                width: 100,
                height: 100,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Rgba8Unorm,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            surface.configure(&device, &surface_config);

            self.surface_config = Some(surface_config);
            self.renderer = Some(Renderer::new(device, queue));
        });

        self.timeline.borrow_mut().start();
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

                self.resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                self.timeline.borrow_mut().forward();

                let texture = match self.surface.as_ref().unwrap().get_current_texture() {
                    Ok(texture) => texture,
                    Err(_) => {
                        // Ignores this error
                        return;
                    }
                };

                self.renderer.as_mut().unwrap().render(
                    &texture.texture,
                    self.surface_config.as_ref().unwrap().format,
                    &mut self.scene.borrow_mut().instances,
                    &self.camera,
                    &self.clear_color,
                );

                texture.present();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }

        (self.on_window_event)(event_loop, &event, &mut self.camera);
    }
}
