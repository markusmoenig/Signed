use crate::prelude::*;

pub struct Sidebar {
    pub width: i32,

    stack_layout_id: TheId,
}

#[allow(clippy::new_without_default)]
impl Sidebar {
    pub fn new() -> Self {
        Self {
            width: 380,
            stack_layout_id: TheId::empty(),
        }
    }

    pub fn init_ui(&mut self, ui: &mut TheUI, _ctx: &mut TheContext) {
        let width = self.width;

        let mut sectionbar_canvas = TheCanvas::new();

        let mut section_bar_canvas = TheCanvas::new();
        section_bar_canvas.set_widget(TheSectionbar::new(TheId::named("Sectionbar")));
        sectionbar_canvas.set_top(section_bar_canvas);

        let mut objects_sectionbar_button =
            TheSectionbarButton::new(TheId::named("Objects Section"));
        objects_sectionbar_button.set_text("Objects".to_string());
        objects_sectionbar_button.set_state(TheWidgetState::Selected);

        let mut points_sectionbar_button = TheSectionbarButton::new(TheId::named("Points Section"));
        points_sectionbar_button.set_text("Points".to_string());

        // let mut item_sectionbar_button = TheSectionbarButton::new("Items Section".to_string());
        // item_sectionbar_button.set_text("Items".to_string());

        let mut shapes_sectionbar_button = TheSectionbarButton::new(TheId::named("Shapes Section"));
        shapes_sectionbar_button.set_text("Shapes".to_string());

        let mut vlayout = TheVLayout::new(TheId::named("Section Buttons"));
        vlayout.add_widget(Box::new(objects_sectionbar_button));
        vlayout.add_widget(Box::new(points_sectionbar_button));
        //vlayout.add_widget(Box::new(item_sectionbar_button));
        vlayout.add_widget(Box::new(shapes_sectionbar_button));
        vlayout.set_margin(vec4i(5, 10, 5, 10));
        vlayout.set_padding(4);
        vlayout.set_background_color(Some(SectionbarBackground));
        vlayout.limiter_mut().set_max_width(90);
        sectionbar_canvas.set_layout(vlayout);

        //

        let mut header = TheCanvas::new();
        let mut switchbar = TheSwitchbar::new(TheId::named("Switchbar Section Header"));
        switchbar.set_text("Objects".to_string());
        header.set_widget(switchbar);

        let mut stack_layout = TheStackLayout::new(TheId::named("List Stack Layout"));

        stack_layout.limiter_mut().set_max_width(width);

        self.stack_layout_id = stack_layout.id().clone();

        // Objects

        let mut objects_canvas = TheCanvas::default();

        let mut list_layout = TheListLayout::new(TheId::named("Object List"));
        list_layout.limiter_mut().set_max_size(vec2i(width, 200));
        let mut list_canvas = TheCanvas::default();
        list_canvas.set_layout(list_layout);

        let mut regions_add_button = TheTraybarButton::new(TheId::named("Object Add"));
        regions_add_button.set_icon_name("icon_role_add".to_string());
        let mut regions_remove_button = TheTraybarButton::new(TheId::named("Object Remove"));
        regions_remove_button.set_icon_name("icon_role_remove".to_string());

        let mut toolbar_hlayout = TheHLayout::new(TheId::empty());
        toolbar_hlayout.set_background_color(None);
        toolbar_hlayout.set_margin(vec4i(5, 2, 5, 0));
        toolbar_hlayout.add_widget(Box::new(regions_add_button));
        toolbar_hlayout.add_widget(Box::new(regions_remove_button));

        let mut toolbar_canvas = TheCanvas::default();
        toolbar_canvas.set_widget(TheTraybar::new(TheId::empty()));
        toolbar_canvas.set_layout(toolbar_hlayout);
        list_canvas.set_bottom(toolbar_canvas);

        let mut text_layout = TheTextLayout::new(TheId::empty());
        text_layout.limiter_mut().set_max_width(width);
        let name_edit = TheTextLineEdit::new(TheId::named("Regions Name Edit"));
        text_layout.add_pair("Name".to_string(), Box::new(name_edit));

        // let mut yellow_canvas = TheCanvas::default();
        // let mut yellow_color = TheColorButton::new(TheId::named("Yellow"));
        // yellow_color.set_color([255, 255, 0, 255]);
        // yellow_color.limiter_mut().set_max_size(vec2i(width, 350));
        // yellow_canvas.set_widget(yellow_color);

        objects_canvas.set_top(list_canvas);
        objects_canvas.set_layout(text_layout);
        // objects_canvas.set_bottom(yellow_canvas);
        stack_layout.add_canvas(objects_canvas);

        // Points

        let mut points_canvas = TheCanvas::default();

        let mut list_layout = TheListLayout::new(TheId::named("Point List"));
        list_layout.limiter_mut().set_max_size(vec2i(width, 200));
        let mut list_canvas = TheCanvas::default();
        list_canvas.set_layout(list_layout);

        let mut points_add_button = TheTraybarButton::new(TheId::named("Points Add"));
        points_add_button.set_icon_name("icon_role_add".to_string());
        let mut points_remove_button = TheTraybarButton::new(TheId::named("Points Remove"));
        points_remove_button.set_icon_name("icon_role_remove".to_string());

        let mut toolbar_hlayout = TheHLayout::new(TheId::empty());
        toolbar_hlayout.set_background_color(None);
        toolbar_hlayout.set_margin(vec4i(5, 2, 5, 0));
        toolbar_hlayout.add_widget(Box::new(points_add_button));
        toolbar_hlayout.add_widget(Box::new(points_remove_button));

        let mut toolbar_canvas = TheCanvas::default();
        toolbar_canvas.set_widget(TheTraybar::new(TheId::empty()));
        toolbar_canvas.set_layout(toolbar_hlayout);
        list_canvas.set_bottom(toolbar_canvas);

        let mut text_layout = TheTextLayout::new(TheId::empty());
        text_layout.limiter_mut().set_max_width(width);
        let name_edit = TheTextLineEdit::new(TheId::named("Character Name Edit"));
        text_layout.add_pair("Name".to_string(), Box::new(name_edit));

        // let mut red_canvas = TheCanvas::default();
        // let mut red_color = TheColorButton::new(TheId::named("Red"));
        // red_color.set_color([255, 0, 0, 255]);
        // red_color.limiter_mut().set_max_size(vec2i(width, 350));
        // red_canvas.set_widget(red_color);

        points_canvas.set_top(list_canvas);
        points_canvas.set_layout(text_layout);
        // points_canvas.set_bottom(red_canvas);
        stack_layout.add_canvas(points_canvas);

        // Shapes

        let mut shapes_canvas = TheCanvas::default();

        let mut list_layout = TheListLayout::new(TheId::named("Shapes List"));
        list_layout.limiter_mut().set_max_size(vec2i(width, 200));
        let mut list_canvas = TheCanvas::default();
        list_canvas.set_layout(list_layout);

        let mut shapes_add_button = TheTraybarButton::new(TheId::named("Shapes Add"));
        shapes_add_button.set_icon_name("icon_role_add".to_string());
        let mut shapes_remove_button = TheTraybarButton::new(TheId::named("Shapes Remove"));
        shapes_remove_button.set_icon_name("icon_role_remove".to_string());

        let mut toolbar_hlayout = TheHLayout::new(TheId::empty());
        toolbar_hlayout.set_background_color(None);
        toolbar_hlayout.set_margin(vec4i(5, 2, 5, 0));
        toolbar_hlayout.add_widget(Box::new(shapes_add_button));
        toolbar_hlayout.add_widget(Box::new(shapes_remove_button));

        let mut toolbar_canvas = TheCanvas::default();
        toolbar_canvas.set_widget(TheTraybar::new(TheId::empty()));
        toolbar_canvas.set_layout(toolbar_hlayout);
        list_canvas.set_bottom(toolbar_canvas);

        let mut text_layout = TheTextLayout::new(TheId::empty());
        text_layout.limiter_mut().set_max_width(width);
        let name_edit = TheTextLineEdit::new(TheId::named("Tiles Name Edit"));
        text_layout.add_pair("Name".to_string(), Box::new(name_edit));
        let grid_edit = TheTextLineEdit::new(TheId::named("Tiles Grid Edit"));
        text_layout.add_pair("Grid Size".to_string(), Box::new(grid_edit));

        let mut tiles_list_canvas = TheCanvas::default();

        let mut tiles_list_header_canvas = TheCanvas::default();
        tiles_list_header_canvas.set_widget(TheTraybar::new(TheId::empty()));

        let mut tile_list_layout = TheListLayout::new(TheId::named("Tiles Tilemap List"));
        tile_list_layout
            .limiter_mut()
            .set_max_size(vec2i(width, 360));

        tiles_list_canvas.set_top(tiles_list_header_canvas);
        tiles_list_canvas.set_layout(tile_list_layout);

        shapes_canvas.set_top(list_canvas);
        shapes_canvas.set_layout(text_layout);
        shapes_canvas.set_bottom(tiles_list_canvas);
        stack_layout.add_canvas(shapes_canvas);

        //

        let mut canvas = TheCanvas::new();

        canvas.set_top(header);
        canvas.set_right(sectionbar_canvas);
        canvas.top_is_expanding = false;
        canvas.set_layout(stack_layout);

        ui.canvas.set_right(canvas);
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
            TheEvent::StateChanged(id, state) => {
                if id.name == "Object Add" {
                    if let Some(list_layout) = ui.get_list_layout("Object List") {
                        let object = Object::default();

                        let mut item =
                            TheListItem::new(TheId::named_with_id("Object Item", object.id));
                        item.set_text(object.name.clone());
                        item.set_state(TheWidgetState::Selected);
                        item.set_context_menu(Some(TheContextMenu {
                            items: vec![TheContextMenuItem::new(
                                "Rename Object...".to_string(),
                                TheId::named("Rename Object"),
                            )],
                            ..Default::default()
                        }));
                        list_layout.deselect_all();
                        let id = item.id().clone();
                        list_layout.add_item(item, ctx);
                        ctx.ui
                            .send_widget_state_changed(&id, TheWidgetState::Selected);

                        project.add_object(object);
                    }
                }
            }
            _ => {}
        }

