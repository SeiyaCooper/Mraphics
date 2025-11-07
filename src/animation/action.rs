pub struct Action {
    pub start_time: f32,
    pub duration: f32,

    pub on_start: Box<dyn FnMut()>,
    pub on_stop: Box<dyn FnMut()>,
    pub on_update: Box<dyn FnMut(f32, f32)>,
}

impl Action {
    pub fn new() -> Self {
        Self {
            start_time: 0.0,
            duration: 1.0,
            on_start: Box::new(|| {}),
            on_stop: Box::new(|| {}),
            on_update: Box::new(|_, _| {}),
        }
    }

    pub fn execute(&mut self, progress: f32, elapsed_time: f32) {
        (self.on_update)(progress, elapsed_time);
    }
}
