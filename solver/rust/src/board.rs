use crate::piece::{PieceWithPosition, SquarePosition};

pub struct BoardShape {
    pub height: i32,
    pub width: i32,
}

impl BoardShape {
    pub fn new(height: i32, width: i32) -> BoardShape {
        BoardShape {
            height,
            width,
        }
    }
}

pub struct Board {
    pub shape: BoardShape,
    pub pieces: Vec<PieceWithPosition>,
}

impl Board {
    pub fn new(shape: BoardShape, pieces: Vec<PieceWithPosition>) -> Board {
        Board {
            shape,
            pieces,
        }
    }

    pub fn push_piece(&mut self, piece: PieceWithPosition) -> Result<(), String> {
        // check


        self.pieces.push(piece);
    }

    fn filled_squares(&self) -> Vec<SquarePosition> {
        let mut result: Vec<SquarePosition> = Vec::new();
        for piece in self.pieces.iter() {
            for square in piece.piece.squares.iter() {
                result.push((square.0 + piece.x, square.1 + piece.y));
            }
        }
        result
    }

    fn is_valid(&self, piece: &PieceWithPosition) -> bool {
        for piece in self.pieces.iter() {
            if !self.is_valid_piece(piece) {
                return false;
            }
        }
        true
    }
}