        redraw
    }

    pub fn load_from_project(&mut self, ui: &mut TheUI, ctx: &mut TheContext, project: &Project) {
        if let Some(layout) = ui.canvas.get_layout(Some(&"Tiles List".to_string()), None) {
            if let Some(list_layout) = layout.as_list_layout() {
                // list_layout.clear();
                // for t in &project.tilemaps {
                //     let mut item = TheListItem::new(TheId::named_with_id("Tiles Item", t.id));
                //     item.set_text(t.name.clone());
                //     //item.set_state(TheWidgetState::Selected);
                //     // list_layout.deselect_all();
                //     // let id = item.id().clone();
                //     list_layout.add_item(item, ctx);
                //     // ctx.ui.send_widget_state_changed(&id, TheWidgetState::Selected);
                // }
            }
        }
    }

    /// Apply the given item to the UI
    ///
    /*
    pub fn apply_tilemap(
        &mut self,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        tilemap: Option<&Tilemap>,
    ) {
        if let Some(widget) = ui
            .canvas
            .get_widget(Some(&"Tiles Name Edit".to_string()), None)
        {
            if let Some(tilemap) = tilemap {
                widget.set_value(TheValue::Text(tilemap.name.clone()));
            } else {
                widget.set_value(TheValue::Empty);
            }
        }
        if let Some(widget) = ui
            .canvas
            .get_widget(Some(&"Tiles Grid Edit".to_string()), None)
        {
            if let Some(tilemap) = tilemap {
                widget.set_value(TheValue::Text(tilemap.grid_size.clone().to_string()));
            } else {
                widget.set_value(TheValue::Empty);
            }
        }

        //
        if let Some(layout) = ui
            .canvas
            .get_layout(Some(&"Tiles Tilemap List".to_string()), None)
        {
            if let Some(list_layout) = layout.as_list_layout() {
                if let Some(tilemap) = tilemap {
                    list_layout.clear();
                    for tile in &tilemap.tiles {
                        let mut item =
                            TheListItem::new(TheId::named_with_id("Tiles Tilemap Item", tile.id));
                        item.set_text(tile.name.clone());
                        item.set_state(TheWidgetState::Selected);
                        list_layout.deselect_all();
                        list_layout.add_item(item, ctx);
                    }
                } else {
                    list_layout.clear();
                }
            }
        }
        }*/

    /// Deselects the section buttons
    pub fn deselect_sections_buttons(&mut self, ui: &mut TheUI, except: String) {
        if let Some(layout) = ui.canvas.get_layout(Some(&"Section Buttons".into()), None) {
            for w in layout.widgets() {
                if !w.id().name.starts_with(&except) {
                    w.set_state(TheWidgetState::None);
                }
            }
        }
    }

    /// Returns the selected id in the given list layout
    pub fn get_selected_in_list_layout(&self, ui: &mut TheUI, layout_name: &str) -> Option<TheId> {
        if let Some(layout) = ui.canvas.get_layout(Some(&layout_name.to_string()), None) {
            if let Some(list_layout) = layout.as_list_layout() {
                return list_layout.selected();
            }
        }
        None
    }
}
