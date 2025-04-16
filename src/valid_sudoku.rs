/*
Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
    Each row must contain the digits 1-9 without repetition.
    Each column must contain the digits 1-9 without repetition.
    Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

Note:
    A Sudoku board (partially filled) could be valid but is not necessarily solvable.
    Only the filled cells need to be validated according to the mentioned rules.

Example 1:
Input: board = 
[["5","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: true

Example 2:
Input: board = 
[["8","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: false
Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.

Constraints:
    board.length == 9
    board[i].length == 9
    board[i][j] is a digit 1-9 or '.'.
*/

use crate::Tests;

pub fn check(items: &Vec<char>) -> bool {
    let mut checks: u16 = 0;
    for item in items {
        if *item == '.' {
            continue;
        }
        if (checks & (1 << (*item as u8 - '0' as u8))) != 0 {
            return false;
        }
        checks |= 1 << (*item as u8 - '0' as u8);
    }
    true
}

pub fn board_piece(
    items: &mut Vec<char>,
    board: &Vec<Vec<char>>,
    i: usize,
    j: usize
) {
    for k in 0..3 {
        for l in 0..3 {
            items[3 * k + l] = board[3 * j + l][3 * i + k];
        }
    }
}


pub fn solution(board: Vec<Vec<char>>) -> bool {
    for row in board.iter() {
        if !check(row) {
            return false;
        }
    }
    let mut items = vec!['.'; 9];
    for i in 0..9 {
        for j in 0..9 {
            items[j] = board[j][i];
        }
        if !check(&items) {
            return false;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            board_piece(&mut items, &board, i, j);
            if !check(&items) {
                return false;
            }
        }
    }
    true
}

pub fn test(test: &mut Tests) {
    test.add_test(solution(vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ]) == true);
    test.add_test(solution(vec![
        vec!['8','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ]) == false);
}
