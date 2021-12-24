use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    io::Read,
};

use rustc_hash::FxHashMap;

const HALLWAY: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOMS: [usize; 4] = [2, 4, 6, 8];
const DEPTH: usize = 4;
const WIDTH: usize = 11;

fn parse_input() -> BTreeMap<(usize, usize), Square> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut positions = BTreeMap::new();
    for (y, row) in input.trim().split('\n').enumerate() {
        for (x, sq) in row.chars().enumerate() {
            if let Ok(square) = Square::try_from(sq) {
                let pos = (x - 1, if y == 2 { 1 } else { 4 });
                positions.insert(pos, square);
            }
        }
    }

    for (pos, sq) in [
        ((2, 2), Square::D),
        ((2, 3), Square::D),
        ((4, 2), Square::C),
        ((4, 3), Square::B),
        ((6, 2), Square::B),
        ((6, 3), Square::A),
        ((8, 2), Square::A),
        ((8, 3), Square::C),
    ] {
        positions.insert(pos, sq);
    }

    positions
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Square {
    A,
    B,
    C,
    D,
}

impl TryFrom<char> for Square {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            _ => Err(()),
        }
    }
}

impl Square {
    fn correct_room(&self) -> usize {
        match self {
            Self::A => 2,
            Self::B => 4,
            Self::C => 6,
            Self::D => 8,
        }
    }

    fn energy(&self) -> i64 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    energy: i64,
    positions: BTreeMap<(usize, usize), Square>,
}

impl State {
    fn new(positions: BTreeMap<(usize, usize), Square>) -> Self {
        Self {
            energy: 0,
            positions,
        }
    }

    fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        // Move must be valid
        let sq = self.positions.remove(&from).unwrap();
        self.positions.insert(to, sq);
        self.energy += ((from.0 as i64 - to.0 as i64).abs() + (from.1 as i64 - to.1 as i64).abs())
            * sq.energy();
    }

    fn reduce(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for x in HALLWAY {
                let pos = (x, 0);
                if let Some(&sq) = self.positions.get(&pos) {
                    let room = sq.correct_room();

                    if !self.can_move_pos_to_room(x, room) {
                        continue;
                    }
                    for y in (1..=DEPTH).rev() {
                        let target = (room, y);
                        if let Some(&already) = self.positions.get(&target) {
                            if already.correct_room() != room {
                                break;
                            }
                        } else {
                            self.make_move(pos, target);
                            changed = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn can_move_pos_to_room(&self, start: usize, room: usize) -> bool {
        assert_ne!(start, room);
        let (a, b) = if start < room {
            (start + 1, room)
        } else {
            (room, start - 1)
        };
        (a..=b).all(|x| !self.positions.contains_key(&(x, 0)))
    }

    fn search_hallway(&self, start: usize) -> [bool; WIDTH] {
        let mut bag = vec![start];
        let mut visited = [false; WIDTH];
        visited[start] = true;

        while let Some(pos) = bag.pop() {
            for dp in [-1, 1] {
                let new_pos = pos as isize + dp;
                if new_pos < 0 || new_pos >= WIDTH as isize {
                    continue;
                }
                let new_pos = new_pos as usize;
                if !visited[new_pos] && !self.positions.contains_key(&(new_pos, 0)) {
                    bag.push(new_pos);
                    visited[new_pos] = true;
                }
            }
        }
        visited
    }

    fn possible_moves(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut moves = vec![];
        for (&pos, &_sq) in self.positions.iter() {
            if pos.1 == 0 {
                continue;
            }
            // let cr = sq.correct_room();
            // if self.can_move_pos_to_room(pos.0, cr) {}
            for x in self.moveable_hallways_for(pos) {
                moves.push((pos, (x, 0)));
            }
        }
        moves
    }

    fn moveable_hallways_for(&self, start: (usize, usize)) -> impl Iterator<Item = usize> {
        // If it and everything below it is in the correct position, then we don't want to move it.
        if (start.1..=DEPTH)
            .map(|y| (start.0, y))
            .all(|pos| self.positions.get(&pos).unwrap().correct_room() == start.0)
        {
            return vec![].into_iter();
        }
        for y in 1..start.1 {
            if self.positions.contains_key(&(start.0, y)) {
                return vec![].into_iter();
            }
        }

        let mut movable = self.search_hallway(start.0);
        for x in ROOMS {
            movable[x] = false;
        }
        movable
            .into_iter()
            .enumerate()
            .filter_map(|(x, able)| able.then(|| x))
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn is_solved(&self) -> bool {
        for (&(x, _y), sq) in self.positions.iter() {
            if sq.correct_room() != x {
                return false;
            }
        }
        true
    }
}

fn main() {
    let positions = parse_input();

    let start = State::new(positions);
    let mut bag: BinaryHeap<_> = [Reverse(start)].into();
    let mut visited = FxHashMap::default();

    while let Some(Reverse(state)) = bag.pop() {
        if state.is_solved() {
            println!("{}", state.energy);
            break;
        }
        for (from, to) in state.possible_moves() {
            let mut new_state = state.clone();
            new_state.make_move(from, to);
            new_state.reduce();
            // Slight bug where multiple new_states can be reduced to the same thing. If the one with
            // higher cost is processed first, then we will end up exploring extra states.
            if new_state.energy
                < visited
                    .get(&new_state.positions)
                    .copied()
                    .unwrap_or(i64::MAX)
            {
                visited.insert(new_state.positions.clone(), new_state.energy);
                bag.push(Reverse(new_state));
            }
        }
    }
}
