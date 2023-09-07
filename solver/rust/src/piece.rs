
type Pixel = i16;
pub type SquarePosition = (Pixel, Pixel);

#[derive(Debug)]
pub struct PieceShape {
    pub squares: Vec<SquarePosition>,
}

trait YShadow {
    fn shadow(&self) -> SquarePosition;
}

impl YShadow for SquarePosition {
    fn shadow(&self) -> SquarePosition {
        return (-self.0, -self.1);
    }
}



impl PieceShape {
    //! とりあえずy軸対称で
    pub fn get_back(&self) -> PieceShape {
        let mut backed_squares: Vec<SquarePosition> = Vec::new();
        for square in self.squares.iter() {
            backed_squares.push(square.shadow())
        }
        PieceShape{
            squares: backed_squares,
        }
    }

    pub fn print(&self) -> String {
        if self.squares.len() == 0 {
            return String::from("empty");
        }
        let mut result: String = String::new();
        let mut min_x: Pixel = self.squares[0].0;
        let mut max_x: Pixel = self.squares[0].0;
        let mut min_y: Pixel = self.squares[0].1;
        let mut max_y: Pixel = self.squares[0].1;
        for square in self.squares.iter() {
            if square.0 < min_x {
                min_x = square.0;
            }
            if square.0 > max_x {
                max_x = square.0;
            }
            if square.1 < min_y {
                min_y = square.1;
            }
            if square.1 > max_y {
                max_y = square.1;
            }
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.squares.contains(&(x, y)) {
                    result.push_str("▢");
                } else {
                    result.push_str("　");
                }
            }
            result.push_str("\n");
        }
        result
    }

    pub fn min_x(&self) -> Pixel {
        if self.squares.len() == 0 {
            return 0;
        }
        let mut min: Pixel = self.squares[0].0;
        for square in self.squares.iter() {
            if square.0 < min {
                min = square.0;
            }
        }
        min
    }

    pub fn max_x(&self) -> Pixel {
        if self.squares.len() == 0 {
            return 0;
        }
        let mut max: Pixel = self.squares[0].0;
        for square in self.squares.iter() {
            if square.0 > max {
                max = square.0;
            }
        }
        max
    }

    pub fn min_y(&self) -> Pixel {
        if self.squares.len() == 0 {
            return 0;
        }
        let mut min: Pixel = self.squares[0].1;
        for square in self.squares.iter() {
            if square.1 < min {
                min = square.1;
            }
        }
        min
    }

    pub fn max_y(&self) -> Pixel {
        if self.squares.len() == 0 {
            return 0;
        }
        let mut max: Pixel = self.squares[0].1;
        for square in self.squares.iter() {
            if square.1 > max {
                max = square.1;
            }
        }
        max
    }
}

pub struct PieceWithPosition {
    pub piece: PieceShape,
    pub position: SquarePosition,
}
