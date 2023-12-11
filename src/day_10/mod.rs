use ndarray::Array2;
use std::mem::MaybeUninit;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn add_offset(&self, position: (u8, u8)) -> (u8, u8) {
        match self {
            Self::North => (position.0, position.1 + 1),
            Self::East => (position.0 + 1, position.1),
            Self::South => (position.0, position.1 - 1),
            Self::West => (position.0 - 1, position.1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    None,
    Start,
}

impl Tile {
    fn follow_pipe(&self, from: Direction) -> Option<Direction> {
        match self {
            Self::NorthSouth => match from {
                Direction::North => Some(Direction::North),
                Direction::South => Some(Direction::South),
                _ => None,
            },
            Self::EastWest => match from {
                Direction::East => Some(Direction::East),
                Direction::West => Some(Direction::West),
                _ => None,
            },
            Self::NorthEast => match from {
                Direction::South => Some(Direction::East),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Self::NorthWest => match from {
                Direction::South => Some(Direction::West),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Self::SouthEast => match from {
                Direction::North => Some(Direction::East),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            Self::SouthWest => match from {
                Direction::North => Some(Direction::West),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Error)]
#[error("Invalid tile: {0}")]
struct InvalidTile(char);
impl TryFrom<u8> for Tile {
    type Error = InvalidTile;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'|' => Self::NorthSouth,
            b'-' => Self::EastWest,
            b'L' => Self::NorthEast,
            b'J' => Self::NorthWest,
            b'7' => Self::SouthWest,
            b'F' => Self::SouthEast,
            b'.' => Self::None,
            b'S' => Self::Start,
            other => return Err(InvalidTile(other as char)),
        })
    }
}

fn index_grid(grid: &Array2<Tile>, position: (u8, u8)) -> Tile {
    grid[(position.0 as usize, position.1 as usize)]
}

fn new_grid(input: &str) -> eyre::Result<(Array2<Tile>, (u8, u8))> {
    let width = unsafe { input.lines().next().unwrap_unchecked() }
        .as_bytes()
        .len();
    let mut grid = Array2::uninit((width, width));

    let mut start_pos = None;
    for (y, line) in input.lines().rev().enumerate() {
        for (x, byte) in line.as_bytes().into_iter().enumerate() {
            let tile = Tile::try_from(*byte)?;
            if tile == Tile::Start {
                start_pos = Some((x as u8, y as u8));
            }

            grid[(x, y)] = MaybeUninit::new(tile);
        }
    }

    Ok((
        unsafe { grid.assume_init() },
        start_pos.ok_or_else(|| eyre::eyre!("No start position found"))?,
    ))
}

#[derive(Debug)]
struct GridWalker {
    grid: Array2<Tile>,
    position: (u8, u8),
    facing: Direction,
}

impl GridWalker {
    fn new(grid: Array2<Tile>, start_pos: (u8, u8)) -> Self {
        for facing in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let next_pos = facing.add_offset(start_pos);
            if index_grid(&grid, next_pos).follow_pipe(facing).is_some() {
                return Self {
                    grid,
                    position: next_pos,
                    facing,
                };
            }
        }

        panic!("No valid direction found from start")
    }

    fn step(&mut self) -> bool {
        let tile = index_grid(&self.grid, self.position);
        if tile == Tile::Start {
            true
        } else {
            self.facing = tile.follow_pipe(self.facing).expect("Invalid direction");
            self.position = self.facing.add_offset(self.position);
            false
        }
    }
}

pub fn first(input: &str) -> eyre::Result<u64> {
    let (grid, start_pos) = new_grid(input)?;
    let mut walker = GridWalker::new(grid, start_pos);

    let mut steps = 1;
    while !walker.step() {
        steps += 1;
    }

    Ok((steps / 2) as u64)
}
