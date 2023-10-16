#[derive(PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn matrix(n: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; n]; n];

    let mut current_column = 0;
    let mut current_row = 0;
    let mut direction: Direction = Direction::Right;
    let mut current_value = 0;
    let mut right_completed_columns = 0;
    let mut top_completed_rows = 0;
    let mut left_completed_columns = 0;
    let mut bottom_completed_rows = 0;

    while current_value < n.pow(2) {
        current_value += 1;
        matrix[current_row][current_column] = current_value;

        if direction == Direction::Right {
            if current_column == n - 1 - right_completed_columns {
                direction = Direction::Down;
                top_completed_rows += 1;
                current_row += 1;
            } else {
                current_column += 1;
            }
        } else if direction == Direction::Down {
            if current_row == n - 1 - bottom_completed_rows {
                direction = Direction::Left;
                right_completed_columns += 1;
                current_column -= 1;
            } else {
                current_row += 1;
            }
        } else if direction == Direction::Left {
            if current_column == 0 + left_completed_columns {
                direction = Direction::Up;
                bottom_completed_rows += 1;
                current_row -= 1;
            } else {
                current_column -= 1;
            }
        } else {
            if current_row == 0 + top_completed_rows {
                direction = Direction::Right;
                left_completed_columns += 1;
                current_column += 1;
            } else {
                current_row -= 1;
            }
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_2() {
        assert_eq!(matrix(2), vec![vec![1, 2], vec![4, 3]]);
    }

    #[test]
    fn test_matrix_3() {
        assert_eq!(matrix(3), vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]);
    }

    #[test]
    fn test_matrix_4() {
        assert_eq!(
            matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }

    #[test]
    fn test_matrix_5() {
        assert_eq!(
            matrix(5),
            vec![
                vec![1, 2, 3, 4, 5],
                vec![16, 17, 18, 19, 6],
                vec![15, 24, 25, 20, 7],
                vec![14, 23, 22, 21, 8],
                vec![13, 12, 11, 10, 9]
            ]
        );
    }
}
