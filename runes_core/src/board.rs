use std::fmt::{Debug, Display};
use colorize::AnsiColor;

#[derive(Clone)]
pub struct Board {
    fields: Vec<Field>,
    pub size: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Position(pub usize, pub usize);
impl Position {
    pub fn near(&self, other: &Position) -> bool {
        self.0.abs_diff(other.0) < 2 && self.1.abs_diff(other.1) < 2
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum Field {
    Empty = 0,
    Birth = 1,
    Gift = 2,
    Wealth = 3,
    Knowledge = 4,
    Joy = 5,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            fields: vec![Field::Empty; size * size],
            size,
        }
    }

    pub fn change(&mut self, pos: Position, symbol: Field) {
        self.fields[pos.0 * self.size + pos.1] = symbol;
    }

    pub fn fields_around(&self, pos: &Position) -> Vec<Field> {
        let mut around = Vec::with_capacity(8);
        for x in pos.0.max(1) - 1..=pos.0.min(self.size - 2) + 1 {
            for y in pos.1.max(1) - 1..=pos.1.min(self.size - 2) + 1 {
                if x == pos.0 && y == pos.1 {
                    continue;
                }
                around.push(self.fields[x * self.size + y]);
            }
        }
        around
    }

    /// returns (empty_count, birth_count, gift_count, wealth_count, knowledge_count)
    pub fn count_around(&self, pos: &Position) -> (u8, u8, u8, u8, u8) {
        let mut empty_count = 0;
        let mut birth_count = 0;
        let mut gift_count = 0;
        let mut wealth_count = 0;
        let mut knowledge_count = 0;

        for field in self.fields_around(pos) {
            match field {
                Field::Empty => empty_count += 1,
                Field::Birth => birth_count += 1,
                Field::Gift => gift_count += 1,
                Field::Wealth => wealth_count += 1,
                Field::Knowledge => knowledge_count += 1,
                Field::Joy => (),
            }
        }
        (
            empty_count,
            birth_count,
            gift_count,
            wealth_count,
            knowledge_count,
        )
    }

    pub fn is_empty(&self, pos: &Position) -> bool {
        self.fields[pos.0 * self.size + pos.1] == Field::Empty
    }

    pub fn reset(&mut self) {
        self.fields = vec![Field::Empty; self.size * self.size];
    }

    pub fn is_full(&self) -> bool {
        self.fields.iter().all(|&f| f != Field::Empty)
    }

    pub fn field_at(&self, pos: &Position) -> Field {
        self.fields[pos.0 * self.size + pos.1]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Header
        write!(f, "    ")?;
        for i in 0..self.size {
            write!(
                f,
                "{}",
                if i % 2 == 0 {
                    format!("{:<6}", i + 1).bold().cyan()
                } else {
                    String::new()
                }
            )?;
        }
        writeln!(f)?;
        //Board
        for i in 0..self.size {
            write!(
                f,
                "{} [ ",
                if i % 2 == 0 {
                    format!("{:2}", i + 1).bold().cyan()
                } else {
                    "  ".to_string()
                }
            )?;
            for j in 0..self.size {
                write!(f, "{:?}", self.fields[i * self.size + j])?;
                if j < self.size - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Field::Empty => "Empty".to_string(),
                Field::Birth => "Berkana ᛒ".to_string(),
                Field::Gift => "Gebo X".b_green(),
                Field::Wealth => "Fehu ᚠ".yellow(),
                Field::Knowledge => "Kano <".blue(),
                Field::Joy => "Wunjo ᚹ".red(),
            }
        )
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", *self as u8)
        write!(
            f,
            "{}",
            match self {
                Field::Empty => " ".to_string(),
                Field::Birth => "B".to_string(),
                Field::Gift => "X".green(),
                Field::Wealth => "W".yellow(),
                Field::Knowledge => "K".blue(),
                Field::Joy => "J".b_redb(),
            }
        )
    }
}
