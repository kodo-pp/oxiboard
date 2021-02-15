use crate::draw::{Cairo, Draw};
use enum_as_inner::EnumAsInner;

#[derive(Debug)]
pub struct WrongBoardStateError {
    is_active: bool,
    description: Option<String>,
}

impl WrongBoardStateError {
    pub fn expected_static(description: Option<impl Into<String>>) -> Self { 
        Self {
            is_active: true,
            description: description.map(Into::into),
        }
    }

    pub fn expected_active(description: Option<impl Into<String>>) -> Self {
        Self {
            is_active: false,
            description: description.map(Into::into),
        }
    }
}

impl std::fmt::Display for WrongBoardStateError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_active {
            write!(fmt, "A glyph is already being drawn")?;
        } else {
            write!(fmt, "No glyph is currently being drawn")?;
        }

        if let Some(desc) = &self.description {
            write!(fmt, ", so {}", desc)?;
        }
        Ok(())
    }
}

impl std::error::Error for WrongBoardStateError {}

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

    pub fn current_glyph(&self) -> &Glyph {
        &self.current_glyph
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

    pub fn begin_drawing(&mut self, initial_point: Point) -> Result<(), WrongBoardStateError> {
        match self {
            Self::Static(_) => (),
            _ => {
                return Err(WrongBoardStateError::expected_static(Some(
                    "cannot start drawing another glyph",
                )))
            }
        }

        take_mut::take(self, |board| {
            Self::Active(board.into_static().unwrap().begin_drawing(initial_point))
        });

        Ok(())
    }

    pub fn add_point(&mut self, point: Point) -> Result<(), WrongBoardStateError> {
        match self {
            Self::Active(board) => Ok(board.add_point(point)),
            Self::Static(_) => Err(WrongBoardStateError::expected_active(Some(
                "cannot add a point to the current glyph"
            )))
        }
    }

    pub fn finish(&mut self) -> Result<(), WrongBoardStateError> {
        match self {
            Self::Active(_) => (),
            _ => return Err(WrongBoardStateError::expected_active(Some(
                "cannot finish drawing the current glyph"
            ))),
        }

        take_mut::take(self, |board| {
            Self::Static(board.into_active().unwrap().finish())
        });

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        match self {
            Self::Active(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn current_glyph(&self) -> Result<&Glyph, WrongBoardStateError> {
        match self {
            Self::Active(board) => Ok(board.current_glyph()),
            _ => Err(WrongBoardStateError::expected_active(Some(
                "there is no current glyph",
            ))),
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
