use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Board<const SIZE: usize> {
    fields: [[Field; SIZE]; SIZE],
}

pub struct Position(pub usize, pub usize);

#[derive(Clone, Copy)]
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

    pub fn fields_around(&self, pos: Position) -> Vec<Field> {
        let mut around = Vec::new();
        for x in (pos.0 - 1).max(0)..=(pos.0 + 1).min(SIZE) {
            for y in (pos.1 - 1).max(0)..=(pos.1 + 1).min(SIZE) {
                around.push(self.fields[x][y]);
            }
        }
        return around;
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

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Field::Empty => " ",
            Field::Birth => "ᛒ",
            Field::Gift => "X",
            Field::Wealth => "ᚠ",
            Field::Knowledge => "<",
            Field::Joy => "ᚹ",
        })
    }
}