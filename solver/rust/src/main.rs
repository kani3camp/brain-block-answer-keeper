use rust::board::{Board, BoardShape};
use rust::piece::{Countable, PieceShape};

fn main() {
    let pieces: [PieceShape; 12] = [
        // □□□
        //  □
        //  □
        PieceShape {
            squares: vec![(1, 3), (2, 3), (3, 3), (2, 2), (2, 1)],
        },
        // □□□
        // □
        // □
        PieceShape {
            squares: vec![(1, 3), (2, 3), (3, 3), (1, 2), (1, 1)],
        },
        //
        // □□□□□
        //
        PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (5, 2)],
        },
        //  □□
        // □□
        // □
        PieceShape {
            squares: vec![(2, 3), (3, 3), (1, 2), (2, 2), (1, 1)],
        },
        //
        // □□□□
        //    □
        PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (4, 1)],
        },
        //
        // □□□
        //   □□
        PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (3, 1), (4, 1)],
        },
        //  □
        // □□□
        //   □
        PieceShape {
            squares: vec![(2, 3), (1, 2), (2, 2), (3, 2), (3, 1)],
        },
        //  □
        // □□□
        //  □
        PieceShape {
            squares: vec![(2, 3), (1, 2), (2, 2), (3, 2), (2, 1)],
        },
        // □□
        //  □
        // □□
        PieceShape {
            squares: vec![(1, 3), (2, 3), (2, 2), (1, 1), (2, 1)],
        },
        //
        // □□□□
        //   □
        PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (3, 1)],
        },
        // □□
        //  □
        //  □□
        PieceShape {
            squares: vec![(1, 3), (2, 3), (2, 2), (2, 1), (3, 1)],
        },
        // □□
        // □□
        //  □
        PieceShape {
            squares: vec![(1, 3), (2, 3), (1, 2), (2, 2), (2, 1)],
        },
    ];

    for piece in pieces.iter() {
        println!("{}", piece.print());
    }

    println!("全ピースの合計マス数: {}", pieces.to_vec().count_squares());

    let board_shape = BoardShape::new(10, 6);
    let board = Board::new(board_shape, Vec::new());

    solve(board, pieces.to_vec());
}

fn solve(mut board: Board, pieces: Vec<PieceShape>) {
    let mut remaining_pieces: Vec<PieceShape> = pieces.clone();
    for anchor in board.get_anchor_positions() {
        println!("anchor: {:?}", anchor);
        if remaining_pieces.is_empty() {
            println!("ピースを使い切った!!");
            if board.is_valid() {
                println!("ボードを埋めた!!");
            }
            println!("ピースを使い切ったのにボードを埋められなかった");
            board.print();
            return;
        }
        if board.is_position_filled(&anchor) {
            println!("埋まっているのでスキップ");
            continue;
        }
        let mut remove_piece_i: Option<usize> = None;
        'piece_choice: for (i, piece_shape) in remaining_pieces.iter().enumerate() {
            // TRY
            for reversed in [false, true] {
                let mut piece = piece_shape.clone();
                if reversed {
                    piece = piece.reversed();
                }
                for square in piece.squares.iter() {
                    for num90 in 0..4 {
                        let rotated_piece = piece.rotate90(num90);
                        let shifted_piece =
                            rotated_piece.shift(anchor.0 - square.0, anchor.1 - square.1);
                        if board.is_applicable_piece(&shifted_piece) {
                            println!("ハマった！num90: {}, reversed: {}", num90, reversed);
                            println!("{}", shifted_piece.print());
                            let result = board.push_piece(shifted_piece.clone());
                            if result.is_ok() {
                                remove_piece_i = Some(i);
                                break 'piece_choice;
                            }
                        }
                    }
                }
            }
        }
        if let Some(i) = remove_piece_i {
            remaining_pieces.remove(i);
        } else {
            println!("ハマるピースがなかった");
            board.print();
            return;
            // TODO: １つ前のanchorに戻って続きから試す
        }
    }
    println!("\nremaining_pieces:");
    for piece in remaining_pieces.iter() {
        println!("{}", piece.print());
    }

    board.print_filled_squares();
}
