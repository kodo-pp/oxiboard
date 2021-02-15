pub use cairo::Context as Cairo;

pub trait Draw {
    fn draw(&self, ctx: &Cairo);
}
