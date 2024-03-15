use crate::prelude::*;
use lazy_static::lazy_static;
use std::sync::mpsc::Receiver;
use theframework::prelude::*;

lazy_static! {
    pub static ref RENDERVIEW: Mutex<RenderView> = Mutex::new(RenderView::default());
    pub static ref PANEL: Mutex<Panel> = Mutex::new(Panel::default());
    pub static ref TRACER: Mutex<Tracer> = Mutex::new(Tracer::new());
}

pub struct EditorContext {
    pub curr_object: Option<Uuid>,
    pub curr_point: Option<Uuid>,
}

pub struct Editor {
    project: Project,
    update_tracker: UpdateTracker,

    context: EditorContext,

    sidebar: Sidebar,
    event_receiver: Option<Receiver<TheEvent>>,
}

impl TheTrait for Editor {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            sidebar: Sidebar::new(),
            event_receiver: None,
            update_tracker: UpdateTracker::default(),

            project: Project::default(),
            context: EditorContext {
                curr_object: None,
                curr_point: None,
            },
        }
    }

    fn init_ui(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        // Menubar
        let mut top_canvas = TheCanvas::new();

        let menubar = TheMenubar::new(TheId::named("Menubar"));

        let mut open_button = TheMenubarButton::new(TheId::named("Open"));
        open_button.set_icon_name("icon_role_load".to_string());

        let mut save_button = TheMenubarButton::new(TheId::named("Save"));
        save_button.set_icon_name("icon_role_save".to_string());

        let mut save_as_button = TheMenubarButton::new(TheId::named("Save As"));
        save_as_button.set_icon_name("icon_role_save_as".to_string());
        save_as_button.set_icon_offset(vec2i(2, -5));

        let mut undo_button = TheMenubarButton::new(TheId::named("Undo"));
        undo_button.set_icon_name("icon_role_undo".to_string());

        let mut redo_button = TheMenubarButton::new(TheId::named("Redo"));
        redo_button.set_icon_name("icon_role_redo".to_string());

        let mut hlayout = TheHLayout::new(TheId::named("Menu Layout"));
        hlayout.set_background_color(None);
        hlayout.set_margin(vec4i(40, 5, 20, 0));
        hlayout.add_widget(Box::new(open_button));
        hlayout.add_widget(Box::new(save_button));
        hlayout.add_widget(Box::new(save_as_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(undo_button));
        hlayout.add_widget(Box::new(redo_button));

        top_canvas.set_widget(menubar);
        top_canvas.set_layout(hlayout);
        ui.canvas.set_top(top_canvas);

        self.sidebar.init_ui(ui, ctx);

        // Shared Layout
        let render_canvas = RENDERVIEW
            .lock()
            .unwrap()
            .init_ui(ui, ctx, &mut self.project);
        let panel_canvas = PANEL.lock().unwrap().init_ui(ui, ctx, &mut self.project);

        let mut vsplitlayout = TheSharedVLayout::new(TheId::named("Shared VLayout"));
        vsplitlayout.add_canvas(render_canvas);
        vsplitlayout.add_canvas(panel_canvas);
        vsplitlayout.set_shared_ratio(0.6);
        vsplitlayout.set_mode(TheSharedVLayoutMode::Shared);

        let mut shared_canvas = TheCanvas::new();
        shared_canvas.set_layout(vsplitlayout);

        ui.canvas.set_center(shared_canvas);

        // Statusbar
        let mut status_canvas = TheCanvas::new();
        let mut statusbar = TheStatusbar::new(TheId::named("Statusbar"));
        statusbar.set_text("Welcome to Signed.".to_string());
        status_canvas.set_widget(statusbar);

        ui.canvas.set_bottom(status_canvas);

        // Get events
        self.event_receiver = Some(ui.add_state_listener("Main Receiver".into()));
    }

    /// Handle UI events and UI state
    fn update_ui(&mut self, ui: &mut TheUI, ctx: &mut TheContext) -> bool {
        let mut redraw = false;

        let tick_update = self.update_tracker.update(500);

        if tick_update {
            if let Some(renderview) = ui.get_render_view("Render View") {
                let dim = renderview.dim();

                let zoom: f32 = 1.0;

                let width = (dim.width as f32 / zoom) as i32;
                let height = (dim.height as f32 / zoom) as i32;

                let buffer = renderview.render_buffer_mut();
                buffer.resize(width, height);
                TRACER.lock().unwrap().render(buffer, &self.project);
                redraw = true;
            }

            if let Some(pointview) = ui.get_render_view("Point View") {
                let dim = pointview.dim();

                let zoom: f32 = 1.0;

                let width = (dim.width as f32 / zoom) as i32;
                let height = (dim.height as f32 / zoom) as i32;

                let buffer = pointview.render_buffer_mut();
                buffer.resize(width, height);
                //TRACER.lock().unwrap().render(buffer, &self.project);
                redraw = true;
            }
        }

        if let Some(receiver) = &mut self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                redraw = self.sidebar.handle_event(
                    &event,
                    ui,
                    ctx,
                    &mut self.project,
                    &mut self.context,
                );
                if PANEL.lock().unwrap().handle_event(
                    &event,
                    ui,
                    ctx,
                    &mut self.project,
                    &mut self.context,
                ) {
                    redraw = true;
                }
                match event {
                    TheEvent::FileRequesterResult(id, paths) => {
                        if id.name == "Open" {
                            for p in paths {
                                let contents = std::fs::read_to_string(p).unwrap_or("".to_string());
                                self.project =
                                    serde_json::from_str(&contents).unwrap_or(Project::default());
                                self.sidebar.load_from_project(ui, ctx, &self.project);
                                redraw = true;
                            }
                        } else if id.name == "Save" {
                            for p in paths {
                                let json = serde_json::to_string(&self.project).unwrap();
                                std::fs::write(p, json).expect("Unable to write file");
                            }
                        }
                    }
                    TheEvent::StateChanged(id, _state) => {
                        // Open / Save Project

                        if id.name == "Open" {
                            ctx.ui.open_file_requester(
                                TheId::named_with_id(id.name.as_str(), Uuid::new_v4()),
                                "Open".into(),
                                TheFileExtension::new(
                                    "Eldiron".into(),
                                    vec!["eldiron".to_string()],
                                ),
                            );
                            ctx.ui
                                .set_widget_state("Open".to_string(), TheWidgetState::None);
                            ctx.ui.clear_hover();
                            redraw = true;
                        } else if id.name == "Save" {
                            ctx.ui.save_file_requester(
                                TheId::named_with_id(id.name.as_str(), Uuid::new_v4()),
                                "Save".into(),
                                TheFileExtension::new(
                                    "Eldiron".into(),
                                    vec!["eldiron".to_string()],
                                ),
                            );
                            ctx.ui
                                .set_widget_state("Save".to_string(), TheWidgetState::None);
                            ctx.ui.clear_hover();
                            redraw = true;
                        }
                    }
                    TheEvent::ValueChanged(id, value) => {
                        //println!("{:?} {:?}", id, value);
                        // if id.name == "Tiles Name Edit" {
                        //     if let Some(list_id) =
                        //         self.sidebar.get_selected_in_list_layout(ui, "Tiles List")
                        //     {
                        //         ctx.ui.send(TheEvent::SetValue(list_id.uuid, value));
                        //     }
                        // }
                    }
                    _ => {}
                }
            }
        }
        redraw
    }
}
