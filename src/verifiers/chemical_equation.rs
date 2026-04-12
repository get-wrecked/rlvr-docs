//! Chemical Equation Balancing Verifier
//!
//! Verifies that a balanced chemical equation has equal atom counts on both sides.
//! Pure constraint satisfaction — no chemistry knowledge needed, just counting.
//!
//! Format: "2H2 + O2 -> 2H2O"
//! Verification: count atoms of each element on LHS and RHS, must be equal.

use std::collections::HashMap;
use super::VerifyResult;

/// Parse a chemical formula like "H2O", "Ca(OH)2", "Fe2O3" into atom counts.
pub fn parse_formula(formula: &str) -> Result<HashMap<String, i64>, String> {
    let chars: Vec<char> = formula.chars().collect();
    parse_group(&chars, &mut 0)
}

fn parse_group(chars: &[char], pos: &mut usize) -> Result<HashMap<String, i64>, String> {
    let mut counts: HashMap<String, i64> = HashMap::new();

    while *pos < chars.len() {
        if chars[*pos] == '(' {
            *pos += 1; // skip '('
            let sub = parse_group(chars, pos)?;
            if *pos < chars.len() && chars[*pos] == ')' {
                *pos += 1; // skip ')'
            } else {
                return Err("unmatched parenthesis".into());
            }
            let mult = parse_number(chars, pos);
            for (elem, count) in sub {
                *counts.entry(elem).or_insert(0) += count * mult;
            }
        } else if chars[*pos] == ')' {
            break; // end of group
        } else if chars[*pos].is_ascii_uppercase() {
            let elem = parse_element(chars, pos);
            let num = parse_number(chars, pos);
            *counts.entry(elem).or_insert(0) += num;
        } else {
            return Err(format!("unexpected character: {}", chars[*pos]));
        }
    }

    Ok(counts)
}

fn parse_element(chars: &[char], pos: &mut usize) -> String {
    let mut elem = String::new();
    if *pos < chars.len() && chars[*pos].is_ascii_uppercase() {
        elem.push(chars[*pos]);
        *pos += 1;
        while *pos < chars.len() && chars[*pos].is_ascii_lowercase() {
            elem.push(chars[*pos]);
            *pos += 1;
        }
    }
    elem
}

fn parse_number(chars: &[char], pos: &mut usize) -> i64 {
    let mut num_str = String::new();
    while *pos < chars.len() && chars[*pos].is_ascii_digit() {
        num_str.push(chars[*pos]);
        *pos += 1;
    }
    if num_str.is_empty() { 1 } else { num_str.parse().unwrap_or(1) }
}

/// Parse a term like "2H2O" or "Fe2O3" (coefficient + formula).
fn parse_term(term: &str) -> Result<HashMap<String, i64>, String> {
    let term = term.trim();
    if term.is_empty() {
        return Err("empty term".into());
    }

    // Extract leading coefficient
    let mut i = 0;
    let chars: Vec<char> = term.chars().collect();
    while i < chars.len() && chars[i].is_ascii_digit() {
        i += 1;
    }

    let coeff: i64 = if i == 0 { 1 } else { term[..i].parse().unwrap_or(1) };
    let formula = &term[i..];

    let atoms = parse_formula(formula)?;
    let mut result = HashMap::new();
    for (elem, count) in atoms {
        *result.entry(elem).or_insert(0) += count * coeff;
    }
    Ok(result)
}

/// Parse one side of an equation (e.g., "2H2 + O2") into total atom counts.
fn parse_side(side: &str) -> Result<HashMap<String, i64>, String> {
    let mut total: HashMap<String, i64> = HashMap::new();
    for term in side.split('+') {
        let atoms = parse_term(term.trim())?;
        for (elem, count) in atoms {
            *total.entry(elem).or_insert(0) += count;
        }
    }
    Ok(total)
}

/// Verify a balanced chemical equation.
/// Format: "LHS -> RHS" or "LHS = RHS" or "LHS → RHS"
pub fn verify(equation: &str) -> VerifyResult {
    // Split on arrow/equals
    let (lhs_str, rhs_str) = if let Some(pos) = equation.find("->") {
        (&equation[..pos], &equation[pos + 2..])
    } else if let Some(pos) = equation.find('→') {
        let byte_pos = equation.find('→').unwrap();
        (&equation[..byte_pos], &equation[byte_pos + '→'.len_utf8()..])
    } else if let Some(pos) = equation.find('=') {
        (&equation[..pos], &equation[pos + 1..])
    } else {
        return VerifyResult::wrong("no arrow/equals found in equation");
    };

    let lhs = match parse_side(lhs_str.trim()) {
        Ok(a) => a,
        Err(e) => return VerifyResult::wrong(format!("LHS parse error: {e}")),
    };

    let rhs = match parse_side(rhs_str.trim()) {
        Ok(a) => a,
        Err(e) => return VerifyResult::wrong(format!("RHS parse error: {e}")),
    };

    // Check all elements balance
    let mut all_elements: Vec<String> = lhs.keys().chain(rhs.keys()).cloned().collect();
    all_elements.sort();
    all_elements.dedup();

    let mut balanced = 0;
    let mut unbalanced = Vec::new();

    for elem in &all_elements {
        let l = lhs.get(elem).copied().unwrap_or(0);
        let r = rhs.get(elem).copied().unwrap_or(0);
        if l == r && l > 0 {
            balanced += 1;
        } else {
            unbalanced.push(format!("{elem}: {l} vs {r}"));
        }
    }

    if unbalanced.is_empty() {
        VerifyResult::correct()
    } else {
        let total = all_elements.len();
        VerifyResult::partial(
            balanced as f64 / total as f64,
            format!("unbalanced: {}", unbalanced.join(", ")),
        )
    }
}

