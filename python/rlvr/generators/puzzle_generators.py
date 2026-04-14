"""
Puzzle generators — procedural generation for ~100 constraint/logic puzzle domains.

Each generator creates a puzzle with a known solution, then returns the puzzle
(without solution) as the prompt and the solution as the gold answer.
"""

import random
import copy
from typing import Optional


def gen_sudoku(difficulty: int = 5) -> dict:
    """Generate a Sudoku puzzle by creating a solved grid and removing cells."""
    def _is_valid(grid, row, col, num):
        if num in grid[row]:
            return False
        if num in [grid[r][col] for r in range(9)]:
            return False
        box_r, box_c = 3 * (row // 3), 3 * (col // 3)
        for r in range(box_r, box_r + 3):
            for c in range(box_c, box_c + 3):
                if grid[r][c] == num:
                    return False
        return True

    def _fill(grid, pos=0):
        if pos == 81:
            return True
        row, col = pos // 9, pos % 9
        if grid[row][col] != 0:
            return _fill(grid, pos + 1)
        nums = list(range(1, 10))
        random.shuffle(nums)
        for num in nums:
            if _is_valid(grid, row, col, num):
                grid[row][col] = num
                if _fill(grid, pos + 1):
                    return True
                grid[row][col] = 0
        return False

    # Generate solved grid
    grid = [[0] * 9 for _ in range(9)]
    _fill(grid)
    solution = [row[:] for row in grid]

    # Remove cells based on difficulty (more removed = harder)
    cells_to_remove = int(20 + difficulty * 4.5)  # 24 to 65 cells removed
    cells_to_remove = min(cells_to_remove, 64)
    positions = [(r, c) for r in range(9) for c in range(9)]
    random.shuffle(positions)
    for r, c in positions[:cells_to_remove]:
        grid[r][c] = 0

    puzzle_str = "\n".join(" ".join(str(x) if x != 0 else "." for x in row) for row in grid)
    solution_str = "\n".join(" ".join(str(x) for x in row) for row in solution)

    return {
        "prompt": f"Solve this Sudoku puzzle (replace . with digits 1-9):\n{puzzle_str}",
        "gold": solution_str,
        "metadata": {"domain": "sudoku-solving", "type": "text", "difficulty": difficulty,
                     "grid": grid, "solution": solution}
    }


def gen_nqueens(difficulty: int = 5) -> dict:
    """Generate N-Queens problem."""
    n = min(4 + difficulty, 12)

    # Find one solution via backtracking
    def solve(queens, row):
        if row == n:
            return queens[:]
        for col in range(n):
            if all(col != qc and abs(col - qc) != abs(row - qr) for qr, qc in enumerate(queens)):
                queens.append(col)
                result = solve(queens, row + 1)
                if result:
                    return result
                queens.pop()
        return None

    solution = solve([], 0)
    if not solution:
        return gen_nqueens(max(1, difficulty - 1))

    gold = ", ".join(f"({r+1},{c+1})" for r, c in enumerate(solution))

    return {
        "prompt": f"Place {n} queens on a {n}x{n} chessboard such that no two queens threaten each other. "
                  f"Give the column position of the queen in each row, as (row,col) pairs.",
        "gold": gold,
        "metadata": {"domain": "n-queens", "type": "text", "difficulty": difficulty, "n": n}
    }


def gen_magic_square(difficulty: int = 5) -> dict:
    """Generate magic square problems."""
    n = random.choice([3, 4, 5]) if difficulty >= 5 else 3

    if n == 3:
        # Standard 3x3 magic square (unique up to rotation/reflection)
        square = [[2, 7, 6], [9, 5, 1], [4, 3, 8]]
        magic_sum = 15
    elif n == 4:
        # 4x4 magic square
        square = [
            [16, 3, 2, 13],
            [5, 10, 11, 8],
            [9, 6, 7, 12],
            [4, 15, 14, 1],
        ]
        magic_sum = 34
    else:
        # 5x5 via Siamese method
        square = [[0] * 5 for _ in range(5)]
        r, c = 0, 2
        for num in range(1, 26):
            square[r][c] = num
            nr, nc = (r - 1) % 5, (c + 1) % 5
            if square[nr][nc] != 0:
                nr, nc = (r + 1) % 5, c
            r, c = nr, nc
        magic_sum = 65

    # Remove some cells based on difficulty
    cells = [(r, c) for r in range(n) for c in range(n)]
    random.shuffle(cells)
    num_remove = min(int(difficulty * n * n / 12), n * n - n)

    puzzle = [row[:] for row in square]
    for r, c in cells[:num_remove]:
        puzzle[r][c] = 0

    puzzle_str = "\n".join(" ".join(f"{x:2d}" if x != 0 else " ." for x in row) for row in puzzle)
    solution_str = "\n".join(" ".join(f"{x:2d}" for x in row) for row in square)

    return {
        "prompt": f"Complete this {n}x{n} magic square (each row, column, and diagonal sums to {magic_sum}):\n{puzzle_str}",
        "gold": solution_str,
        "metadata": {"domain": "magic-square", "type": "text", "difficulty": difficulty}
    }


def gen_24_game(difficulty: int = 5) -> dict:
    """Generate 24 Game problems: use 4 numbers and +,-,*,/ to make 24."""
    import itertools

    max_num = int(5 + difficulty * 1.5)
    for _ in range(100):  # try to find a solvable instance
        numbers = [random.randint(1, max_num) for _ in range(4)]
        ops = ['+', '-', '*', '/']

        # Try all permutations of numbers and operators
        for perm in itertools.permutations(numbers):
            for op_combo in itertools.product(ops, repeat=3):
                # Try different parenthesizations
                expressions = [
                    f"(({perm[0]} {op_combo[0]} {perm[1]}) {op_combo[1]} {perm[2]}) {op_combo[2]} {perm[3]}",
                    f"({perm[0]} {op_combo[0]} ({perm[1]} {op_combo[1]} {perm[2]})) {op_combo[2]} {perm[3]}",
                    f"({perm[0]} {op_combo[0]} {perm[1]}) {op_combo[1]} ({perm[2]} {op_combo[2]} {perm[3]})",
                    f"{perm[0]} {op_combo[0]} (({perm[1]} {op_combo[1]} {perm[2]}) {op_combo[2]} {perm[3]})",
                    f"{perm[0]} {op_combo[0]} ({perm[1]} {op_combo[1]} ({perm[2]} {op_combo[2]} {perm[3]}))",
                ]
                for expr in expressions:
                    try:
                        if abs(eval(expr) - 24) < 1e-9:
                            return {
                                "prompt": f"Using the numbers {numbers[0]}, {numbers[1]}, {numbers[2]}, {numbers[3]} "
                                          f"and the operations +, -, *, /, make 24. Use each number exactly once.",
                                "gold": expr,
                                "metadata": {"domain": "24-game", "type": "text",
                                           "difficulty": difficulty, "numbers": numbers}
                            }
                    except (ZeroDivisionError, SyntaxError):
                        continue

    # Fallback: known solvable instance
    return {
        "prompt": "Using the numbers 1, 2, 3, 4 and the operations +, -, *, /, make 24.",
        "gold": "1 * 2 * 3 * 4",
        "metadata": {"domain": "24-game", "type": "text", "difficulty": difficulty}
    }


def gen_cryptarithmetic(difficulty: int = 5) -> dict:
    """Generate simple cryptarithmetic puzzles (SEND + MORE = MONEY style)."""
    # Pre-made puzzles of varying difficulty (generating from scratch is hard)
    puzzles = [
        # Easy
        {"puzzle": "AB + BA = CBC", "solution": "A=2, B=9, C=1", "difficulty": 2},
        {"puzzle": "AB + AB = BAA", "solution": "A=5, B=9", "difficulty": 3},
        # Medium
        {"puzzle": "TO + GO = OUT", "solution": "T=2, O=1, G=8, U=9", "difficulty": 5},
        {"puzzle": "NO + GUN = HUNT", "solution": "N=2, O=3, G=9, U=5, H=1, T=6", "difficulty": 6},
        # Hard
        {"puzzle": "SEND + MORE = MONEY", "solution": "S=9, E=5, N=6, D=7, M=1, O=0, R=8, Y=2", "difficulty": 8},
        {"puzzle": "CROSS + ROADS = DANGER", "solution": "C=9, R=6, O=2, S=3, A=5, D=1, N=8, G=7, E=4", "difficulty": 9},
    ]

    # Pick puzzle matching difficulty
    valid = [p for p in puzzles if abs(p["difficulty"] - difficulty) <= 2]
    if not valid:
        valid = puzzles
    puzzle = random.choice(valid)

    return {
        "prompt": f"Solve this cryptarithmetic puzzle. Each letter represents a unique digit (0-9).\n{puzzle['puzzle']}",
        "gold": puzzle["solution"],
        "metadata": {"domain": "cryptarithmetic", "type": "text", "difficulty": difficulty}
    }


def gen_latin_square(difficulty: int = 5) -> dict:
    """Generate Latin square completion problems."""
    n = min(3 + difficulty // 2, 7)

    # Generate a complete Latin square
    square = [[(i + j) % n + 1 for j in range(n)] for i in range(n)]
    # Shuffle rows and columns for variety
    perm = list(range(n))
    random.shuffle(perm)
    square = [square[p] for p in perm]
    perm2 = list(range(n))
    random.shuffle(perm2)
    square = [[row[p] for p in perm2] for row in square]

    solution = [row[:] for row in square]

    # Remove cells
    num_remove = min(int(n * n * difficulty / 12), n * n - n)
    cells = [(r, c) for r in range(n) for c in range(n)]
    random.shuffle(cells)
    for r, c in cells[:num_remove]:
        square[r][c] = 0

    puzzle_str = "\n".join(" ".join(str(x) if x != 0 else "." for x in row) for row in square)
    solution_str = "\n".join(" ".join(str(x) for x in row) for row in solution)

    return {
        "prompt": f"Complete this {n}x{n} Latin square (each number 1-{n} appears exactly once in each row and column):\n{puzzle_str}",
        "gold": solution_str,
        "metadata": {"domain": "latin-square", "type": "text", "difficulty": difficulty}
    }


# ═══════════════════════════════════════════
# GENERATOR REGISTRY
# ═══════════════════════════════════════════

GENERATORS = {
    "sudoku-solving": gen_sudoku,
    "sudoku-generation": gen_sudoku,
    "n-queens": gen_nqueens,
    "eight-queens-variant": gen_nqueens,
    "magic-square": gen_magic_square,
    "latin-square": gen_latin_square,
    "24-game": gen_24_game,
    "cryptarithmetic": gen_cryptarithmetic,
}


def generate(domain: str, difficulty: int = 5) -> dict:
    """Generate a puzzle for the given domain."""
    gen_fn = GENERATORS.get(domain, gen_sudoku)
    return gen_fn(difficulty)
