mod piece_shape_test {
    use rust::piece::{Int, PieceShape};

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
    fn test_rotate90() {
        struct TestCase {
            input: PieceShape,
            num90: u8,
            expected: PieceShape,
        }
        let test_cases = [
            TestCase {
                input: PieceShape {
                    squares: vec![(1, 2), (2, 2), (3, 2)],
                },
                num90: 1,
                expected: PieceShape {
                    squares: vec![(1, 1), (1, 2), (1, 3)],
                },
            },
            TestCase {
                input: PieceShape {
                    squares: vec![(1, 1), (2, 1), (2, 2)],
                },
                num90: 2,
                expected: PieceShape {
                    squares: vec![(2, 2), (1, 2), (1, 1)],
                },
            },
            TestCase {
                input: PieceShape {
                    squares: vec![(1, 1), (2, 1), (2, 2)],
                },
                num90: 3,
                expected: PieceShape {
                    squares: vec![(1, 2), (1, 1), (2, 1)],
                },
            },
        ];

        for test_case in test_cases.iter() {
            let actual = test_case.input.rotate90(test_case.num90);
            assert_eq!(actual, test_case.expected);
        }
    }

    #[test]
    fn test_reversed() {
        struct TestCase {
            input: PieceShape,
            expected: PieceShape,
        }
        let test_cases = [
            TestCase {
                input: PieceShape {
                    squares: vec![(1, 2), (2, 2), (3, 2)],
                },
                expected: PieceShape {
                    squares: vec![(3, 1), (2, 1), (1, 1)],
                },
            },
            TestCase {
                input: PieceShape {
                    squares: vec![(1, 1), (2, 1), (2, 2)],
                },
                expected: PieceShape {
                    squares: vec![(2, 1), (1, 1), (1, 2)],
                },
            },
        ];

        for test_case in test_cases.iter() {
            let actual = test_case.input.reversed();
            assert_eq!(actual, test_case.expected);
        }
    }

    #[test]
    fn test_shift() {
        struct TestCase {
            input: PieceShape,
            shift_x: Int,
            shift_y: Int,
            expected: PieceShape,
        }
        let test_cases = [
            TestCase {
                input: PieceShape {
                    squares: vec![(1, 2), (2, 2), (3, 2)],
                },
                shift_x: 1,
                shift_y: 1,
                expected: PieceShape {
                    squares: vec![(2, 3), (3, 3), (4, 3)],
                },
            },
            TestCase {
                input: PieceShape {
                    squares: vec![(1, 2), (2, 2), (3, 2), (4, 2), (4, 1)],
                },
                shift_x: -2,
                shift_y: -3,
                expected: PieceShape {
                    squares: vec![(-1, -1), (0, -1), (1, -1), (2, -1), (2, -2)],
                },
            },
        ];

        for test_case in test_cases.iter() {
            let actual = test_case.input.shift(test_case.shift_x, test_case.shift_y);
            assert_eq!(actual, test_case.expected);
        }
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
