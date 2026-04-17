//! Sudoku Verifier
//!
//! Verifies Sudoku solutions: checks that a 9x9 grid satisfies all Sudoku constraints.
//! Also verifies that the solution respects the original puzzle's givens.
//!
//! This is a pure constraint-satisfaction verifier — no external dependencies needed.

use super::VerifyResult;

/// A 9x9 Sudoku grid. 0 = empty cell.
pub type Grid = [[u8; 9]; 9];

/// Parse a Sudoku grid from a flat string of 81 digits (0 = empty).
pub fn parse_grid(s: &str) -> Option<Grid> {
    let digits: Vec<u8> = s
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .map(|c| if c == '.' { 0 } else { c as u8 - b'0' })
        .collect();

    if digits.len() != 81 {
        return None;
    }

    let mut grid = [[0u8; 9]; 9];
    for (i, &d) in digits.iter().enumerate() {
        grid[i / 9][i % 9] = d;
    }
    Some(grid)
}

/// Check if a solved grid is valid (all rows, columns, and 3x3 boxes contain 1-9 exactly once).
pub fn is_valid_solution(grid: &Grid) -> bool {
    // Check all cells are 1-9
    for row in grid {
        for &cell in row {
            if !(1..=9).contains(&cell) {
                return false;
            }
        }
    }

    // Check rows
    for row in grid {
        if !has_all_digits(row.iter().copied()) {
            return false;
        }
    }

    // Check columns
    for col in 0..9 {
        if !has_all_digits(grid.iter().map(|row| row[col])) {
            return false;
        }
    }

    // Check 3x3 boxes
    for box_row in 0..3 {
        for box_col in 0..3 {
            let iter =
                (0..3).flat_map(|r| (0..3).map(move |c| grid[box_row * 3 + r][box_col * 3 + c]));
            if !has_all_digits(iter) {
                return false;
            }
        }
    }

    true
}

/// Check that solution respects puzzle givens (non-zero cells in puzzle must match solution).
pub fn respects_givens(puzzle: &Grid, solution: &Grid) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if puzzle[r][c] != 0 && puzzle[r][c] != solution[r][c] {
                return false;
            }
        }
    }
    true
}

fn has_all_digits(iter: impl Iterator<Item = u8>) -> bool {
    let mut seen = [false; 10]; // index 0 unused
    for d in iter {
        if !(1..=9).contains(&d) {
            return false;
        }
        if seen[d as usize] {
            return false; // duplicate
        }
        seen[d as usize] = true;
    }
    // Check all 1-9 were seen
    (1..=9).all(|i| seen[i])
}

