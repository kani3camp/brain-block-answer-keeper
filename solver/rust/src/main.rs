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
    println!("starting calculation...");
    let start_at = std::time::Instant::now();
    let show_log = false;

    let mut remaining_pieces: Vec<PieceShape> = pieces.clone();

    let mut stack = SolveStack::new();

    let mut anchor_i = 0;

    loop {
        if anchor_i >= board.get_anchor_positions().len() {
            println!("アンカーがなくなった");
            break;
        }
        let anchor = board.get_anchor_positions()[anchor_i];

        if show_log {
            println!("anchor: {:?}", anchor);
        }
        if remaining_pieces.is_empty() {
            println!("ピースを使い切った!!");
            if board.is_valid() {
                println!("ボードを埋めた!!");
                break;
            }
            println!("ピースを使い切ったのにボードを埋められなかった");
            break;
        }
        if board.is_position_filled(&anchor) {
            if show_log {
                println!("埋まっているのでスキップ");
            }
            anchor_i += 1;
            continue;
        }
        if stack.target_stack_index().is_none() {
            stack.push(Anchor, anchor_i);
        }
        // PIECE
        let mut remove_piece_i: Option<usize> = None;
        'PIECE: for (piece_shape_i, piece_shape) in remaining_pieces.iter().enumerate() {
            if stack.target_stack_index().is_some() {
                if stack.target_stack_index() == stack.latest_stack_index_of(Piece) {
                    if stack.latest_index_of(Piece) == Some(piece_shape_i) {
                        stack.clear_reset_target_stack_index();
                    } else {
                        continue;
                    }
                } else if stack.latest_index_of(Piece) != Some(piece_shape_i) {
                    continue;
                }
            } else {
                if stack.latest_type() == Some(Piece) {
                    stack.pop().unwrap();
                }
                if stack.latest_type() == Some(Anchor) {
                    stack.push(Piece, piece_shape_i);
                }
            }
            // REVERSED
            for (reversed_i, reversed) in [false, true].iter().enumerate() {
                if stack.target_stack_index().is_some() {
                    if stack.target_stack_index() == stack.latest_stack_index_of(Reversed) {
                        if stack.latest_index_of(Reversed) == Some(reversed_i) {
                            stack.clear_reset_target_stack_index();
                        } else {
                            continue;
                        }
                    } else if stack.latest_index_of(Reversed) != Some(reversed_i) {
                        continue;
                    }
                } else {
                    if stack.latest_type() == Some(Reversed) {
                        stack.pop().unwrap();
                    }
                    if stack.latest_type() == Some(Piece) {
                        stack.push(Reversed, reversed_i);
                    }
                }

                let mut piece = piece_shape.clone();
                if *reversed {
                    piece = piece.reversed();
                }
                // SQUARE
                for (square_i, square) in piece.squares.iter().enumerate() {
                    if stack.target_stack_index().is_some() {
                        if stack.target_stack_index() == stack.latest_stack_index_of(Square) {
                            if stack.latest_index_of(Square) == Some(square_i) {
                                stack.clear_reset_target_stack_index();
                            } else {
                                continue;
                            }
                        } else if stack.latest_index_of(Square) != Some(square_i) {
                            continue;
                        }
                    } else {
                        if stack.latest_type() == Some(Square) {
                            stack.pop().unwrap();
                        }
                        if stack.latest_type() == Some(Reversed) {
                            stack.push(Square, square_i);
                        }
                    }
                    // ROTATE90
                    for num90 in 0..4 {
                        if stack.target_stack_index().is_some() {
                            if stack.target_stack_index() == stack.latest_stack_index_of(Rotate90) {
                                if stack.latest_index_of(Rotate90) == Some(num90) {
                                    stack.clear_reset_target_stack_index();
                                } else {
                                    continue;
                                }
                            } else if stack.latest_index_of(Rotate90) != Some(num90) {
                                continue;
                            }
                        } else {
                            if stack.latest_type() == Some(Rotate90) {
                                stack.pop().unwrap();
                            }
                            if stack.latest_type() == Some(Square) {
                                stack.push(Rotate90, num90);
                            }
                        }
                        let rotated_piece = piece.rotate90(num90 as u8);
                        let shifted_piece =
                            rotated_piece.shift(anchor.0 - square.0, anchor.1 - square.1);
                        if board.is_applicable_piece(&shifted_piece) {
                            if show_log {
                                println!("ハマった！num90: {}, reversed: {}", num90, reversed);
                                println!("{}", shifted_piece.print(Some(&anchor)));
                            }
                            board.push_piece(shifted_piece.clone()).unwrap();
                            remove_piece_i = Some(piece_shape_i);
                            break 'PIECE;
                        }
                    }
                    // 全4方向試して、ダメだった
                    assert_eq!(stack.latest_type(), Some(Rotate90));
                    stack.pop().unwrap();
                    assert_eq!(stack.latest_type(), Some(Square));
                }
                // 全ピースマス試して、ダメだった
                assert_eq!(stack.latest_type(), Some(Square));
                stack.pop().unwrap();
                assert_eq!(stack.latest_type(), Some(Reversed));
            }
            // 表裏試して、ダメだった
            assert_eq!(stack.latest_type(), Some(Reversed));
            stack.pop().unwrap();
            assert_eq!(stack.latest_type(), Some(Piece));
        }
        if let Some(i) = remove_piece_i {
            remaining_pieces.remove(i);
            anchor_i += 1;
        } else {
            // 余ったピース全部試して、ダメだった
            assert_eq!(stack.latest_type(), Some(Piece));
            stack.pop().unwrap();
            assert_eq!(stack.latest_type(), Some(Anchor));

            if show_log {
                println!("ハマるピースがなかったので巻き戻して続きから");
            }

            // １つ前にピースをはめたanchorに戻って続きから試す

            // 今失敗したanchorをpopする
            stack.pop().unwrap();
            assert_eq!(stack.latest_type(), Some(Rotate90));

            'POP: loop {
                // １つ前に置いたピースを元に戻す
                let returned_piece_shape = board.pieces.pop().unwrap();
                remaining_pieces
                    .insert(stack.latest_index_of(Piece).unwrap(), returned_piece_shape);

                anchor_i = stack.latest_index_of(Anchor).unwrap();

                // num90 loop
                let current_num90_index = stack.latest_index_of(Rotate90).unwrap();
                if current_num90_index < 4 - 1 {
                    stack.set_latest_index_of(Rotate90, current_num90_index + 1);
                    stack.set_reset_target_stack_index(
                        stack.latest_stack_index_of(Rotate90).unwrap(),
                    );
                    break 'POP;
                } else {
                    stack.set_latest_index_of(Rotate90, 0);
                }
                // square loop
                let current_square_index = stack.latest_index_of(Square).unwrap();
                let current_piece_index = stack.latest_index_of(Piece).unwrap();
                if current_square_index < remaining_pieces[current_piece_index].squares.len() - 1 {
                    stack.set_latest_index_of(Square, current_square_index + 1);
                    stack
                        .set_reset_target_stack_index(stack.latest_stack_index_of(Square).unwrap());
                    break 'POP;
                } else {
                    stack.set_latest_index_of(Square, 0);
                }
                // reversed loop
                let current_reversed_index = stack.latest_index_of(Reversed).unwrap();
                if current_reversed_index < 2 - 1 {
                    stack.set_latest_index_of(Reversed, current_reversed_index + 1);
                    stack.set_reset_target_stack_index(
                        stack.latest_stack_index_of(Reversed).unwrap(),
                    );
                    break 'POP;
                } else {
                    stack.set_latest_index_of(Reversed, 0);
                }
                // piece loop
                if current_piece_index < remaining_pieces.len() - 1 {
                    stack.set_latest_index_of(Piece, current_piece_index + 1);
                    stack.set_reset_target_stack_index(stack.latest_stack_index_of(Piece).unwrap());
                    break 'POP;
                } else {
                    stack.set_latest_index_of(Piece, 0);
                }

                // ここまでくると、そのanchorにはハマるピースがなかったので、さらに１つ前のanchorに戻る
                assert_eq!(stack.latest_type(), Some(Rotate90));
                stack.pop().unwrap();
                assert_eq!(stack.latest_type(), Some(Square));
                stack.pop().unwrap();
                assert_eq!(stack.latest_type(), Some(Reversed));
                stack.pop().unwrap();
                assert_eq!(stack.latest_type(), Some(Piece));
                stack.pop().unwrap();
                assert_eq!(stack.latest_type(), Some(Anchor));
                stack.pop().unwrap();
                assert_eq!(stack.latest_type(), Some(Rotate90));
            }
        }
    }
    board.print_every_pieces();

    println!("\nremaining_pieces:");
    for piece in remaining_pieces.iter() {
        println!("{}", piece.print(None));
    }

    board.print_filled_squares();

    println!("elapsed: {:?}", start_at.elapsed());
}
