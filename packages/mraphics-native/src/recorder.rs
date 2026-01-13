use crate::Canvas;
use mraphics_core::{Action, Camera, LogicalTimeline, Timeline};
use std::{
    cell::Cell,
    io::Write,
    process::{Command, Stdio},
    rc::Rc,
};

pub struct Recorder<'canvas> {
    pub timeline: LogicalTimeline<'canvas>,

    pub output_path: String,
}

impl<'canvas> Recorder<'canvas> {
    pub fn new() -> Self {
        Self {
            timeline: LogicalTimeline::new(),

            output_path: String::from("video.mp4"),
        }
    }

    pub fn with_fps(mut self, fps: f32) -> Self {
        self.timeline.logical_fps = fps;
        self
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.timeline.logical_fps = fps;
    }

    pub fn record<'res, T: Timeline<'res>, C: Camera>(
        &mut self,
        canvas: &'canvas mut Canvas<'res, T, C>,
    ) {
        let canvas_time_bak = canvas.timeline.current_time();

        let is_recording = Rc::new(Cell::new(true));
        let is_recording_clone = is_recording.clone();

        let mut command = Command::new("ffmpeg");

        #[rustfmt::skip]
        command.stdin(Stdio::piped()).args([
            "-y",
            "-f", "rawvideo",
            "-s", &format!("{}x{}", canvas.size.0, canvas.size.1),
            "-pix_fmt", "rgba",
            "-r", &self.timeline.logical_fps.to_string(),
            "-i", "-",
            "-vcodec", "libx264",
            &self.output_path,
        ]);

        let mut process = command.spawn().expect("Failed to invoke FFmpeg");

        let playhead = Rc::new(Cell::new(0.0));
        let playhead_clone = playhead.clone();

        let mut record_action = Action::new();
        record_action.on_execute = Box::new(move || {
            if canvas.timeline.all_stopped() {
                process.stdin.as_ref().unwrap().flush().unwrap();

                process.wait().unwrap();

                is_recording_clone.replace(false);

                canvas.timeline.seek(canvas_time_bak);

                return;
            }

            canvas.timeline.seek(playhead_clone.get());
            canvas.timeline.process();
            canvas.render_offscreen();

            let raw_img = canvas
                .renderer
                .as_ref()
                .unwrap()
                .read_texture_rgbau8(canvas.offscreen_texture.as_ref().unwrap(), canvas.size);

            process.stdin.as_mut().unwrap().write_all(&raw_img).unwrap();
        });

        self.timeline.add_action(record_action);
        self.timeline.start();

        while is_recording.get() {
            playhead.replace(self.timeline.current_time());
            self.timeline.forward();
        }
    }
}
