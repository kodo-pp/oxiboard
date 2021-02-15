use crate::board::Board;
use crate::draw::Draw;
use cairo::Context as Cairo;
use gdk::EventMask;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, DrawingArea};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("GTK Application returned an error code {0}")]
pub struct GtkAppError(i32);

pub struct Oxiboard {
    canvas: DrawingArea,
    board: Board,
}

fn setup_gtk_app(app: &Application) {
    let glade_ui = include_str!("oxiboard.glade");

    let builder = Builder::new();
    builder
        .add_from_string(glade_ui)
        .expect("Failed to load the user interface");

    let main_window = builder
        .get_object::<ApplicationWindow>("main_window")
        .expect("Failed to locate `main_window`");
    main_window.set_title("Oxiboard");

    let canvas = builder
        .get_object::<DrawingArea>("canvas")
        .expect("Failed to locate `canvas`");

    let event_mask = EventMask::POINTER_MOTION_MASK
        | EventMask::BUTTON_PRESS_MASK
        | EventMask::BUTTON_RELEASE_MASK;
    canvas.add_events(event_mask);

    main_window.show_all();
    app.add_window(&main_window);

    let oxiboard = Rc::new(RefCell::new(Oxiboard {
        canvas,
        board: Board::new(),
    }));

    let oxiboard_clone = Rc::clone(&oxiboard);
    oxiboard
        .borrow()
        .canvas
        .connect_button_press_event(move |canvas, button| {
            oxiboard_clone
                .borrow_mut()
                .handle_button_press_event(canvas, button);
            Inhibit(false)
        });

    let oxiboard_clone = Rc::clone(&oxiboard);
    oxiboard
        .borrow()
        .canvas
        .connect_button_release_event(move |canvas, button| {
            oxiboard_clone
                .borrow_mut()
                .handle_button_release_event(canvas, button);
            Inhibit(false)
        });

    let oxiboard_clone = Rc::clone(&oxiboard);
    oxiboard
        .borrow()
        .canvas
        .connect_motion_notify_event(move |canvas, motion| {
            oxiboard_clone
                .borrow_mut()
                .handle_motion_notify_event(canvas, motion);
            Inhibit(false)
        });

    let oxiboard_clone = Rc::clone(&oxiboard);
    oxiboard.borrow().canvas.connect_draw(move |canvas, ctx| {
        oxiboard_clone.borrow_mut().handle_draw_event(canvas, ctx);
        Inhibit(false)
    });
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let gtk_app = Application::new(None, Default::default()).unwrap();
    gtk_app.connect_activate(setup_gtk_app);

    let return_code = gtk_app.run(&[]);
    match return_code {
        0 => Ok(()),
        x => Err(Box::new(GtkAppError(x))),
    }
}

impl Oxiboard {
    fn handle_button_press_event(&mut self, canvas: &DrawingArea, button: &gdk::EventButton) {
        if let Some(coords) = button.get_coords() {
            self.board.begin_drawing(coords).unwrap();
        }
        canvas.queue_draw();
    }

    fn handle_button_release_event(&mut self, _canvas: &DrawingArea, _button: &gdk::EventButton) {
        self.board.finish().unwrap();
    }

    fn handle_motion_notify_event(&mut self, canvas: &DrawingArea, motion: &gdk::EventMotion) {
        match (self.board.is_active(), motion.get_coords()) {
            (true, Some(coords)) => {
                self.board.add_point(coords).unwrap();
            }
            _ => (),
        }
        canvas.queue_draw()
    }

    fn handle_draw_event(&self, _canvas: &DrawingArea, ctx: &Cairo) {
        ctx.set_line_width(5.0);
        ctx.set_source_rgb(0.0, 0.0, 1.0);
        ctx.set_line_cap(cairo::LineCap::Round);
        self.board.draw(ctx);
    }
}
