use theframework::*;

pub mod camera;
pub mod editor;
pub mod misc;
pub mod object;
pub mod panel;
pub mod point;
pub mod project;
pub mod renderview;
pub mod sidebar;
pub mod tracer;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::misc::*;
    pub use crate::object::*;
    pub use crate::panel::*;
    pub use crate::point::*;
    pub use crate::project::*;
    pub use crate::renderview::*;
    pub use crate::tracer::*;

    pub use crate::sidebar::*;
    pub use ::serde::{Deserialize, Serialize};
    pub use maths_rs::prelude::*;
    pub use rustc_hash::*;
    pub use std::sync::Mutex;
    pub use theframework::prelude::*;
    pub use uuid::Uuid;
}

use crate::editor::Editor;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let editor = Editor::new();
    let mut app = TheApp::new();

    _ = app.run(Box::new(editor));
}
