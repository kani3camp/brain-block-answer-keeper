/*!
ピース関連の定義をまとめたモジュール。
*/

pub type Int = i16;
pub type SquarePosition = (Int, Int);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PieceShape {
    pub squares: Vec<SquarePosition>,
}

trait YShadow {
    fn y_shadow(&self) -> SquarePosition;
}

impl YShadow for SquarePosition {
    fn y_shadow(&self) -> SquarePosition {
        (-self.0, self.1)
    }
}

impl PieceShape {
    /// 裏返しにする。y軸対称で。
    /// 最後に全マスが第1象限におさまるようにシフトするため、各マスの相対的な位置のみ保証される。
    pub fn get_back(&mut self) {
        let mut backed_squares: Vec<SquarePosition> = Vec::new();
        for square in self.squares.iter() {
            backed_squares.push(square.y_shadow())
        }
        self.squares = backed_squares;
    }

    pub fn print(&self, anchor: Option<&SquarePosition>) -> String {
        if self.squares.is_empty() {
            return String::from("empty");
        }
        let mut result: String = String::new();
        for y in self.min_y()..=self.max_y() {
            for x in self.min_x()..=self.max_x() {
                if self.squares.contains(&(x, y)) {
                    if anchor.is_some() && anchor.unwrap() == &(x, y) {
                        result.push('A');
                    } else {
                        result.push('O');
                    }
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }
        result
    }

    pub fn print_left_bottom_position(&self) -> String {
        if self.squares.is_empty() {
            return String::from("");
        }
        format!("({}, {})", self.min_x(), self.min_y())
    }

    pub fn min_x(&self) -> Int {
        if self.squares.is_empty() {
            return 0;
        }
        let mut min: Int = self.squares[0].0;
        for square in self.squares.iter() {
            if square.0 < min {
                min = square.0;
            }
        }
        min
    }

    pub fn max_x(&self) -> Int {
        if self.squares.is_empty() {
            return 0;
        }
        let mut max: Int = self.squares[0].0;
        for square in self.squares.iter() {
            if square.0 > max {
                max = square.0;
            }
        }
        max
    }

    pub fn min_y(&self) -> Int {
        if self.squares.is_empty() {
            return 0;
        }
        let mut min: Int = self.squares[0].1;
        for square in self.squares.iter() {
            if square.1 < min {
                min = square.1;
            }
        }
        min
    }

    pub fn max_y(&self) -> Int {
        if self.squares.is_empty() {
            return 0;
        }
        let mut max: Int = self.squares[0].1;
        for square in self.squares.iter() {
            if square.1 > max {
                max = square.1;
            }
        }
        max
    }

    /// ピースの形状を90度回転させる。
    /// 回転行列を使って座標を変換し、全マスが第1象限におさまるようにシフトする。
    /// そのため、各マスの相対的な位置のみ保証される。回転の軸となるマスは考慮されない。
    pub fn rotate90(&self, num90: u8) -> PieceShape {
        let mut piece_shape = self.clone();
        assert!((0..4).contains(&num90));
        for _ in 0..num90 {
            for square in piece_shape.squares.iter_mut() {
                (square.0, square.1) = (-square.1, square.0);
            }
            piece_shape = piece_shape.shift_to_quadrant1();
        }
        piece_shape
    }

    pub fn reversed(&self) -> PieceShape {
        let mut reversed_shape = self.clone();
        for square in reversed_shape.squares.iter_mut() {
            square.0 = -square.0;
        }
        reversed_shape.shift_to_quadrant1()
    }

    pub fn shift(&self, x: Int, y: Int) -> PieceShape {
        let mut piece_shape = self.clone();
        for square in piece_shape.squares.iter_mut() {
            square.0 += x;
            square.1 += y;
        }
        piece_shape
    }

    pub fn shift_to_quadrant1(&self) -> PieceShape {
        let mut piece_shape = self.clone();
        let min_x = piece_shape.min_x();
        let min_y = piece_shape.min_y();
        if min_x > 0 && min_y > 0 {
            return piece_shape;
        }
        for square in piece_shape.squares.iter_mut() {
            square.0 += 1 - min_x;
            square.1 += 1 - min_y;
        }
        piece_shape
    }
}

pub trait Countable {
    fn count_squares(&self) -> usize;
}

impl Countable for Vec<PieceShape> {
    fn count_squares(&self) -> usize {
        let mut result: usize = 0;
        for piece in self.iter() {
            result += piece.squares.len();
        }
        result
    }
}
