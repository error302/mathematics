//! Exact rational values with canonical string forms (Section 13.1).

use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

pub type Q = BigRational;

/// Canonical wire form: `"n"` for integers, `"n/d"` otherwise, positive denominator, gcd 1.
pub fn to_canonical(q: &Q) -> String {
    if q.denom().is_one() {
        q.numer().to_string()
    } else {
        format!("{}/{}", q.numer(), q.denom())
    }
}

/// Parses a canonical wire string (`"-3"`, `"6/8"` is normalized to `"3/4"`).
pub fn from_canonical(s: &str) -> Option<Q> {
    let (n, d) = match s.split_once('/') {
        Some((n, d)) => (n, d),
        None => (s, "1"),
    };
    if !is_int_literal(n) || !d.bytes().all(|b| b.is_ascii_digit()) || d.is_empty() {
        return None;
    }
    let n: BigInt = n.parse().ok()?;
    let d: BigInt = d.parse().ok()?;
    if d.is_zero() {
        return None;
    }
    Some(Q::new(n, d))
}

fn is_int_literal(s: &str) -> bool {
    let body = s.strip_prefix('-').unwrap_or(s);
    !body.is_empty() && body.bytes().all(|b| b.is_ascii_digit())
}

pub fn int(n: i64) -> Q {
    Q::from_integer(BigInt::from(n))
}

pub fn frac(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

/// LaTeX rendering of an exact value (for prompts/feedback; never used for checking).
pub fn to_latex(q: &Q) -> String {
    if q.denom().is_one() {
        q.numer().to_string()
    } else if q.is_negative() {
        format!("-\\frac{{{}}}{{{}}}", -q.numer(), q.denom())
    } else {
        format!("\\frac{{{}}}{{{}}}", q.numer(), q.denom())
    }
}

/// Plain-language speech rendering, e.g. "3 over 4".
pub fn to_speech(q: &Q) -> String {
    if q.denom().is_one() {
        q.numer().to_string()
    } else {
        format!("{} over {}", q.numer(), q.denom())
    }
}

/// Finite decimal string if the value has a terminating expansion (denominator 2^a 5^b).
pub fn to_finite_decimal(q: &Q) -> Option<String> {
    let mut d = q.denom().clone();
    let two = BigInt::from(2);
    let five = BigInt::from(5);
    let mut twos = 0u32;
    let mut fives = 0u32;
    while d.is_even() {
        d /= &two;
        twos += 1;
    }
    while (&d % &five).is_zero() {
        d /= &five;
        fives += 1;
    }
    if !d.is_one() {
        return None;
    }
    let places = twos.max(fives);
    if places == 0 {
        return Some(q.numer().to_string());
    }
    let scale = BigInt::from(10).pow(places);
    let scaled = (q * Q::from_integer(scale.clone())).to_integer();
    let neg = scaled.is_negative();
    let digits = scaled.abs().to_string();
    let p = places as usize;
    let padded = if digits.len() <= p {
        format!("{}{}", "0".repeat(p - digits.len() + 1), digits)
    } else {
        digits
    };
    let (i, f) = padded.split_at(padded.len() - p);
    Some(format!("{}{}.{}", if neg { "-" } else { "" }, i, f))
}

/// Bit length of the largest of numerator/denominator magnitudes.
pub fn bit_size(q: &Q) -> u64 {
    q.numer().bits().max(q.denom().bits())
}

pub fn to_i64(q: &Q) -> Option<i64> {
    if q.denom().is_one() {
        q.numer().to_i64()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_normalizes() {
        assert_eq!(to_canonical(&frac(2, 4)), "1/2");
        assert_eq!(to_canonical(&frac(3, -6)), "-1/2");
        assert_eq!(to_canonical(&frac(-4, -2)), "2");
        assert_eq!(from_canonical("6/8").unwrap(), frac(3, 4));
        assert!(from_canonical("1/0").is_none());
        assert!(from_canonical("1/-2").is_none());
        assert!(from_canonical("").is_none());
        assert!(from_canonical("--1").is_none());
    }

    #[test]
    fn long_integers_stay_exact() {
        let s = "123456789012345678901234567890123456789";
        let q = from_canonical(s).unwrap();
        assert_eq!(to_canonical(&(q.clone() + int(1))), "123456789012345678901234567890123456790");
    }

    #[test]
    fn finite_decimals() {
        assert_eq!(to_finite_decimal(&frac(3, 4)).unwrap(), "0.75");
        assert_eq!(to_finite_decimal(&frac(-1, 8)).unwrap(), "-0.125");
        assert_eq!(to_finite_decimal(&frac(1, 3)), None);
        assert_eq!(to_finite_decimal(&int(15)).unwrap(), "15");
        assert_eq!(to_finite_decimal(&frac(3, 10)).unwrap(), "0.3");
    }

    #[test]
    fn latex() {
        assert_eq!(to_latex(&frac(-3, 4)), "-\\frac{3}{4}");
        assert_eq!(to_latex(&int(7)), "7");
    }
}
