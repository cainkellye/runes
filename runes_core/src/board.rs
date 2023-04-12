use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct Board {
    fields: Vec<Field>,
    pub size: usize,
}

#[derive(Clone, Copy)]
pub struct Position(pub usize, pub usize);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
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
        return around;
    }

    pub fn is_empty(&self, pos: &Position) -> bool {
        self.fields[pos.0 * self.size + pos.1] == Field::Empty
    }

    pub fn is_joy(&self, pos: &Position) -> bool {
        self.fields[pos.0 * self.size + pos.1] == Field::Joy
    }

    pub fn reset(&mut self) {
        self.fields = vec![Field::Empty; self.size * self.size];
    }

    pub fn is_full(&self) -> bool {
        self.fields.iter().all(|&f| f != Field::Empty)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size {
            write!(f, "[")?;
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
        use colorize::AnsiColor;
        write!(
            f,
            "{}",
            match self {
                Field::Empty => " ".to_string(),
                Field::Birth => "ᛒ".to_string(),
                Field::Gift => "X".b_green(),
                Field::Wealth => "ᚠ".yellow(),
                Field::Knowledge => "<".blue(),
                Field::Joy => "ᚹ".red(),
            }
        )
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colorize::AnsiColor;
        //write!(f, "{}", *self as u8)
        write!(
            f,
            "{}",
            match self {
                Field::Empty => " ".to_string(),
                Field::Birth => "B".to_string(),
                Field::Gift => "X".b_green(),
                Field::Wealth => "W".yellow(),
                Field::Knowledge => "K".blue(),
                Field::Joy => "J".red(),
            }
        )
    }
}
