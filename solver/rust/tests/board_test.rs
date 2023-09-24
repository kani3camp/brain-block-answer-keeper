mod board_test {
    use rust::board::{Board, BoardShape};
    use rust::piece::PieceShape;

    #[test]
    fn test_board() {
        let board_shape = BoardShape::new(5, 5);
        let board = Board::new(board_shape, vec![]);
        assert_eq!(board.shape.height, 5);
        assert_eq!(board.shape.width, 5);
    }

    #[test]
    fn test_push_piece() {
        let board_shape = BoardShape::new(5, 5);
        let mut board = Board::new(board_shape, vec![]);
        let piece_shape = PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (4, 1)],
        };
        board.push_piece(piece_shape).unwrap();
        assert_eq!(board.pieces.len(), 1);
    }

    #[test]
    fn test_is_applicable_piece() {
        let board_shape = BoardShape::new(5, 5);
        let mut board = Board::new(board_shape, vec![]);
        let piece_shape = PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (4, 1)],
        };
        assert!(board.is_applicable_piece(&piece_shape));
        board.push_piece(piece_shape).unwrap();
        let piece_shape = PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (3, 1), (4, 1)],
        };
        assert!(!board.is_applicable_piece(&piece_shape));
    }

    #[test]
    fn test_is_position_filled() {
        let board_shape = BoardShape::new(5, 5);
        let mut board = Board::new(board_shape, vec![]);
        let piece_shape = PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (4, 1)],
        };
        board.push_piece(piece_shape).unwrap();
        assert!(board.is_position_filled(&(1, 2)));
        assert!(!board.is_position_filled(&(1, 1)));
    }

    #[test]
    fn test_is_valid() {
        let board_shape = BoardShape::new(5, 5);
        let mut board = Board::new(board_shape, vec![]);
        let piece_shape = PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (4, 1)],
        };
        let result = board.push_piece(piece_shape);
        assert!(result.is_ok());
        assert!(board.is_valid());
        let piece_shape = PieceShape {
            squares: vec![(1, 3), (2, 3), (2, 4), (3, 4), (4, 4)],
        };
        let result = board.push_piece(piece_shape);
        assert!(result.is_ok());
        assert!(board.is_valid());
    }

    #[test]
    fn test_get_anchor_positions() {
        let board_shape = BoardShape::new(5, 5);
        let board = Board::new(board_shape, vec![]);
        assert_eq!(board.get_anchor_positions().len(), 25);
    }

    #[test]
    fn test_is_filled() {
        let board_shape = BoardShape::new(5, 5);
        let mut board = Board::new(board_shape, vec![]);
        let piece_shape = PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (4, 1)],
        };
        board.push_piece(piece_shape).unwrap();
        assert!(board.is_filled());
        let piece_shape = PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (3, 1), (4, 1)],
        };
        board.push_piece(piece_shape).unwrap();
        assert!(!board.is_filled());
    }
}