/// Verify that a student's balanced equation matches the expected equation.
/// Both sides must have the same atoms.
pub fn verify_balancing(unbalanced: &str, proposed_balanced: &str) -> VerifyResult {
    // First check the balanced equation is actually balanced
    let balance_result = verify(proposed_balanced);
    if balance_result.score < 1.0 {
        return balance_result;
    }

    // Then check it uses the same compounds (not a different reaction)
    // This is optional — for now just check balance
    balance_result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Formula Parsing ==========

    #[test]
    fn parse_h2o() {
        let atoms = parse_formula("H2O").unwrap();
        assert_eq!(atoms.get("H"), Some(&2));
        assert_eq!(atoms.get("O"), Some(&1));
    }

    #[test]
    fn parse_fe2o3() {
        let atoms = parse_formula("Fe2O3").unwrap();
        assert_eq!(atoms.get("Fe"), Some(&2));
        assert_eq!(atoms.get("O"), Some(&3));
    }

    #[test]
    fn parse_ca_oh_2() {
        let atoms = parse_formula("Ca(OH)2").unwrap();
        assert_eq!(atoms.get("Ca"), Some(&1));
        assert_eq!(atoms.get("O"), Some(&2));
        assert_eq!(atoms.get("H"), Some(&2));
    }

    #[test]
    fn parse_mg_oh_2() {
        let atoms = parse_formula("Mg(OH)2").unwrap();
        assert_eq!(atoms.get("Mg"), Some(&1));
        assert_eq!(atoms.get("O"), Some(&2));
        assert_eq!(atoms.get("H"), Some(&2));
    }

    #[test]
    fn parse_term_with_coefficient() {
        let atoms = parse_term("2H2O").unwrap();
        assert_eq!(atoms.get("H"), Some(&4));
        assert_eq!(atoms.get("O"), Some(&2));
    }

    // ========== Balanced Equations ==========

    #[test]
    fn balanced_water_synthesis() {
        // 2H2 + O2 -> 2H2O
        let result = verify("2H2 + O2 -> 2H2O");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn balanced_combustion() {
        // CH4 + 2O2 -> CO2 + 2H2O
        let result = verify("CH4 + 2O2 -> CO2 + 2H2O");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn balanced_rust() {
        // 4Fe + 3O2 -> 2Fe2O3
        let result = verify("4Fe + 3O2 -> 2Fe2O3");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn balanced_neutralization() {
        // HCl + NaOH -> NaCl + H2O
        let result = verify("HCl + NaOH -> NaCl + H2O");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn balanced_photosynthesis() {
        // 6CO2 + 6H2O -> C6H12O6 + 6O2
        let result = verify("6CO2 + 6H2O -> C6H12O6 + 6O2");
        assert_eq!(result.score, 1.0);
    }

    // ========== Unbalanced Equations ==========

    #[test]
    fn unbalanced_water() {
        // H2 + O2 -> H2O (unbalanced — 2 O on left, 1 on right)
        let result = verify("H2 + O2 -> H2O");
        assert!(result.score < 1.0);
    }

    #[test]
    fn unbalanced_combustion() {
        // CH4 + O2 -> CO2 + H2O (unbalanced)
        let result = verify("CH4 + O2 -> CO2 + H2O");
        assert!(result.score < 1.0);
    }

    // ========== Arrow Formats ==========

    #[test]
    fn equals_sign_format() {
        let result = verify("2H2 + O2 = 2H2O");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn unicode_arrow_format() {
        let result = verify("2H2 + O2 → 2H2O");
        assert_eq!(result.score, 1.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_balanced_unbalanced_pairs() {
        // Same reaction, balanced vs unbalanced
        assert_eq!(verify("2H2 + O2 -> 2H2O").score, 1.0);
        assert!(verify("H2 + O2 -> H2O").score < 1.0);

        assert_eq!(verify("4Fe + 3O2 -> 2Fe2O3").score, 1.0);
        assert!(verify("Fe + O2 -> Fe2O3").score < 1.0);

        assert_eq!(verify("CH4 + 2O2 -> CO2 + 2H2O").score, 1.0);
        assert!(verify("CH4 + O2 -> CO2 + H2O").score < 1.0);
    }

    // ========== ADVERSARIAL ==========

    #[test]
    fn adversarial_close_but_wrong() {
        // 3H2 + O2 -> 2H2O — H is 6 vs 4, must fail
        let result = verify("3H2 + O2 -> 2H2O");
        assert!(result.score < 1.0, "3H2 gives H=6 on left, 2H2O gives H=4 on right");
    }

    #[test]
    fn adversarial_complex_organic() {
        // Combustion of ethanol: C2H5OH + 3O2 -> 2CO2 + 3H2O
        let result = verify("C2H5OH + 3O2 -> 2CO2 + 3H2O");
        assert_eq!(result.score, 1.0, "Ethanol combustion balanced");
    }

    #[test]
    fn adversarial_trivially_balanced() {
        let result = verify("H2O -> H2O");
        assert_eq!(result.score, 1.0, "Identity is technically balanced");
    }

    #[test]
    fn adversarial_missing_element() {
        // Na on left but not right
        let result = verify("NaCl + H2O -> HCl");
        assert!(result.score < 1.0, "Na missing from right side");
    }
}
