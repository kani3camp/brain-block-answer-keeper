mod piece_shape_test {
    use rust::piece::PieceShape;

    #[test]
    fn test_piece_shape() {
        let piece_shape = PieceShape {
            squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (4, 1)],
        };
        assert_eq!(piece_shape.min_x(), 1);
        assert_eq!(piece_shape.max_x(), 4);
        assert_eq!(piece_shape.min_y(), 1);
        assert_eq!(piece_shape.max_y(), 2);
    }

    #[test]
    fn test_shift_to_quadrant1() {
        struct TestCase {
            input: PieceShape,
            expected: PieceShape,
        }
        let test_cases = [
            TestCase {
                input: PieceShape {
                    squares: vec![(-1, 1), (0, 1)],
                },
                expected: PieceShape {
                    squares: vec![(1, 1), (2, 1)],
                },
            },
            TestCase {
                input: PieceShape {
                    squares: vec![(-1, 1), (0, 1), (1, 1)],
                },
                expected: PieceShape {
                    squares: vec![(1, 1), (2, 1), (3, 1)],
                },
            },
        ];

        for test_case in test_cases.iter() {
            let actual = test_case.input.shift_to_quadrant1();
            assert_eq!(actual, test_case.expected);
        }
    }
}
