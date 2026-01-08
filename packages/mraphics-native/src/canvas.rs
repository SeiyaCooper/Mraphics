use image::ImageBuffer;
use mraphics_core::{
    Animation, Camera, Color, MeshHandle, MeshLike, MeshPool, RenderInstance, Renderer, Scene,
    Timeline,
};
use std::{cell::RefCell, marker::PhantomData, rc::Rc, sync::Arc, time::Duration};
use wgpu::{Surface, SurfaceConfiguration, Texture, TextureFormat};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

struct WindowContext {
    pub window: Arc<Window>,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
}

const OFFSCREEN_TEXTURE_FORMAT: TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

pub struct Canvas<'res, T: Timeline<'res>, C: Camera> {
    window_ctx: Option<WindowContext>,

    pub size: (u32, u32),

    pub(crate) renderer: Option<Renderer>,

    pub(crate) offscreen_texture: Option<Texture>,

    pub camera: C,

    pub on_window_event: Box<dyn FnMut(&winit::event_loop::ActiveEventLoop, &WindowEvent, &mut C)>,

    pub timeline: T,
    pub playhead: f32,

    /// The underlying scene for this canvas.
    /// Wrapped in [`Rc`] to enable shared ownership across timeline actions.
    pub scene: Rc<RefCell<Scene>>,

    /// Manages all meshes of this canvas.
    /// Wrapped in [`Rc`] to enable shared ownership across timeline actions.
    pub mesh_pool: Rc<RefCell<MeshPool>>,

    pub clear_color: Color<f64>,

    _marker: PhantomData<&'res ()>,
}

impl<'res, T: Timeline<'res>, C: Camera> Canvas<'res, T, C> {
    pub fn new(timeline: T, mut camera: C) -> Self {
        camera.set_aspect(1920.0 / 1080.0);
        Self {
            window_ctx: None,

            size: (1920, 1080),

            renderer: None,

            offscreen_texture: None,

            camera,

            on_window_event: Box::new(|_, _, _| {}),

            timeline,
            playhead: 0.0,

            scene: Rc::new(RefCell::new(Scene::new())),
            mesh_pool: Rc::new(RefCell::new(MeshPool::new())),

            clear_color: Color::from_hex_str(mraphics_core::constants::GRAY_E).unwrap(),

            _marker: PhantomData,
        }
    }

    pub fn queue_animation<Ani: Animation<'res>>(&mut self, animation: Ani, duration: &Duration) {
        let mut action = animation.into_action(self.mesh_pool.clone(), self.scene.clone());
        action.duration = duration.as_secs_f32();
        action.start_time = self.playhead;

        self.playhead += action.duration;

        self.timeline.add_action(action);
    }

    pub fn advance_playhead(&mut self, step: &Duration) {
        self.playhead += step.as_secs_f32();
    }

    pub fn add_mesh<Mesh: MeshLike + 'static>(&mut self, mut mesh: Mesh) -> MeshHandle<Mesh> {
        self.scene.borrow_mut().add_renderable(&mut mesh);

        self.scene
            .borrow_mut()
            .acquire_instance_mut_unchecked(mesh.identifier())
            .sync_matrix_data();

        self.mesh_pool.borrow_mut().add_mesh(mesh)
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

    pub fn resize(&mut self, size: (u32, u32)) {
        self.size = size;

        self.camera.set_aspect(size.0 as f32 / size.1 as f32);

        if self.window_ctx.is_some() {
            self.resize_surface(size.0, size.1);
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }

    pub fn snapshot(&mut self, time: f32, path: &str) {
        self.timeline.seek(time);
        self.timeline.process();

        self.render_offscreen();

        let raw_image = self
            .renderer
            .as_ref()
            .unwrap()
            .read_texture_rgbau8(self.offscreen_texture.as_mut().unwrap(), self.size);

        ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(self.size.0, self.size.1, raw_image)
            .unwrap()
            .save(path)
            .unwrap();
    }

    pub fn render_offscreen(&mut self) {
        self.prepare_offscreen_rendering();
        self.renderer.as_mut().unwrap().render(
            self.offscreen_texture.as_mut().unwrap(),
            OFFSCREEN_TEXTURE_FORMAT,
            &mut self.scene.borrow_mut().instances,
            &self.camera,
            &self.clear_color,
        );
    }

    fn prepare_offscreen_rendering(&mut self) {
        if self.offscreen_texture.is_some() {
            return;
        }

        if self.renderer.is_none() {
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            });

            pollster::block_on(async {
                let adapter = instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        force_fallback_adapter: false,
                        ..Default::default()
                    })
                    .await
                    .unwrap();

                let (device, queue) = adapter
                    .request_device(&wgpu::DeviceDescriptor::default())
                    .await
                    .unwrap();

                self.renderer = Some(Renderer::new(device, queue));
            })
        }

        self.offscreen_texture = Some(self.renderer.as_ref().unwrap().device.create_texture(
            &wgpu::wgt::TextureDescriptor {
                label: Some("Offline texture"),
                size: wgpu::Extent3d {
                    width: self.size.0,
                    height: self.size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: OFFSCREEN_TEXTURE_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            },
        ));
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

impl<'res, T: Timeline<'res>, C: Camera> winit::application::ApplicationHandler
    for Canvas<'res, T, C>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("mraphics window"))
                .unwrap(),
        );

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

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

            let surface_caps = surface.get_capabilities(&adapter);
            let surface_config = wgpu::SurfaceConfiguration {
                width: self.size.0,
                height: self.size.1,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Rgba8Unorm,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            surface.configure(&device, &surface_config);

            self.window_ctx = Some(WindowContext {
                window,
                surface,
                surface_config,
            });
            self.renderer = Some(Renderer::new(device, queue));
        });

        self.timeline.start();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
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

                self.timeline.forward();

                let texture = match window_ctx.surface.get_current_texture() {
                    Ok(texture) => texture,
                    Err(_) => {
                        // ignores this error
                        return;
                    }
                };

                self.renderer.as_mut().unwrap().render(
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
