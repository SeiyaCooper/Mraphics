use std::f32::consts::PI;

use mraphics::math::Camera;
use nalgebra::{Point3, Rotation3, Vector3};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};

#[derive(Debug)]
pub enum OrbitControlState {
    Wait,
    Rotate([f64; 2]),
    Zoom,
    Move,
}

struct MouseState {
    position: [f64; 2],
}

pub struct OrbitControl {
    pub state: OrbitControlState,
    mouse_state: MouseState,

    pub center: Point3<f32>,
    target_center: Point3<f32>,

    pub enable_zoom: bool,
    pub radius: f32,
    pub scale: f32,
    pub zoom_speed: f32,

    pub enable_rotate: bool,
    pub theta: f32,
    pub phi: f32,
    pub phi_max: f32,
    pub phi_min: f32,
    pub delta_angle_max: f32,
    pub rotate_speed: f32,
    pub rotate_ease: f32,
    start_phi: f32,
    start_theta: f32,
    target_phi: f32,
    target_theta: f32,

    pub enable_move: bool,
    pub move_speed: f32,
    pub move_ease: f32,
}

impl Default for OrbitControl {
    fn default() -> Self {
        Self {
            state: OrbitControlState::Wait,
            mouse_state: MouseState {
                position: [0.0, 0.0],
            },

            center: Point3::origin(),
            target_center: Point3::origin(),

            enable_zoom: true,
            radius: 5.0,
            scale: 1.0,
            zoom_speed: 1.1,

            enable_rotate: true,
            theta: 0.0,
            phi: 0.0,
            phi_max: PI / 2.0,
            phi_min: -PI / 2.0,
            delta_angle_max: 0.1,
            rotate_speed: 0.01,
            rotate_ease: 0.01,
            start_phi: 0.0,
            start_theta: 0.0,
            target_phi: 0.0,
            target_theta: 0.0,

            enable_move: true,
            move_speed: 0.001,
            move_ease: 0.15,
        }
    }
}

impl OrbitControl {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update<C: Camera>(&mut self, camera: &mut C) {
        let delta_phi = ((self.target_phi - self.phi) * self.rotate_ease).min(self.delta_angle_max);
        let delta_theta =
            ((self.target_theta - self.theta) * self.rotate_ease).min(self.delta_angle_max);

        self.phi = (self.phi + delta_phi).min(self.phi_max).max(self.phi_min);
        self.theta += delta_theta;

        let rotation = Rotation3::from_euler_angles(self.phi, -self.theta, 0.0);

        camera.set_center(
            &(rotation.transform_vector(&Vector3::new(0.0, 0.0, self.radius * self.scale))
                + &self.center.coords),
        );

        camera.look_at(&self.center);
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Released => self.state = OrbitControlState::Wait,
                ElementState::Pressed => match button {
                    MouseButton::Left => {
                        self.start_phi = self.phi;
                        self.start_theta = self.theta;
                        self.state = OrbitControlState::Rotate(self.mouse_state.position);
                    }
                    _ => {}
                },
            },
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, y) => {
                    self.on_mouse_wheel(*y > 0.0);
                }
                MouseScrollDelta::PixelDelta(pos) => {
                    self.on_mouse_wheel(pos.y > 0.0);
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_state.position = [position.x, position.y];
                self.on_mouse_move([position.x, position.y]);
            }
            _ => {}
        }
    }

    pub fn rotate(&mut self, delta_phi: f32, delta_theta: f32) {
        self.target_phi = delta_phi + self.start_phi;
        self.target_theta = delta_theta + self.start_theta;
    }

    pub fn zoom_to(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn on_mouse_move(&mut self, pos: [f64; 2]) {
        if let OrbitControlState::Rotate(start_pos) = self.state {
            let delta_phi = self.rotate_speed * (pos[1] - start_pos[1]) as f32;
            let delta_theta = self.rotate_speed * (pos[0] - start_pos[0]) as f32;
            self.rotate(delta_phi, delta_theta);
        }
    }

    fn on_mouse_wheel(&mut self, positive: bool) {
        if positive {
            self.zoom_to(self.scale / self.zoom_speed);
        } else {
            self.zoom_to(self.scale * self.zoom_speed);
        }
    }
}
