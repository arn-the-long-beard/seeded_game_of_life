use seed::window;
use std::cmp;

pub struct FpsCounter {
    frames: Vec<f64>,
    last_frame_timestamp: f64,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            frames: Vec::new(),
            last_frame_timestamp: window().performance().unwrap().now(), //should have it outside so it would be  more beautiful
        }
    }
    /// Ex- Render function
    /// Same as original in JS except I moved most of call to DOM outside to make it "cleaner"
    /// Maybe I could also have passed time as an argument to make it even better
    /// If we removed calls to window() we could make unit test and benchmark
    pub fn calculate(&mut self) -> FpsStatistic {
        let now = window().performance().unwrap().now();
        let delta = now - self.last_frame_timestamp;
        self.last_frame_timestamp = now;

        let fps = 1. / delta * 1000.;

        self.frames.push(fps);

        if self.frames.len() > 100 {
            self.frames.remove(0);
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        let mut sum: f64 = 0.;

        for i in 0..self.frames.len() {
            sum = sum + self.frames[i] as f64;

            min = cmp::min(self.frames[i] as i32, min);

            max = cmp::max(self.frames[i] as i32, max);
        }
        let mean = sum / self.frames.len() as f64;

        FpsStatistic {
            fps: fps as u32,
            mean: mean as u32,
            min,
            max,
        }
    }
}

pub struct FpsStatistic {
    pub fps: u32,
    pub mean: u32,
    pub min: i32,
    pub max: i32,
}
