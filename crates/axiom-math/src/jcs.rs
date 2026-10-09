//! RFC 8785 JSON Canonicalization Scheme (JCS) for AXIOM artifacts (Section 14.2).
//!
//! AXIOM policy: arbitrary-precision quantities travel as canonical decimal strings,
//! so artifacts only contain integer JSON numbers within the IEEE-754 safe range.
//! Non-integer or out-of-range numbers are rejected rather than reformatted.

use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CanonicalError {
    #[error("non-integer or unsafe JSON number {0}; encode exact values as strings")]
    UnsupportedNumber(String),
}

const MAX_SAFE: i64 = 9_007_199_254_740_991;

/// Serializes a JSON value to its canonical JCS form.
pub fn canonicalize(value: &Value) -> Result<String, CanonicalError> {
    let mut out = String::new();
    write_value(value, &mut out)?;
    Ok(out)
}

/// SHA-256 over the UTF-8 canonical bytes, as lowercase hex.
pub fn canonical_hash(value: &Value) -> Result<String, CanonicalError> {
    Ok(sha256_hex(canonicalize(value)?.as_bytes()))
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn write_value(v: &Value, out: &mut String) -> Result<(), CanonicalError> {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if !(-MAX_SAFE..=MAX_SAFE).contains(&i) {
                    return Err(CanonicalError::UnsupportedNumber(n.to_string()));
                }
                out.push_str(&i.to_string());
            } else if let Some(u) = n.as_u64() {
                if u > MAX_SAFE as u64 {
                    return Err(CanonicalError::UnsupportedNumber(n.to_string()));
                }
                out.push_str(&u.to_string());
            } else {
                return Err(CanonicalError::UnsupportedNumber(n.to_string()));
            }
        }
        Value::String(s) => write_string(s, out),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            // JCS sorts property names by their UTF-16 code units.
            let mut keys: Vec<(&String, Vec<u16>)> =
                map.keys().map(|k| (k, k.encode_utf16().collect())).collect();
            keys.sort_by(|a, b| a.1.cmp(&b.1));
            out.push('{');
            for (i, (k, _)) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string(k, out);
                out.push(':');
                write_value(&map[*k], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

/// ECMAScript `JSON.stringify` string escaping, as required by JCS.
fn write_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Parses JSON text while rejecting duplicate object keys (Section 14.2).
pub fn parse_strict(text: &str) -> Result<Value, String> {
    use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
    use std::fmt;

    struct Strict;
    impl<'de> DeserializeSeed<'de> for Strict {
        type Value = Value;
        fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
            d.deserialize_any(StrictVisitor)
        }
    }
    struct StrictVisitor;
    impl<'de> Visitor<'de> for StrictVisitor {
        type Value = Value;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("JSON value")
        }
        fn visit_bool<E>(self, v: bool) -> Result<Value, E> {
            Ok(Value::Bool(v))
        }
        fn visit_i64<E>(self, v: i64) -> Result<Value, E> {
            Ok(Value::from(v))
        }
        fn visit_u64<E>(self, v: u64) -> Result<Value, E> {
            Ok(Value::from(v))
        }
        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Value, E> {
            serde_json::Number::from_f64(v)
                .map(Value::Number)
                .ok_or_else(|| E::custom("non-finite number"))
        }
        fn visit_str<E>(self, v: &str) -> Result<Value, E> {
            Ok(Value::String(v.to_owned()))
        }
        fn visit_string<E>(self, v: String) -> Result<Value, E> {
            Ok(Value::String(v))
        }
        fn visit_unit<E>(self) -> Result<Value, E> {
            Ok(Value::Null)
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
            let mut v = Vec::new();
            while let Some(item) = seq.next_element_seed(Strict)? {
                v.push(item);
            }
            Ok(Value::Array(v))
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
            let mut m = serde_json::Map::new();
            while let Some(key) = map.next_key::<String>()? {
                if m.contains_key(&key) {
                    return Err(serde::de::Error::custom(format!("duplicate key `{key}`")));
                }
                let val = map.next_value_seed(Strict)?;
                m.insert(key, val);
            }
            Ok(Value::Object(m))
        }
    }

    let mut de = serde_json::Deserializer::from_str(text);
    let v = Strict.deserialize(&mut de).map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sorts_keys_and_strips_whitespace() {
        let v = json!({"b": 1, "a": [true, null, "x"], "c": {"z": 0, "y": -3}});
        assert_eq!(
            canonicalize(&v).unwrap(),
            r#"{"a":[true,null,"x"],"b":1,"c":{"y":-3,"z":0}}"#
        );
    }

    #[test]
    fn utf16_key_order() {
        // RFC 8785 §3.2.3 example ordering: "\r" < "1" < "\u0080" < "ö" < "€" < "😀" < "ﬂ"
        let v = json!({"\u{20ac}": 1, "\r": 2, "\u{fb33}": 3, "1": 4, "\u{1f600}": 5, "\u{80}": 6, "\u{f6}": 7});
        let c = canonicalize(&v).unwrap();
        let order: Vec<usize> = ["\\r", "\"1\"", "\u{80}", "\u{f6}", "\u{20ac}", "\u{1f600}", "\u{fb33}"]
            .iter()
            .map(|k| c.find(k).unwrap())
            .collect();
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(order, sorted);
    }

    #[test]
    fn escapes_like_ecmascript() {
        let v = json!("a\"b\\c\n\u{1}\u{7f}é");
        assert_eq!(canonicalize(&v).unwrap(), "\"a\\\"b\\\\c\\n\\u0001\u{7f}é\"");
    }

    #[test]
    fn rejects_floats_and_unsafe_integers() {
        assert!(canonicalize(&json!(1.5)).is_err());
        assert!(canonicalize(&json!(9_007_199_254_740_992u64)).is_err());
        assert!(canonicalize(&json!(9_007_199_254_740_991u64)).is_ok());
    }

    #[test]
    fn strict_parse_rejects_duplicates() {
        assert!(parse_strict(r#"{"a":1,"a":2}"#).is_err());
        assert!(parse_strict(r#"{"a":{"b":1,"b":1}}"#).is_err());
        assert!(parse_strict(r#"{"a":1,"b":[{"c":2}]}"#).is_ok());
        assert!(parse_strict(r#"{"a":1} x"#).is_err());
    }

    #[test]
    fn known_hash() {
        // sha256("{}") computed independently.
        assert_eq!(
            canonical_hash(&json!({})).unwrap(),
            "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
        );
    }
}
