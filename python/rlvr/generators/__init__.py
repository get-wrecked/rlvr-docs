"""
Procedural generators — produce unlimited training data for RLVR domains.

Each generator returns: {"prompt": str, "gold": str | list, "metadata": dict}

Generator categories (by domain count):
  - math_arithmetic    — basic arithmetic, fractions, percentages (~50 domains)
  - math_calculus      — derivatives, integrals, limits, series (~20 domains)
  - math_linear_alg    — matrices, eigenvalues, systems of equations (~15 domains)
  - math_number_theory — primes, GCD, modular arithmetic (~15 domains)
  - math_combinatorics — counting, permutations, partitions (~10 domains)
  - math_probability   — probability, statistics, distributions (~15 domains)
  - logic_boolean      — truth tables, SAT, boolean algebra (~10 domains)
  - logic_puzzles      — Sudoku, KenKen, nonograms, constraint puzzles (~30 domains)
  - code_algorithms    — algorithm problems with test cases (~40 domains)
  - code_data_struct   — data structure operations (~15 domains)
  - science_physics    — mechanics, E&M, thermodynamics (~20 domains)
  - science_chemistry  — equations, stoichiometry, equilibrium (~15 domains)
  - conversions        — unit, base, timezone, coordinate (~20 domains)
  - games              — game state evaluation, move generation (~20 domains)
"""
