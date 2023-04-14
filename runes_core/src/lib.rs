pub mod ai_player;
pub mod board;
pub mod game;
pub mod human_player;

#[cfg(test)]
mod tests {
    use crate::board::{Board, Field, Position};

    #[test]
    #[rustfmt::skip]
    fn board() {
        let mut b = Board::new(5);
        b.change(Position(2, 2), Field::Birth);
        b.change(Position(1, 3), Field::Gift);
        b.change(Position(3, 1), Field::Gift);
        b.change(Position(3, 2), Field::Knowledge);
        b.change(Position(2, 3), Field::Wealth);
//         assert_eq!(format!("{:?}", b).trim_end(), 
// "[ ,  ,  ,  ,  ]
// [ ,  ,  , X,  ]
// [ ,  , B, W,  ]
// [ , X, K,  ,  ]
// [ ,  ,  ,  ,  ]");

        assert_eq!(
            format!("{:?}", b.fields_around(&Position(1, 2))).trim_end(),
            "[ ,  ,  ,  , X,  , B, W]"
        );
        assert_eq!(
            format!("{:?}", b.fields_around(&Position(4, 0))).trim_end(),
            "[ , X,  ]"
        );
        assert_eq!(
            format!("{:?}", b.fields_around(&Position(1, 4))).trim_end(),
            "[ ,  , X, W,  ]"
        );
    }
}
