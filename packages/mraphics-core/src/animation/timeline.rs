use crate::animation::Action;
use std::time::Instant;

#[derive(Debug)]
pub enum TimelineState {
    PLAYING,
    PAUSED,
    WAITING,
}

pub trait Timeline<'res> {
    fn current_time(&self) -> f32;
    fn seek(&mut self, time: f32);
    fn state(&self) -> &TimelineState;

    fn start(&mut self);
    fn forward(&mut self);
    fn pause(&mut self);

    fn actions(&self) -> &Vec<Action<'res>>;
    fn actions_mut(&mut self) -> &mut Vec<Action<'res>>;
    fn add_action(&mut self, action: Action<'res>);

    fn all_stopped(&self) -> bool {
        for action in self.actions() {
            if !action.stopped {
                return false;
            }
        }

        true
    }

    fn process(&mut self) {
        let current_time = self.current_time();
        for action in self.actions_mut() {
            let elapsed = current_time - action.start_time;
            let progress = elapsed / action.duration;
            action.execute(progress, elapsed);
        }
    }
}

pub struct LogicalTimeline<'res> {
    pub state: TimelineState,
    pub logical_fps: f32,

    current_frame: i32,

    actions: Vec<Action<'res>>,
}

impl<'res> LogicalTimeline<'res> {
    pub fn new() -> Self {
        Self {
            state: TimelineState::WAITING,
            logical_fps: 60.0,
            current_frame: 0,
            actions: Vec::new(),
        }
    }
}

impl<'res> Timeline<'res> for LogicalTimeline<'res> {
    fn current_time(&self) -> f32 {
        (self.current_frame as f32) * (1.0 / self.logical_fps)
    }

    fn seek(&mut self, time: f32) {
        self.current_frame = (self.logical_fps * time) as i32;
    }

    fn state(&self) -> &TimelineState {
        &self.state
    }

    fn start(&mut self) {
        self.state = TimelineState::PLAYING;
        self.process();
    }

    fn forward(&mut self) {
        match self.state {
            TimelineState::PLAYING => {
                self.current_frame += 1;
                self.process();
            }
            _ => {}
        }
    }

    fn pause(&mut self) {
        self.state = TimelineState::PAUSED;
    }

    fn actions(&self) -> &Vec<Action<'res>> {
        &self.actions
    }

    fn actions_mut(&mut self) -> &mut Vec<Action<'res>> {
        &mut self.actions
    }

    fn add_action(&mut self, action: Action<'res>) {
        self.actions.push(action);
    }
}

pub struct PhysicalTimeline<'res> {
    pub state: TimelineState,
    pub current_time: f32,

    start_instant: Instant,
    time_at_pause: f32,

    actions: Vec<Action<'res>>,
}

impl<'res> PhysicalTimeline<'res> {
    pub fn new() -> Self {
        Self {
            state: TimelineState::WAITING,
            current_time: 0.0,
            actions: Vec::new(),
            start_instant: Instant::now(),
            time_at_pause: 0.0,
        }
    }
}

impl<'res> Timeline<'res> for PhysicalTimeline<'res> {
    fn current_time(&self) -> f32 {
        self.current_time
    }

    fn seek(&mut self, time: f32) {
        self.current_time = time;
    }

    fn state(&self) -> &TimelineState {
        &self.state
    }

    fn start(&mut self) {
        match self.state {
            TimelineState::WAITING => self.time_at_pause = 0.0,
            TimelineState::PAUSED => {
                self.time_at_pause = self.current_time;
            }
            TimelineState::PLAYING => {}
        }

        self.start_instant = Instant::now();
        self.state = TimelineState::PLAYING;
        self.process();
    }

    fn forward(&mut self) {
        match self.state {
            TimelineState::PLAYING => {
                self.current_time = self.time_at_pause + self.start_instant.elapsed().as_secs_f32();
                self.process();
            }
            _ => {}
        }
    }

    fn pause(&mut self) {
        self.state = TimelineState::PAUSED;
    }

    fn actions(&self) -> &Vec<Action<'res>> {
        &self.actions
    }

    fn actions_mut(&mut self) -> &mut Vec<Action<'res>> {
        &mut self.actions
    }

    fn add_action(&mut self, action: Action<'res>) {
        self.actions.push(action);
    }
}
