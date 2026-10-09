//! Versioned scalar answer grammar `scalar-exact-v1` (Section 13.2).
//!
//! Accepts integers, finite decimals, `+ - * / ^`, and parentheses. Never evaluates
//! learner text as code. Rejects ambiguous juxtaposition (`2(3)`, `1 3/4`) with a
//! clarification instead of guessing. All evaluation is exact and budgeted.

use crate::number::{bit_size, Q};
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};

pub const GRAMMAR_VERSION: &str = "scalar-exact-v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MathBudget {
    pub max_input_bytes: usize,
    pub max_nodes: usize,
    pub max_depth: usize,
    pub max_integer_bits: u64,
}

impl Default for MathBudget {
    /// Initial budgets from Section 13.2.
    fn default() -> Self {
        MathBudget {
            max_input_bytes: 8 * 1024,
            max_nodes: 1000,
            max_depth: 64,
            max_integer_bits: 4096,
        }
    }
}

/// Parse failure codes are stable identifiers; messages are learner-facing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParseFailure {
    pub code: String,
    pub message: String,
    /// Character offset (0-based) in the normalized input, when meaningful.
    pub position: Option<usize>,
    /// True when the failure is a resource budget limit (reported as `unsupported`).
    pub budget: bool,
}

impl ParseFailure {
    fn new(code: &str, message: impl Into<String>, position: Option<usize>) -> Self {
        ParseFailure {
            code: code.into(),
            message: message.into(),
            position,
            budget: false,
        }
    }
    fn budget(code: &str, message: impl Into<String>) -> Self {
        ParseFailure {
            code: code.into(),
            message: message.into(),
            position: None,
            budget: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Ast {
    /// Exact literal stored canonically ("3", "0.25" -> "1/4").
    Num { value: String, raw: String },
    Neg { arg: Box<Ast> },
    Add { lhs: Box<Ast>, rhs: Box<Ast> },
    Sub { lhs: Box<Ast>, rhs: Box<Ast> },
    Mul { lhs: Box<Ast>, rhs: Box<Ast> },
    Div { lhs: Box<Ast>, rhs: Box<Ast> },
    Pow { base: Box<Ast>, exp: Box<Ast> },
}

impl Ast {
    /// True iff the expression is a bare literal or a single fraction `a/b` of
    /// integer literals (optionally negated) — used by form-sensitive checkers.
    pub fn as_simple_fraction(&self) -> Option<(BigInt, BigInt)> {
        fn int_lit(a: &Ast) -> Option<BigInt> {
            match a {
                Ast::Num { raw, .. } if raw.bytes().all(|b| b.is_ascii_digit()) => raw.parse().ok(),
                _ => None,
            }
        }
        match self {
            Ast::Div { lhs, rhs } => Some((int_lit(lhs)?, int_lit(rhs)?)),
            Ast::Neg { arg } => match arg.as_ref() {
                Ast::Div { lhs, rhs } => Some((-int_lit(lhs)?, int_lit(rhs)?)),
                _ => None,
            },
            _ => None,
        }
    }
}

/// Documented normalization stage: Unicode minus/multiplication/division → ASCII.
pub fn normalize(raw: &str) -> String {
    raw.chars()
        .map(|c| match c {
            '\u{2212}' | '\u{2013}' | '\u{2010}' | '\u{FE63}' | '\u{FF0D}' => '-',
            '\u{00D7}' | '\u{22C5}' | '\u{00B7}' | '\u{2219}' => '*',
            '\u{00F7}' | '\u{2215}' | '\u{2044}' => '/',
            '\u{FF0B}' => '+',
            '\u{00A0}' | '\u{2009}' | '\u{202F}' => ' ',
            c => c,
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Result<Vec<(Tok, usize, bool)>, ParseFailure> {
    // (token, position, preceded_by_space)
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let mut space = false;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            space = true;
            i += 1;
            continue;
        }
        let start = i;
        let tok = match c {
            '0'..='9' | '.' => {
                let mut lit = String::new();
                let mut dots = 0;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    if chars[i] == '.' {
                        dots += 1;
                    }
                    lit.push(chars[i]);
                    i += 1;
                }
                if dots > 1 || lit == "." || lit.ends_with('.') {
                    return Err(ParseFailure::new(
                        "bad_decimal",
                        format!("“{lit}” is not a valid decimal. Write decimals like 0.5 or 12.75."),
                        Some(start),
                    ));
                }
                if i < chars.len() && chars[i] == ',' {
                    return Err(ParseFailure::new(
                        "decimal_comma",
                        "This activity uses a point for decimals (for example 2.5), not a comma.",
                        Some(i),
                    ));
                }
                out.push((Tok::Num(lit), start, space));
                space = false;
                continue;
            }
            '+' => Tok::Plus,
            '-' => Tok::Minus,
            '*' => Tok::Star,
            '/' => Tok::Slash,
            '^' => Tok::Caret,
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            ',' => {
                return Err(ParseFailure::new(
                    "decimal_comma",
                    "This activity uses a point for decimals (for example 2.5), not a comma. Write one number only.",
                    Some(i),
                ))
            }
            c if c.is_alphabetic() => {
                return Err(ParseFailure::new(
                    "letters_not_allowed",
                    "This answer should be a number. Letters and variables are not accepted here.",
                    Some(i),
                ))
            }
            c => {
                return Err(ParseFailure::new(
                    "unexpected_symbol",
                    format!("The symbol “{c}” is not accepted. Use digits, a decimal point, + − × ÷ (or * /), ^ and parentheses."),
                    Some(i),
                ))
            }
        };
        out.push((tok, start, space));
        space = false;
        i += 1;
    }
    Ok(out)
}

fn literal_value(lit: &str, budget: &MathBudget) -> Result<Q, ParseFailure> {
    let (int_part, frac_part) = match lit.split_once('.') {
        Some((a, b)) => (a, b),
        None => (lit, ""),
    };
    let digits = format!("{}{}", if int_part.is_empty() { "0" } else { int_part }, frac_part);
    // ~3.33 bits per decimal digit; reject before allocating giant integers.
    if (digits.len() as u64) * 10 / 3 > budget.max_integer_bits + 4 {
        return Err(ParseFailure::budget(
            "integer_too_large",
            "This number is too large for the current checker.",
        ));
    }
    let n: BigInt = digits.parse().map_err(|_| {
        ParseFailure::new("bad_number", format!("“{lit}” is not a number."), None)
    })?;
    let d = BigInt::from(10).pow(frac_part.len() as u32);
    Ok(Q::new(n, d))
}

struct Parser<'a> {
    toks: &'a [(Tok, usize, bool)],
    pos: usize,
    nodes: usize,
    budget: &'a MathBudget,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos).map(|t| &t.0)
    }
    fn here(&self) -> Option<usize> {
        self.toks.get(self.pos).map(|t| t.1)
    }
    fn node(&mut self) -> Result<(), ParseFailure> {
        self.nodes += 1;
        if self.nodes > self.budget.max_nodes {
            return Err(ParseFailure::budget(
                "too_many_nodes",
                "This expression is too long for the current checker.",
            ));
        }
        Ok(())
    }
    fn depth(&self, d: usize) -> Result<(), ParseFailure> {
        if d > self.budget.max_depth {
            return Err(ParseFailure::budget(
                "too_deep",
                "This expression is nested too deeply for the current checker.",
            ));
        }
        Ok(())
    }

    fn expr(&mut self, d: usize) -> Result<Ast, ParseFailure> {
        self.depth(d)?;
        let mut lhs = self.term(d + 1)?;
        loop {
            match self.peek() {
                Some(Tok::Plus) => {
                    self.pos += 1;
                    let rhs = self.term(d + 1)?;
                    self.node()?;
                    lhs = Ast::Add { lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Some(Tok::Minus) => {
                    self.pos += 1;
                    let rhs = self.term(d + 1)?;
                    self.node()?;
                    lhs = Ast::Sub { lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                _ => return Ok(lhs),
            }
        }
    }

    fn term(&mut self, d: usize) -> Result<Ast, ParseFailure> {
        self.depth(d)?;
        let mut lhs = self.unary(d + 1)?;
        loop {
            match self.peek() {
                Some(Tok::Star) => {
                    self.pos += 1;
                    let rhs = self.unary(d + 1)?;
                    self.node()?;
                    lhs = Ast::Mul { lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Some(Tok::Slash) => {
                    self.pos += 1;
                    let rhs = self.unary(d + 1)?;
                    self.node()?;
                    lhs = Ast::Div { lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Some(Tok::Num(_)) | Some(Tok::LParen) => {
                    let (_, p, spaced) = &self.toks[self.pos];
                    let mixed = *spaced && matches!(self.peek(), Some(Tok::Num(_)));
                    return Err(if mixed {
                        ParseFailure::new(
                            "mixed_number_ambiguous",
                            "Two numbers are written side by side. For a mixed number such as one and three quarters, write 1+3/4 or 7/4.",
                            Some(*p),
                        )
                    } else {
                        ParseFailure::new(
                            "implicit_multiplication",
                            "Write multiplication explicitly, for example 2*(3+1) instead of 2(3+1).",
                            Some(*p),
                        )
                    });
                }
                _ => return Ok(lhs),
            }
        }
    }

    fn unary(&mut self, d: usize) -> Result<Ast, ParseFailure> {
        self.depth(d)?;
        match self.peek() {
            Some(Tok::Minus) => {
                self.pos += 1;
                let arg = self.unary(d + 1)?;
                self.node()?;
                Ok(Ast::Neg { arg: Box::new(arg) })
            }
            Some(Tok::Plus) => {
                self.pos += 1;
                self.unary(d + 1)
            }
            _ => self.power(d + 1),
        }
    }

    fn power(&mut self, d: usize) -> Result<Ast, ParseFailure> {
        self.depth(d)?;
        let base = self.primary(d + 1)?;
        if let Some(Tok::Caret) = self.peek() {
            self.pos += 1;
            let exp = self.unary(d + 1)?;
            self.node()?;
            return Ok(Ast::Pow { base: Box::new(base), exp: Box::new(exp) });
        }
        Ok(base)
    }

    fn primary(&mut self, d: usize) -> Result<Ast, ParseFailure> {
        self.depth(d)?;
        let pos = self.here();
        match self.peek().cloned() {
            Some(Tok::Num(lit)) => {
                self.pos += 1;
                self.node()?;
                let v = literal_value(&lit, self.budget)?;
                Ok(Ast::Num { value: crate::number::to_canonical(&v), raw: lit })
            }
            Some(Tok::LParen) => {
                self.pos += 1;
                if let Some(Tok::RParen) = self.peek() {
                    return Err(ParseFailure::new("empty_parentheses", "The parentheses are empty.", pos));
                }
                let inner = self.expr(d + 1)?;
                match self.peek() {
                    Some(Tok::RParen) => {
                        self.pos += 1;
                        if let Some(Tok::Num(_)) | Some(Tok::LParen) = self.peek() {
                            return Err(ParseFailure::new(
                                "implicit_multiplication",
                                "Write multiplication explicitly, for example (1+2)*3 instead of (1+2)3.",
                                self.here(),
                            ));
                        }
                        Ok(inner)
                    }
                    _ => Err(ParseFailure::new(
                        "unbalanced_parentheses",
                        "A parenthesis is opened but never closed.",
                        pos,
                    )),
                }
            }
            Some(Tok::RParen) => Err(ParseFailure::new(
                "unbalanced_parentheses",
                "There is a closing parenthesis without a matching opening one.",
                pos,
            )),
            Some(_) => Err(ParseFailure::new(
                "missing_operand",
                "An operator is missing a number on one side.",
                pos,
            )),
            None => Err(ParseFailure::new(
                "incomplete",
                "The expression ends too early — a number seems to be missing.",
                None,
            )),
        }
    }
}

/// Parses raw learner text under grammar `scalar-exact-v1`.
pub fn parse_scalar(raw: &str, budget: &MathBudget) -> Result<Ast, ParseFailure> {
    if raw.len() > budget.max_input_bytes {
        return Err(ParseFailure::budget("input_too_long", "This answer is too long for the current checker."));
    }
    let norm = normalize(raw);
    if norm.trim().is_empty() {
        return Err(ParseFailure::new("empty", "Enter an answer first.", None));
    }
    let toks = tokenize(&norm)?;
    let mut p = Parser { toks: &toks, pos: 0, nodes: 0, budget };
    let ast = p.expr(0)?;
    if p.pos < toks.len() {
        return Err(match toks[p.pos].0 {
            Tok::RParen => ParseFailure::new(
                "unbalanced_parentheses",
                "There is a closing parenthesis without a matching opening one.",
                Some(toks[p.pos].1),
            ),
            _ => ParseFailure::new("unexpected_token", "Part of this answer could not be read.", Some(toks[p.pos].1)),
        });
    }
    Ok(ast)
}

/// Evaluation failure: mathematical (undefined) versus budget (unsupported).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalFailure {
    pub code: String,
    pub message: String,
    pub budget: bool,
}

fn guard(q: Q, budget: &MathBudget) -> Result<Q, EvalFailure> {
    if bit_size(&q) > budget.max_integer_bits {
        return Err(EvalFailure {
            code: "integer_too_large".into(),
            message: "This value is too large for the current checker.".into(),
            budget: true,
        });
    }
    Ok(q)
}

/// Exact evaluation over Q with explicit conventions:
/// division by zero is undefined; `0^0` is rejected as undefined in this grammar;
/// exponents must be integers with bounded result size.
pub fn evaluate(ast: &Ast, budget: &MathBudget) -> Result<Q, EvalFailure> {
    let r = match ast {
        Ast::Num { value, .. } => crate::number::from_canonical(value).ok_or(EvalFailure {
            code: "bad_literal".into(),
            message: "Invalid literal.".into(),
            budget: false,
        })?,
        Ast::Neg { arg } => -evaluate(arg, budget)?,
        Ast::Add { lhs, rhs } => evaluate(lhs, budget)? + evaluate(rhs, budget)?,
        Ast::Sub { lhs, rhs } => evaluate(lhs, budget)? - evaluate(rhs, budget)?,
        Ast::Mul { lhs, rhs } => evaluate(lhs, budget)? * evaluate(rhs, budget)?,
        Ast::Div { lhs, rhs } => {
            let n = evaluate(lhs, budget)?;
            let d = evaluate(rhs, budget)?;
            if d.is_zero() {
                return Err(EvalFailure {
                    code: "division_by_zero".into(),
                    message: "This expression divides by zero, so it has no value.".into(),
                    budget: false,
                });
            }
            n / d
        }
        Ast::Pow { base, exp } => {
            let b = evaluate(base, budget)?;
            let e = evaluate(exp, budget)?;
            if !e.denom().is_one() {
                return Err(EvalFailure {
                    code: "non_integer_exponent".into(),
                    message: "Only whole-number exponents are accepted in this activity.".into(),
                    budget: false,
                });
            }
            if b.is_zero() && !e.is_positive() {
                return Err(EvalFailure {
                    code: "zero_power_undefined".into(),
                    message: "0 raised to zero or a negative power is not defined in this activity.".into(),
                    budget: false,
                });
            }
            let mag = e.numer().abs();
            let est = (bit_size(&b).max(1) as u128) * mag.to_u128().unwrap_or(u128::MAX);
            if est > budget.max_integer_bits as u128 && !b.numer().abs().is_one() || !b.denom().is_one() && est > budget.max_integer_bits as u128 {
                return Err(EvalFailure {
                    code: "integer_too_large".into(),
                    message: "This power is too large for the current checker.".into(),
                    budget: true,
                });
            }
            let k = mag.to_u32().unwrap_or(u32::MAX);
            let n = num_traits::pow::pow(b.numer().clone(), k as usize);
            let d = num_traits::pow::pow(b.denom().clone(), k as usize);
            let p = Q::new(n, d);
            if e.is_negative() { Q::one() / p } else { p }
        }
    };
    guard(r, budget)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::{frac, int, to_canonical};

    fn val(s: &str) -> Q {
        let b = MathBudget::default();
        evaluate(&parse_scalar(s, &b).unwrap(), &b).unwrap()
    }
    fn perr(s: &str) -> String {
        parse_scalar(s, &MathBudget::default()).unwrap_err().code
    }
    fn eerr(s: &str) -> String {
        let b = MathBudget::default();
        evaluate(&parse_scalar(s, &b).unwrap(), &b).unwrap_err().code
    }

    #[test]
    fn exact_values() {
        assert_eq!(val("1/2"), val("2/4"));
        assert_eq!(val("0.1 + 0.2"), val("0.3"));
        assert_eq!(val("3/-6"), frac(-1, 2));
        assert_eq!(val("-(3/4)"), frac(-3, 4));
        assert_eq!(val("2^10"), int(1024));
        assert_eq!(val("2^-2"), frac(1, 4));
        assert_eq!(val("(1/2)^3"), frac(1, 8));
        assert_eq!(val("-2^2"), int(-4)); // power binds tighter than unary minus
        assert_eq!(val(".5"), frac(1, 2));
        assert_eq!(val("7 − 2 × 3"), int(1));
        assert_eq!(val("12 ÷ 4"), int(3));
        assert_eq!(val("1 + 2 * 3"), int(7));
        assert_eq!(val("8 - 3 - 2"), int(3));
        assert_eq!(val("24 / 4 / 2"), int(3));
        assert_eq!(val("2^3^2"), int(512));
    }

    #[test]
    fn long_integer_stays_exact() {
        let s = "99999999999999999999999999999999 + 1";
        assert_eq!(to_canonical(&val(s)), "100000000000000000000000000000000");
    }

    #[test]
    fn division_by_zero_rejected() {
        assert_eq!(eerr("1/0"), "division_by_zero");
        assert_eq!(eerr("1/(2-2)"), "division_by_zero");
        assert_eq!(eerr("0^0"), "zero_power_undefined");
        assert_eq!(eerr("4^(1/2)"), "non_integer_exponent");
    }

    #[test]
    fn ambiguity_is_rejected_with_clarification() {
        assert_eq!(perr("1 3/4"), "mixed_number_ambiguous");
        assert_eq!(perr("2(3)"), "implicit_multiplication");
        assert_eq!(perr("(2)3"), "implicit_multiplication");
        assert_eq!(perr("(1)(2)"), "implicit_multiplication");
        assert_eq!(perr("1/2x"), "letters_not_allowed");
        assert_eq!(perr("2,5"), "decimal_comma");
        assert_eq!(perr("1..2"), "bad_decimal");
        assert_eq!(perr("5."), "bad_decimal");
        assert_eq!(perr("(1+2"), "unbalanced_parentheses");
        assert_eq!(perr("1+2)"), "unbalanced_parentheses");
        assert_eq!(perr("1+"), "incomplete");
        assert_eq!(perr("*3"), "missing_operand");
        assert_eq!(perr("   "), "empty");
        assert_eq!(perr("1 $ 2"), "unexpected_symbol");
    }

    #[test]
    fn budgets_are_unsupported_not_incorrect() {
        let b = MathBudget::default();
        let long = "1+".repeat(5000) + "1";
        let e = parse_scalar(&long, &b).unwrap_err();
        assert!(e.budget);
        let deep = "(".repeat(100) + "1" + &")".repeat(100);
        let e = parse_scalar(&deep, &b).unwrap_err();
        assert!(e.budget, "{e:?}");
        let ast = parse_scalar("10^100000", &b).unwrap();
        let e = evaluate(&ast, &b).unwrap_err();
        assert!(e.budget);
        let huge = "9".repeat(2000);
        assert!(parse_scalar(&huge, &b).unwrap_err().budget);
        // 1^huge is fine and cheap.
        assert_eq!(val("1^100000"), int(1));
    }

    #[test]
    fn simple_fraction_form() {
        let b = MathBudget::default();
        let a = parse_scalar("6/8", &b).unwrap();
        assert_eq!(a.as_simple_fraction().map(|(n, d)| (n.to_string(), d.to_string())), Some(("6".into(), "8".into())));
        assert!(parse_scalar("0.75", &b).unwrap().as_simple_fraction().is_none());
        assert!(parse_scalar("12/2/2", &b).unwrap().as_simple_fraction().is_none());
    }
}
