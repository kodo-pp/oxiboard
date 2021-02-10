mod app;
use crate::app::Oxiboard;


fn main() {
    let app = Oxiboard::new().expect("Failed to create a GTK application");
    app.run().unwrap();
}
