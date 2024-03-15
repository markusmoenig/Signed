use std::time::{Duration, Instant};
use theframework::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum CameraMode {
    Pinhole,
    Orthogonal,
}

/// Ray
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Ray {
    pub o: Vec3d,
    pub d: Vec3d,

    pub inv_direction: Vec3d,

    pub sign_x: usize,
    pub sign_y: usize,
    pub sign_z: usize,
}

impl Ray {
    pub fn new(o: Vec3d, d: Vec3d) -> Self {
        Self {
            o,
            d,

            inv_direction: Vec3d::new(1.0 / d.x, 1.0 / d.y, 1.0 / d.z),
            sign_x: (d.x < 0.0) as usize,
            sign_y: (d.y < 0.0) as usize,
            sign_z: (d.z < 0.0) as usize,
        }
    }

    /// Returns the position on the ray at the given distance
    pub fn at(&self, d: f64) -> Vec3d {
        self.o + self.d * d
    }
}
pub struct UpdateTracker {
    //update_counter: u32,
    //last_fps_check: Instant,
    last_redraw_update: Instant,
    last_tick_update: Instant,
}

impl Default for UpdateTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdateTracker {
    pub fn new() -> Self {
        UpdateTracker {
            //update_counter: 0,
            //last_fps_check: Instant::now(),
            last_redraw_update: Instant::now(),
            last_tick_update: Instant::now(),
        }
    }

    pub fn update(&mut self, tick_ms: u64) -> bool {
        let mut tick_update = false;

        // self.update_counter += 1;

        // if self.last_fps_check.elapsed() >= Duration::from_secs(1) {
        //     self.calculate_and_reset_fps();
        // }

        if self.last_tick_update.elapsed() >= Duration::from_millis(tick_ms) {
            self.last_tick_update = Instant::now();
            tick_update = true;
        }

        tick_update
    }

    // fn calculate_and_reset_fps(&mut self) {
    //     //let fps = self.update_counter;
    //     self.update_counter = 0;
    //     self.last_fps_check = Instant::now();
    //     //println!("FPS: {}", fps);
    // }
}
