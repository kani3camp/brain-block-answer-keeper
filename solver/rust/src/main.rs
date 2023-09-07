use crate::board::BoardShape;
use crate::piece::PieceShape;

mod piece;
mod board;


fn main() {
    let pieces: [PieceShape; 12] = [
        // □□□
        //  □
        //  □
        PieceShape{
            squares: vec![(1, 1), (2, 1), (3, 1), (2, 2), (2, 3)],
        },

        // □□□
        // □
        // □
        PieceShape{
            squares: vec![(1, 1), (2, 1), (3, 1), (1, 2), (1, 3)],
        },

        //
        // □□□□□
        //
        PieceShape{
            squares: vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)],
        },

        //  □□
        // □□
        // □
        PieceShape{
            squares: vec![(2, 1), (3, 1), (1, 2), (2, 2), (1, 3)],
        },

        //
        // □□□□
        //    □
        PieceShape{
            squares: vec![(1, 1), (2, 1), (3, 1), (4, 1), (4, 2)],
        },

        //
        // □□□
        //   □□
        PieceShape{
            squares: vec![(1, 1), (2, 1), (3, 1), (3, 2), (4, 2)],
        },

        //  □
        // □□□
        //   □
        PieceShape{
            squares: vec![(2, 1), (1, 2), (2, 2), (3, 2), (3, 3)],
        },

        //  □
        // □□□
        //  □
        PieceShape{
            squares: vec![(2, 1), (1, 2), (2, 2), (3, 2), (2, 3)],
        },

        // □□
        //  □
        // □□
        PieceShape{
            squares: vec![(1, 1), (2, 1), (2, 2), (1, 3), (2, 3)],
        },

        //
        // □□□□
        //   □
        PieceShape{
            squares: vec![(1, 1), (2, 1), (3, 1), (4, 1), (3, 2)],
        },

        // □□
        //  □
        //  □□
        PieceShape{
            squares: vec![(1, 1), (2, 1), (2, 2), (2, 3), (3, 3)],
        },

        // □□
        // □□
        //  □
        PieceShape{
            squares: vec![(1, 1), (2, 1), (1, 2), (2, 2), (2, 3)],
        },
    ];

    for piece in pieces.iter() {
        println!("{}", piece.print());
    }

    let board = BoardShape::new(10, 6);

    solve(board, pieces.to_vec());
}

fn solve(board: BoardShape, pieces: Vec<PieceShape>) {

}
