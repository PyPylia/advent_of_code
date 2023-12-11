use ndarray::Array2;
use std::mem::MaybeUninit;
use thiserror::Error;

fn position_to_index(position: (u8, u8)) -> (usize, usize) {
    (position.0 as usize, position.1 as usize)
}

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

#[derive(Debug, PartialEq, Eq)]
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

fn get_valid_directions(grid: &Array2<Tile>, start_pos: (u8, u8)) -> [Direction; 2] {
    let mut valids: heapless::Vec<Direction, 2> = heapless::Vec::new();

    for from in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        let next_pos = from.add_offset(start_pos);
        if grid[position_to_index(next_pos)]
            .follow_pipe(from)
            .is_some()
        {
            valids.push(from).ok();
        }
    }

    valids
        .into_array()
        .expect("Should always have two valid directions from start")
}

#[derive(Debug)]
struct GridWalker {
    grid: Array2<Tile>,
    position: (u8, u8),
    facing: Direction,
}

impl GridWalker {
    fn step(&mut self) -> bool {
        let tile = &self.grid[position_to_index(self.position)];
        if *tile == Tile::Start {
            true
        } else {
            self.facing = tile.follow_pipe(self.facing).expect("Invalid direction");
            self.position = self.facing.add_offset(self.position);
            false
        }
    }
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

pub fn first(input: &str) -> eyre::Result<u64> {
    let (grid, start_pos) = new_grid(input)?;
    let valid_directions = get_valid_directions(&grid, start_pos);

    let width = grid.shape()[0];
    let mut distance_grid: Array2<u16> = Array2::zeros((width, width));

    let facing = valid_directions[0];
    let mut walker = GridWalker {
        grid,
        position: facing.add_offset(start_pos),
        facing,
    };

    distance_grid[position_to_index(walker.position)] = 1;

    let mut steps = 1;
    while !walker.step() {
        steps += 1;
        distance_grid[position_to_index(walker.position)] = steps;
    }

    let facing = valid_directions[1];
    walker.facing = facing;
    walker.position = facing.add_offset(start_pos);

    steps = 1;
    while steps < distance_grid[position_to_index(walker.position)] {
        walker.step();
        steps += 1;
    }

    Ok(steps as u64)
}
