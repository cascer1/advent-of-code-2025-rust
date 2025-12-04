use strum_macros::EnumIter;

pub mod template;

// Use this file to add helper functions and additional modules.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, EnumIter)]
pub enum CardinalDirection {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl CardinalDirection {
    pub fn position_at_coords(&self, x: usize, y: usize) -> Option<Pos> {
        self.position_at(Pos { x, y })
    }

    pub fn position_at(&self, pos: Pos) -> Option<Pos> {
        match self {
            CardinalDirection::NorthWest => {
                let x = pos.x.checked_sub(1)?;
                let y = pos.y.checked_sub(1)?;
                Some(Pos { x, y })
            }
            CardinalDirection::North => {
                let y = pos.y.checked_sub(1)?;
                Some(Pos { x: pos.x, y })
            }
            CardinalDirection::NorthEast => {
                let y = pos.y.checked_sub(1)?;
                Some(Pos { x: pos.x + 1, y })
            }
            CardinalDirection::West => {
                let x = pos.x.checked_sub(1)?;
                Some(Pos { x, y: pos.y })
            }
            CardinalDirection::East => Some(Pos {
                x: pos.x + 1,
                y: pos.y,
            }),
            CardinalDirection::SouthWest => {
                let x = pos.x.checked_sub(1)?;
                Some(Pos { x, y: pos.y + 1 })
            }
            CardinalDirection::South => Some(Pos {
                x: pos.x,
                y: pos.y + 1,
            }),
            CardinalDirection::SouthEast => Some(Pos {
                x: pos.x + 1,
                y: pos.y + 1,
            }),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}
