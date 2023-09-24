/*!
ボードの状態を表すモジュールをまとめたモジュール。
*/

use crate::piece::{Int, PieceShape, SquarePosition};

pub struct BoardShape {
    pub height: Int,
    pub width: Int,
}

impl BoardShape {
    pub fn new(height: Int, width: Int) -> BoardShape {
        BoardShape { height, width }
    }
}

pub struct Board {
    pub shape: BoardShape,
    pub pieces: Vec<PieceShape>,
}

impl Board {
    pub fn new(shape: BoardShape, pieces: Vec<PieceShape>) -> Board {
        Board { shape, pieces }
    }

    pub fn print_every_pieces(&self) {
        for (i, piece) in self.pieces.iter().enumerate() {
            println!(
                "{}つ目: {}\n{}",
                i + 1,
                piece.print_left_bottom_position(),
                piece.print(None)
            );
        }
    }

    pub fn print_filled_squares(&self) {
        let filled_area = self.filled_squares();
        for y in 1..=self.shape.height {
            for x in 1..=self.shape.width {
                if filled_area.contains(&(x, y)) {
                    print!("O");
                } else {
                    print!("X");
                }
            }
            println!();
        }
    }

    pub fn push_piece(&mut self, piece: PieceShape) -> Result<(), String> {
        if !self.is_applicable_piece(&piece) {
            return Err("Piece is not applicable".to_string());
        }

        self.pieces.push(piece);
        Ok(())
    }

    fn filled_squares(&self) -> Vec<SquarePosition> {
        let mut square_list: Vec<SquarePosition> = Vec::new();
        for piece in self.pieces.iter() {
            for square in piece.squares.iter() {
                square_list.push((square.0, square.1));
            }
        }
        square_list
    }

    pub fn is_applicable_piece(&self, piece: &PieceShape) -> bool {
        let filled_area = self.filled_squares();
        for square in piece.squares.iter() {
            // ピースが既存のピースと重なっていないか
            if filled_area.contains(square) {
                return false;
            }
            // ピースがボードの外にはみ出していないか
            if square.0 < 1
                || square.0 > self.shape.width
                || square.1 < 1
                || square.1 > self.shape.height
            {
                return false;
            }
        }
        true
    }

    pub fn is_position_filled(&self, position: &SquarePosition) -> bool {
        let filled_area = self.filled_squares();
        filled_area.contains(position)
    }

    pub fn is_valid(&self) -> bool {
        let filled_area = self.filled_squares();

        // filled_areaがボード内に収まっているか
        for square in filled_area.iter() {
            if square.0 < 1
                || square.0 > self.shape.width
                || square.1 < 1
                || square.1 > self.shape.height
            {
                return false;
            }
        }

        // ピース同士が重なっていないか
        for i in 0..filled_area.len() {
            for j in i + 1..filled_area.len() {
                if filled_area[i] == filled_area[j] {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_anchor_positions(&self) -> Vec<SquarePosition> {
        let mut anchor_positions: Vec<SquarePosition> = Vec::new();
        for y in 1..=self.shape.height {
            for x in 1..=self.shape.width {
                anchor_positions.push((x, y));
            }
        }
        anchor_positions
    }

    pub fn is_filled(&self) -> bool {
        let filled_area = self.filled_squares();
        filled_area.len() == self.shape.height as usize * self.shape.width as usize
        // TODO: もっとまじめにチェック
    }
}
