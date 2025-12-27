pub struct Action<'res> {
    pub start_time: f32,
    pub duration: f32,

    pub on_start: Box<dyn FnMut() + 'res>,
    pub on_stop: Box<dyn FnMut() + 'res>,
    pub on_execute: Box<dyn FnMut() + 'res>,
    pub on_update: Box<dyn FnMut(f32, f32) + 'res>,

    started: bool,
    stopped: bool,
}

impl<'res> Action<'res> {
    pub fn new() -> Self {
        Self {
            start_time: 0.0,
            duration: 1.0,
            on_start: Box::new(|| {}),
            on_stop: Box::new(|| {}),
            on_execute: Box::new(|| {}),
            on_update: Box::new(|_, _| {}),

            started: false,
            stopped: false,
        }
    }

    pub fn execute(&mut self, progress: f32, elapsed_time: f32) {
        (self.on_execute)();

        if self.stopped || progress < 0.0 {
            return;
        }

        if !self.started {
            (self.on_start)();
            self.started = true;
        }

        if progress == 0.0 {
            return;
        }

        if progress > 1.0 {
            if !self.stopped {
                (self.on_update)(1.0, self.duration);
                (self.on_stop)();
                self.stopped = true;
            }

            return;
        }

        (self.on_update)(progress, elapsed_time);
    }

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }
}
