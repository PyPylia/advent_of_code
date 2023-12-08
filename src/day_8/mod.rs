use std::{fmt, mem::MaybeUninit};

use thiserror::Error;

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Error)]
#[error("Invalid turn: {0}")]
struct InvalidTurn(char);
impl TryFrom<u8> for Turn {
    type Error = InvalidTurn;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'L' => Ok(Self::Left),
            b'R' => Ok(Self::Right),
            other => Err(InvalidTurn(other as char)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeType {
    Start,
    Middle,
    End,
}

impl From<u8> for NodeType {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Self::Start,
            b'Z' => Self::End,
            _ => Self::Middle,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NodeID {
    index: u16,
    node_type: NodeType,
}

impl From<[u8; 3]> for NodeID {
    fn from(value: [u8; 3]) -> Self {
        let index = (value[0] - b'A') as u16 * 26 * 26
            + (value[1] - b'A') as u16 * 26
            + (value[2] - b'A') as u16;

        Self {
            index,
            node_type: value[2].into(),
        }
    }
}

impl fmt::Display for NodeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.index / 26;
        let last = (self.index % 26) as u8 + b'A';
        let middle = (a % 26) as u8 + b'A';
        let first = (a / 26) as u8 + b'A';

        f.write_str(unsafe { std::str::from_utf8_unchecked(&[first, middle, last]) })
    }
}

#[derive(Clone, Copy, Debug)]
struct Node {
    left: NodeID,
    right: NodeID,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({}, {})", self.left, self.right)
    }
}

impl Node {
    fn from_bytes(s: &[u8]) -> eyre::Result<(NodeID, Self)> {
        let id: [u8; 3] = s[0..3].try_into()?;
        let left: [u8; 3] = s[7..10].try_into()?;
        let right: [u8; 3] = s[12..15].try_into()?;

        Ok((
            id.into(),
            Self {
                left: left.into(),
                right: right.into(),
            },
        ))
    }

    fn turn(self, turn: Turn) -> NodeID {
        match turn {
            Turn::Left => self.left,
            Turn::Right => self.right,
        }
    }
}

type NodeMap = [MaybeUninit<Node>; 26usize.pow(3)];

fn get_turns_nodes(
    input: &str,
) -> eyre::Result<(impl Iterator<Item = Turn>, impl Iterator<Item = &str>)> {
    let (turns_str, nodes_str) = input
        .split_once("\r\n\r\n")
        .ok_or_else(|| eyre::eyre!("Invalid input"))?;

    let turns: Result<Vec<Turn>, InvalidTurn> = turns_str
        .as_bytes()
        .into_iter()
        .map(|byte| Turn::try_from(*byte))
        .collect();
    let turns = turns?.into_iter().cycle();

    Ok((turns, nodes_str.lines()))
}

pub fn first(input: &str) -> eyre::Result<u64> {
    let (mut turns, nodes) = get_turns_nodes(input)?;

    let mut node_map: NodeMap = MaybeUninit::uninit_array();
    for line in nodes {
        let (id, node) = Node::from_bytes(line.as_bytes())?;
        node_map[id.index as usize] = MaybeUninit::new(node);
    }

    let mut steps = 0u64;
    let mut current_id: NodeID = [b'A'; 3].into();

    let zzz: NodeID = [b'Z'; 3].into();
    while current_id != zzz {
        // SAFETY: The node map given by Advent of Code is closed, so every node we access will always be initialized.
        let node = unsafe { node_map[current_id.index as usize].assume_init() };
        // SAFETY: Turns cannot be empty, and repeats infinitely with .cycle
        current_id = node.turn(unsafe { turns.next().unwrap_unchecked() });
        steps += 1;
    }

    Ok(steps)
}

pub fn second(input: &str) -> eyre::Result<u64> {
    let (mut turns, nodes) = get_turns_nodes(input)?;

    let mut current_nodes = vec![];
    let mut node_map: NodeMap = MaybeUninit::uninit_array();
    for line in nodes {
        let (id, node) = Node::from_bytes(line.as_bytes())?;
        node_map[id.index as usize] = MaybeUninit::new(node);
        if let NodeType::Start = id.node_type {
            current_nodes.push(id);
        }
    }

    let mut steps = 0u64;
    let mut isnt_end = true;
    while isnt_end {
        // SAFETY: Turns cannot be empty, and repeats infinitely with .cycle
        let turn = unsafe { turns.next().unwrap_unchecked() };

        isnt_end = false;
        for node_id in &mut current_nodes {
            // SAFETY: The node map given by Advent of Code is closed, so every node we access will always be initialized.
            let node = unsafe { node_map[node_id.index as usize].assume_init() };
            *node_id = node.turn(turn);
            if node_id.node_type != NodeType::End {
                isnt_end = true;
            }
        }

        steps += 1;
    }

    Ok(steps)
}
