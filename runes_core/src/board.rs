use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct Board<const SIZE: usize> {
    fields: [[Field; SIZE]; SIZE],
}

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

impl<const SIZE: usize> Board<SIZE> {
    pub fn new() -> Self {
        Board {
            fields: [[Field::Empty; SIZE]; SIZE],
        }
    }

    pub fn change(&self, pos: Position, symbol: Field) -> Self {
        let mut new = self.clone();
        new.fields[pos.0][pos.1] = symbol;
        return new;
    }

    pub fn fields_around(&self, pos: &Position) -> Vec<Field> {
        let mut around = Vec::with_capacity(8);
        for x in pos.0.max(1) - 1..=pos.0.min(SIZE - 2) + 1 {
            for y in pos.1.max(1) - 1..=pos.1.min(SIZE - 2) + 1 {
                if x == pos.0 && y == pos.1 {
                    continue;
                }
                around.push(self.fields[x][y]);
            }
        }
        return around;
    }

    pub fn is_empty(&self, pos: &Position) -> bool {
        self.fields[pos.0][pos.1] == Field::Empty
    }
}

impl<const SIZE: usize> Debug for Board<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.fields {
            writeln!(f, "{:?}", row)?;
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
                Field::Empty => " ",
                Field::Birth => "ᛒ",
                Field::Gift => "X",
                Field::Wealth => "ᚠ",
                Field::Knowledge => "<",
                Field::Joy => "ᚹ",
            }
        )
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", *self as u8)
        write!(f, "{}",
            match self {
                Field::Empty => " ",
                Field::Birth => "B",
                Field::Gift => "X",
                Field::Wealth => "W",
                Field::Knowledge => "K",
                Field::Joy => "J",
            }
        )
    }
}
