use crate::draw::{Cairo, Draw};
use enum_as_inner::EnumAsInner;

pub type Point = (f64, f64);

#[derive(Debug)]
pub struct StaticBoard {
    glyphs: Vec<Glyph>,
}

#[derive(Debug)]
pub struct ActiveBoard {
    board: StaticBoard,
    current_glyph: Glyph,
}

impl StaticBoard {
    pub fn new() -> Self {
        Self { glyphs: Vec::new() }
    }

    pub fn begin_drawing(self, initial_point: Point) -> ActiveBoard {
        let current_glyph = Glyph {
            points: vec![initial_point],
        };
        ActiveBoard {
            board: self,
            current_glyph,
        }
    }
}

impl ActiveBoard {
    pub fn add_point(&mut self, point: Point) {
        self.current_glyph.points.push(point);
    }

    pub fn finish(self) -> StaticBoard {
        let mut board = self.board;
        board.glyphs.push(self.current_glyph);
        board
    }
}

#[derive(Debug)]
pub struct Glyph {
    points: Vec<Point>,
}

#[derive(Debug, EnumAsInner)]
pub enum Board {
    Static(StaticBoard),
    Active(ActiveBoard),
}

impl Board {
    pub fn new() -> Self {
        Self::Static(StaticBoard::new())
    }

    pub fn begin_drawing(&mut self, initial_point: Point) {
        match self {
            Self::Static(_) => (),
            _ => panic!("Attempted to start drawing a glyph while another one is not finished"),
        }

        take_mut::take(self, |board| {
            Self::Active(board.into_static().unwrap().begin_drawing(initial_point))
        });
    }

    pub fn add_point(&mut self, point: Point) {
        match self {
            Self::Active(board) => board.add_point(point),
            Self::Static(_) => panic!("Attempted to add a point to a static board"),
        }
    }

    pub fn finish(&mut self) {
        match self {
            Self::Active(_) => (),
            _ => panic!("Attempted to finish drawing a glyph although no glyphs are being drawn"),
        }

        take_mut::take(self, |board| {
            Self::Static(board.into_active().unwrap().finish())
        });
    }

    pub fn is_active(&self) -> bool {
        match self {
            Self::Active(_) => true,
            _ => false,
        }
    }
}

impl Draw for Glyph {
    fn draw(&self, ctx: &Cairo) {
        ctx.set_source_rgb(0.0, 0.0, 1.0);
        ctx.set_line_cap(cairo::LineCap::Round);
        let (x0, y0) = self.points[0];
        ctx.move_to(x0, y0);
        if self.points.len() == 1 {
            ctx.line_to(x0, y0);
            ctx.stroke();
            return;
        }

        for point_pair in self.points.windows(2) {
            let (x1, y1) = point_pair[0];
            let (x2, y2) = point_pair[1];
            ctx.move_to(x1, y1);
            ctx.line_to(x2, y2);
            ctx.stroke();
        }
    }
}

impl Draw for StaticBoard {
    fn draw(&self, ctx: &Cairo) {
        for glyph in self.glyphs.iter() {
            glyph.draw(ctx);
        }
    }
}

impl Draw for ActiveBoard {
    fn draw(&self, ctx: &Cairo) {
        self.board.draw(ctx);
        self.current_glyph.draw(ctx);
    }
}

impl Draw for Board {
    fn draw(&self, ctx: &Cairo) {
        match self {
            Self::Active(board) => board.draw(ctx),
            Self::Static(board) => board.draw(ctx),
        }
    }
}
