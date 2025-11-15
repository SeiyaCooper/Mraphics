use crate::animation::Action;

#[derive(Debug)]
pub enum TimelineState {
    PLAYING,
    PAUSED,
    WAITING,
}

pub trait Timeline {
    fn start_time(&self) -> f32;
    fn stop_time(&self) -> f32;
    fn current_time(&self) -> f32;
    fn state(&self) -> &TimelineState;

    fn start(&mut self);
    fn forward(&mut self);
    fn pause(&mut self);

    fn actions(&self) -> &Vec<Action>;
    fn add_action(&mut self, action: Action);
}

pub struct LogicalTimeline {
    pub state: TimelineState,
    pub start_time: f32,
    pub stop_time: f32,
    pub logical_fps: f32,
    pub current_frame: i32,

    actions: Vec<Action>,
}

impl LogicalTimeline {
    pub fn new() -> Self {
        Self {
            state: TimelineState::WAITING,
            start_time: 0.0,
            stop_time: 0.0,
            logical_fps: 60.0,
            current_frame: 0,
            actions: Vec::new(),
        }
    }

    fn process(&mut self) {
        let current_time = self.current_time();
        for action in &mut self.actions {
            let elapsed = current_time - action.start_time;
            let progress = elapsed / action.duration;
            action.execute(progress, elapsed);
        }
    }
}

impl Timeline for LogicalTimeline {
    fn current_time(&self) -> f32 {
        (self.current_frame as f32) * (1.0 / self.logical_fps)
    }

    fn start_time(&self) -> f32 {
        self.start_time
    }

    fn stop_time(&self) -> f32 {
        self.stop_time
    }

    fn state(&self) -> &TimelineState {
        &self.state
    }

    fn start(&mut self) {
        self.state = TimelineState::PLAYING;
        self.process();
    }

    fn forward(&mut self) {
        self.current_frame += 1;
        self.process();
    }

    fn pause(&mut self) {
        self.state = TimelineState::PAUSED;
    }

    fn actions(&self) -> &Vec<Action> {
        &self.actions
    }

    fn add_action(&mut self, action: Action) {
        if action.duration + action.start_time > self.stop_time {
            self.stop_time = action.duration + action.start_time;
        }

        self.actions.push(action);
    }
}

pub struct PhysicalTimeline {}
