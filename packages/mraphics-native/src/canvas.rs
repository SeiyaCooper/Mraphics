use mraphics_core::{
    Animation, Color, MeshHandle, MeshLike, MeshPool, PerspectiveCamera, RenderInstance, Renderer,
    Scene, Timeline,
};
use std::{cell::RefCell, marker::PhantomData, rc::Rc, sync::Arc, time::Duration};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

pub struct Canvas<'res, T: Timeline<'res>> {
    pub window: Option<Arc<Window>>,
    pub camera: PerspectiveCamera,
    pub renderer: Option<Renderer<'static>>,

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
                    .render(
                        &mut self.scene.borrow_mut().instances,
                        &self.camera,
                        &self.clear_color,
                    )
                    .unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }

        (self.on_window_event)(event_loop, &event, &mut self.camera);
    }
}
