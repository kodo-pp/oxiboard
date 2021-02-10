use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gdk::EventMask;
use cairo::Context as Cairo;
use cairo::{ImageSurface, Format};
use std::error::Error;
use thiserror::Error;


#[derive(Debug, Error)]
#[error("GTK Application returned an error code {0}")]
pub struct GtkAppReturnCodeError(i32);


pub struct Oxiboard {
    gtk_app: Application,
}

impl Oxiboard {
    pub fn new() -> Result<Oxiboard, Box<dyn Error>> {
        let app = Application::new(Some("com.github.kodo-pp.oxiboard"), Default::default())?;

        let surface = ImageSurface::create(Format::ARgb32, 400, 400)?;
        let ctx = Cairo::new(&surface);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.move_to(0.0, 0.0);
        ctx.line_to(200.0, 400.0);
        ctx.line_to(400.0, 200.0);
        ctx.close_path();
        ctx.fill();

        app.connect_activate(move |app| {
            let main_window = ApplicationWindow::new(app);
            main_window.set_title("Oxiboard");
            main_window.set_default_size(800, 600);

            let canvas = DrawingArea::new();
            let surface = surface.clone();
            canvas.connect_draw(move |canvas, ctx| {
                println!("draw");
                let alloc = canvas.get_allocation();
                let w = alloc.width as f64;
                let h = alloc.height as f64;
                ctx.move_to(0.0, 0.0);
                ctx.line_to(w / 2.0, h / 2.0);
                ctx.line_to(w, 0.0);
                ctx.fill();
                ctx.set_source_surface(&surface, w / 2.0 - 200.0, h / 2.0 - 200.0);
                ctx.rectangle(w / 2.0 - 200.0, h / 2.0 - 200.0, 400.0, 400.0);
                ctx.fill();
                Inhibit(false)
            });
            canvas.connect_motion_notify_event(|_canvas, event| {
                dbg!(event.get_coords());
                Inhibit(false)
            });
            canvas.add_events(
                EventMask::POINTER_MOTION_MASK
                    | EventMask::BUTTON_MOTION_MASK
                    | EventMask::BUTTON1_MOTION_MASK
            );
            main_window.add(&canvas);
            main_window.show_all();
        });

        Ok(
            Oxiboard {
                gtk_app: app,
            }
        )
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let exit_code = self.gtk_app.run(&[]);
        if exit_code == 0 {
            Ok(())
        } else {
            Err(GtkAppReturnCodeError(exit_code).into())
        }
    }
}