/// Verify a Sudoku solution against a puzzle.
/// `puzzle_str`: 81 chars, 0 or . for empty
/// `solution_str`: 81 chars, all 1-9
pub fn verify(puzzle_str: &str, solution_str: &str) -> VerifyResult {
    let puzzle = match parse_grid(puzzle_str) {
        Some(g) => g,
        None => return VerifyResult::wrong("could not parse puzzle"),
    };

    let solution = match parse_grid(solution_str) {
        Some(g) => g,
        None => return VerifyResult::wrong("could not parse solution"),
    };

    if !respects_givens(&puzzle, &solution) {
        return VerifyResult::wrong("solution changes puzzle givens");
    }

    if !is_valid_solution(&solution) {
        // Count how many constraints are satisfied for partial credit
        let row_ok = (0..9)
            .filter(|&r| has_all_digits(solution[r].iter().copied()))
            .count();
        let col_ok = (0..9)
            .filter(|&c| has_all_digits((0..9).map(|r| solution[r][c])))
            .count();
        let box_ok = (0..3)
            .flat_map(|br| (0..3).map(move |bc| (br, bc)))
            .filter(|&(br, bc)| {
                has_all_digits(
                    (0..3).flat_map(|r| (0..3).map(move |c| solution[br * 3 + r][bc * 3 + c])),
                )
            })
            .count();
        let total = row_ok + col_ok + box_ok;
        let max_total = 27; // 9 rows + 9 cols + 9 boxes
        return VerifyResult::partial(
            total as f64 / max_total as f64,
            format!("invalid: {row_ok}/9 rows, {col_ok}/9 cols, {box_ok}/9 boxes"),
        );
    }

    VerifyResult::correct()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Known valid solved Sudoku
    const VALID_SOLUTION: &str = "\
        534678912\
        672195348\
        198342567\
        859761423\
        426853791\
        713924856\
        961537284\
        287419635\
        345286179";

    // The puzzle (givens) for the above solution
    const PUZZLE: &str = "\
        530070000\
        600195000\
        098000060\
        800060003\
        400803001\
        700020006\
        060000280\
        000419005\
        000080079";

    // ========== Grid Parsing ==========

    #[test]
    fn parse_valid_grid() {
        let grid = parse_grid(VALID_SOLUTION).unwrap();
        assert_eq!(grid[0][0], 5);
        assert_eq!(grid[0][1], 3);
        assert_eq!(grid[8][8], 9);
    }

    #[test]
    fn parse_with_dots() {
        let s = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
        let grid = parse_grid(s).unwrap();
        assert_eq!(grid[0][0], 5);
        assert_eq!(grid[0][2], 0); // dot = 0
    }

    #[test]
    fn parse_wrong_length() {
        assert!(parse_grid("12345").is_none());
        assert!(parse_grid("").is_none());
    }

    // ========== Solution Validation ==========

    #[test]
    fn valid_solution_passes() {
        let grid = parse_grid(VALID_SOLUTION).unwrap();
        assert!(is_valid_solution(&grid));
    }

    #[test]
    fn solution_with_zero_fails() {
        let mut grid = parse_grid(VALID_SOLUTION).unwrap();
        grid[0][0] = 0; // introduce empty cell
        assert!(!is_valid_solution(&grid));
    }

    #[test]
    fn solution_with_duplicate_in_row_fails() {
        let mut grid = parse_grid(VALID_SOLUTION).unwrap();
        // Row 0 is [5,3,4,6,7,8,9,1,2] — change grid[0][1] from 3 to 5 (duplicate)
        grid[0][1] = 5;
        assert!(!is_valid_solution(&grid));
    }

    #[test]
    fn solution_with_duplicate_in_col_fails() {
        let mut grid = parse_grid(VALID_SOLUTION).unwrap();
        // Col 0 is [5,6,1,8,4,7,9,2,3] — change grid[1][0] from 6 to 5 (duplicate)
        grid[1][0] = 5;
        assert!(!is_valid_solution(&grid));
    }

    #[test]
    fn solution_with_duplicate_in_box_fails() {
        let mut grid = parse_grid(VALID_SOLUTION).unwrap();
        // Top-left box is [5,3,4,6,7,2,1,9,8] — change grid[1][1] from 7 to 5
        grid[1][1] = 5;
        assert!(!is_valid_solution(&grid));
    }

    // ========== Givens Respect ==========

    #[test]
    fn solution_respects_puzzle_givens() {
        let puzzle = parse_grid(PUZZLE).unwrap();
        let solution = parse_grid(VALID_SOLUTION).unwrap();
        assert!(respects_givens(&puzzle, &solution));
    }

    #[test]
    fn solution_violates_puzzle_givens() {
        let puzzle = parse_grid(PUZZLE).unwrap();
        let mut solution = parse_grid(VALID_SOLUTION).unwrap();
        // Puzzle has 5 at [0][0], change solution to 3
        solution[0][0] = 3;
        assert!(!respects_givens(&puzzle, &solution));
    }

    // ========== Full Verify ==========

    #[test]
    fn verify_correct_solution() {
        let result = verify(PUZZLE, VALID_SOLUTION);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_wrong_solution() {
        // Swap two digits in solution
        let mut bad = VALID_SOLUTION.to_string().into_bytes();
        bad.swap(1, 2); // swap positions 1 and 2 (3↔4 in row 0)
        let bad_str = String::from_utf8(bad).unwrap();
        let result = verify(PUZZLE, &bad_str);
        assert!(result.score < 1.0, "Swapped solution should not score 1.0");
    }

    #[test]
    fn verify_unparseable_solution() {
        let result = verify(PUZZLE, "not a grid");
        assert_eq!(result.score, 0.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_all_ones_fails() {
        let all_ones = "1".repeat(81);
        let result = verify(PUZZLE, &all_ones);
        assert!(result.score < 1.0, "All 1s should not be valid");
    }

    #[test]
    fn antihardcode_identity_grid_fails() {
        // 123456789 repeated 9 times — valid rows but not columns
        let identity = "123456789".repeat(9);
        let empty_puzzle = "0".repeat(81);
        let result = verify(&empty_puzzle, &identity);
        assert!(
            result.score < 1.0,
            "Identity grid should not be valid (col duplicates)"
        );
        // But it should get partial credit for rows
        assert!(
            result.score > 0.0,
            "Should get partial credit for correct rows"
        );
    }

    #[test]
    fn antihardcode_different_valid_solutions() {
        // Another known valid Sudoku (different from VALID_SOLUTION)
        let solution2 = "\
            812753649\
            943682175\
            675491283\
            154237896\
            369845721\
            287169534\
            521974368\
            438526917\
            796318452";
        let empty_puzzle = "0".repeat(81);
        let result = verify(&empty_puzzle, solution2);
        assert_eq!(result.score, 1.0, "Second valid Sudoku should pass");
    }
}
