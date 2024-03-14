use crate::prelude::*;

pub struct RenderView {}

impl Default for RenderView {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::new_without_default)]
impl RenderView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init_ui(
        &mut self,
        _ui: &mut TheUI,
        _ctx: &mut TheContext,
        _project: &mut Project,
    ) -> TheCanvas {
        let mut canvas = TheCanvas::new();

        let render_view = TheRenderView::new(TheId::named("Render View"));
        canvas.set_widget(render_view);
        canvas
    }

    pub fn handle_event(
        &mut self,
        event: &TheEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        project: &mut Project,
    ) -> bool {
        let mut redraw = false;
        match event {
            _ => {}
        }
        redraw
    }
}
