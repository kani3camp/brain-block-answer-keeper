use rust::board::{Board, BoardShape};
use rust::piece::{Countable, PieceShape};
use rust::stack::LoopType::{Anchor, Piece, Reversed, Rotate90, Square};
use rust::stack::SolveStack;

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
        println!("{}", piece.print(None));
    }

    println!("全ピースの合計マス数: {}", pieces.to_vec().count_squares());

    let board_shape = BoardShape::new(10, 6);
    let board = Board::new(board_shape, Vec::new());

    solve(board, pieces.to_vec());
}

fn solve(mut board: Board, pieces: Vec<PieceShape>) {
    let mut remaining_pieces: Vec<PieceShape> = pieces.clone();

    let mut stack = SolveStack::new();
    let mut resetting = false;

    let mut anchor_i = 0;

    loop {
        if anchor_i >= board.get_anchor_positions().len() {
            println!("アンカーがなくなった");
            board.print();
            return;
        }
        let anchor = board.get_anchor_positions()[anchor_i];

        stack.push(Anchor, anchor_i);
        println!("anchor: {:?}", anchor);
        if remaining_pieces.is_empty() {
            println!("ピースを使い切った!!");
            if board.is_valid() {
                println!("ボードを埋めた!!");
                board.print();
                break;
            }
            println!("ピースを使い切ったのにボードを埋められなかった");
            board.print();
            break;
        }
        if board.is_position_filled(&anchor) {
            println!("埋まっているのでスキップ");
            anchor_i += 1;
            continue;
        }

        let mut remove_piece_i: Option<usize> = None;
        'piece_choice: for (piece_shape_i, piece_shape) in remaining_pieces.iter().enumerate() {
            if resetting {
                if piece_shape_i < stack.latest_index_of(Piece).unwrap() {
                    continue;
                } else {
                    resetting = false;
                }
            }
            if piece_shape_i > 0 {
                let result = stack.pop();
                if result.is_err() {
                    panic!("スタックが空になった");
                }
            }
            stack.push(Piece, piece_shape_i);

            // TRY
            for (reversed_i, reversed) in [false, true].iter().enumerate() {
                if resetting {
                    if reversed_i < stack.latest_index_of(Reversed).unwrap() {
                        continue;
                    } else {
                        resetting = false;
                    }
                }
                if reversed_i > 0 {
                    let result = stack.pop();
                    if result.is_err() {
                        panic!("スタックが空になった");
                    }
                }
                stack.push(Reversed, reversed_i);

                let mut piece = piece_shape.clone();
                if *reversed {
                    piece = piece.reversed();
                }
                for (square_i, square) in piece.squares.iter().enumerate() {
                    if resetting {
                        if square_i < stack.latest_index_of(Square).unwrap() {
                            continue;
                        } else {
                            resetting = false;
                        }
                    }
                    if square_i > 0 {
                        let result = stack.pop();
                        if result.is_err() {
                            panic!("スタックが空になった");
                        }
                    }
                    stack.push(Square, square_i);

                    for num90 in 0..4 {
                        if resetting && num90 < stack.latest_index_of(Rotate90).unwrap() {
                            continue;
                        }
                        if num90 > 0 {
                            let result = stack.pop();
                            if result.is_err() {
                                panic!("スタックが空になった");
                            }
                        }
                        stack.push(Rotate90, num90);
                        let rotated_piece = piece.rotate90(num90 as u8);
                        let shifted_piece =
                            rotated_piece.shift(anchor.0 - square.0, anchor.1 - square.1);
                        if board.is_applicable_piece(&shifted_piece) {
                            println!("ハマった！num90: {}, reversed: {}", num90, reversed);
                            let applicable = board.is_applicable_piece(&shifted_piece);
                            println!("{}", shifted_piece.print(Some(&anchor)));
                            let result = board.push_piece(shifted_piece.clone());
                            if result.is_ok() {
                                remove_piece_i = Some(piece_shape_i);
                                break 'piece_choice;
                            } else {
                                panic!("ボードにピースを追加できなかった");
                            }
                        }
                    }
                    // 全4方向試して、ダメだった
                }
                // 全ピースマス試して、ダメだった
            }
            // 表裏試して、ダメだった
        }
        if let Some(i) = remove_piece_i {
            remaining_pieces.remove(i);
            anchor_i += 1;
        } else {
            // 余ったピース全部試して、ダメだった
            println!("ハマるピースがなかったので巻き戻して続きから");
            // １つ前のanchorに戻って続きから試す
            // 1つポップして、次のインデックスを試す
            let result = stack.pop();
            if result.is_err() {
                panic!("スタックが空になった");
            }

            board.pieces.pop();
            anchor_i -= 1;
            resetting = true;

            // num90 loop
            let current_num90_index = stack.latest_index_of(Rotate90).unwrap();
            if current_num90_index < 4 - 1 {
                stack.set_latest_index_of(Rotate90, current_num90_index + 1);
                continue;
            }
            // square loop
            let current_square_index = stack.latest_index_of(Square).unwrap();
            let current_piece_index = stack.latest_index_of(Piece).unwrap();
            if current_square_index < remaining_pieces[current_piece_index].squares.len() - 1 {
                stack.set_latest_index_of(Square, current_square_index + 1);
                continue;
            }
            // reversed loop
            let current_reversed_index = stack.latest_index_of(Reversed).unwrap();
            if current_reversed_index < 2 - 1 {
                stack.set_latest_index_of(Reversed, current_reversed_index + 1);
                continue;
            }
            // piece loop
            if current_piece_index < remaining_pieces.len() - 1 {
                stack.set_latest_index_of(Piece, current_piece_index + 1);
                continue;
            }
        }
    }
    println!("\nremaining_pieces:");
    for piece in remaining_pieces.iter() {
        println!("{}", piece.print(None));
    }

    board.print_filled_squares();
}
