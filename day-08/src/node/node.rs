#[derive(Clone, Copy)]
pub struct Node {
    pub id: usize,
    pub fork: Fork,
}

#[derive(Clone, Copy)]
pub struct Fork {
    pub left: usize,
    pub right: usize,
}

impl Fork {
    pub fn go(&self, direction: &Direction) -> usize {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

pub enum Direction {
    Left,
    Right,
}

impl Node {
    pub fn parse(input: &str) -> Self {
        let id = hash(&input[0..3]);
        let left = hash(&input[7..10]);
        let right = hash(&input[12..15]);
        Node {
            id,
            fork: Fork { left, right },
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = ({}, {})",
            reverse_hash(self.id),
            reverse_hash(self.fork.left),
            reverse_hash(self.fork.right)
        )
    }
}

pub fn hash(input: &str) -> usize {
    let mut value: usize = 0;

    for (i, char) in input.chars().enumerate() {
        value |= (char as usize - 'A' as usize) << (2 - i) * 5;
    }

    value
}

pub fn reverse_hash(input: usize) -> String {
    let mut string: String = String::from("");
    for i in (0..3).rev() {
        let current = ((((input & 0b11111 << i * 5) >> i * 5) + 'A' as usize) as u8) as char;
        string.push(current);
    }

    string
}
