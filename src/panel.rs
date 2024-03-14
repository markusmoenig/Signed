use crate::prelude::*;

pub struct Panel {}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::new_without_default)]
impl Panel {
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

        // Toolbar
        let mut toolbar_canvas = TheCanvas::default();
        let mut toolbar_hlayout = TheHLayout::new(TheId::empty());
        toolbar_hlayout.limiter_mut().set_max_height(25);
        toolbar_hlayout.set_margin(vec4i(10, 2, 5, 3));
        toolbar_canvas.set_widget(TheTraybar::new(TheId::empty()));
        toolbar_hlayout.set_background_color(None);

        // let mut time_slider = TheTimeSlider::new(TheId::named("TileFX Timeline"));
        // time_slider.set_status_text("The timeline for the tile based effects.");
        // time_slider.limiter_mut().set_max_width(400);
        // toolbar_hlayout.add_widget(Box::new(time_slider));

        // let mut add_button = TheTraybarButton::new(TheId::named("TileFX Clear Marker"));
        // //add_button.set_icon_name("icon_role_add".to_string());
        // add_button.set_text(str!("Clear"));
        // add_button.set_status_text("Clears the currently selected marker.");

        // let mut clear_button = TheTraybarButton::new(TheId::named("TileFX Clear"));
        // //add_button.set_icon_name("icon_role_add".to_string());
        // clear_button.set_text(str!("Clear All"));
        // clear_button.set_status_text("Clears all markers from the timeline.");

        // let mut clear_mask_button = TheTraybarButton::new(TheId::named("TileFX Clear Mask"));
        // clear_mask_button.set_text(str!("Clear Mask"));
        // clear_mask_button.set_status_text("Clear the pixel mask. If there are pixels selected the FX will only be applied to those pixels.");

        // toolbar_hlayout.add_widget(Box::new(add_button));
        // toolbar_hlayout.add_widget(Box::new(clear_button));
        // toolbar_hlayout.add_widget(Box::new(clear_mask_button));
        // toolbar_hlayout.set_reverse_index(Some(1));

        toolbar_canvas.set_layout(toolbar_hlayout);

        canvas.set_top(toolbar_canvas);

        // Center

        let mut center_canvas = TheCanvas::default();

        let mut text_layout = TheTextLayout::new(TheId::named("TileFX Settings"));
        text_layout.limiter_mut().set_max_width(300);
        center_canvas.set_layout(text_layout);

        // let mut center_color_canvas = TheCanvas::default();
        // let mut color_layout = TheVLayout::new(TheId::named("TileFX Color Settings"));
        // color_layout.limiter_mut().set_max_width(140);
        // color_layout.set_background_color(Some(ListLayoutBackground));
        // center_color_canvas.set_layout(color_layout);

        // center_canvas.set_right(center_color_canvas);
        canvas.set_center(center_canvas);

        // Preview

        let mut preview_canvas = TheCanvas::default();

        let mut text_line_edit = TheTextLineEdit::new(TheId::named("Text Edit"));
        text_line_edit.limiter_mut().set_max_width(400);

        let mut tile_rgba = TheRGBAView::new(TheId::named("TileFX RGBA"));
        tile_rgba.set_mode(TheRGBAViewMode::TileSelection);
        tile_rgba.set_grid(Some(10));
        tile_rgba.set_grid_color([40, 40, 40, 255]);
        tile_rgba.set_buffer(TheRGBABuffer::new(TheDim::new(0, 0, 400, 190)));
        tile_rgba.limiter_mut().set_max_size(vec2i(400, 190));

        let mut vlayout = TheVLayout::new(TheId::empty());
        vlayout.limiter_mut().set_max_width(420);
        vlayout.add_widget(Box::new(text_line_edit));
        vlayout.add_widget(Box::new(tile_rgba));
        vlayout.set_background_color(Some(ListLayoutBackground));
        vlayout.set_alignment(TheHorizontalAlign::Left);

        preview_canvas.set_layout(vlayout);

        canvas.set_right(preview_canvas);

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
