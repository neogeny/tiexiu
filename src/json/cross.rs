// Conversions between serde_json::Value and json::JsonValue

use json::JsonValue;
use serde_json::Value as SerdeValue;

pub fn serde_to_json(v: SerdeValue) -> JsonValue {
    match v {
        SerdeValue::Null => JsonValue::Null,
        SerdeValue::Bool(b) => JsonValue::Boolean(b),
        SerdeValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                JsonValue::Number(i.into())
            } else if let Some(u) = n.as_u64() {
                JsonValue::Number(u.into())
            } else {
                JsonValue::String(n.to_string())
            }
        }
        SerdeValue::String(s) => JsonValue::String(s),
        SerdeValue::Array(arr) => {
            let mut result = JsonValue::new_array();
            for item in arr {
                let _ = result.push(serde_to_json(item));
            }
            result
        }
        SerdeValue::Object(obj) => {
            let mut result = JsonValue::new_object();
            for (key, value) in obj {
                result[key] = serde_to_json(value);
            }
            result
        }
    }
}

pub fn json_to_serde(v: JsonValue) -> SerdeValue {
    match v {
        JsonValue::Null => SerdeValue::Null,
        JsonValue::Boolean(b) => SerdeValue::Bool(b),
        JsonValue::Number(n) => {
            if let Ok(i) = n.try_into() as Result<i64, _> {
                SerdeValue::Number(i.into())
            } else if let Ok(u) = n.try_into() as Result<u64, _> {
                SerdeValue::Number(u.into())
            } else {
                let f: f64 = n.into();
                if let Some(sn) = serde_json::Number::from_f64(f) {
                    SerdeValue::Number(sn)
                } else {
                    SerdeValue::Null
                }
            }
        }
        JsonValue::String(s) => SerdeValue::String(s),
        JsonValue::Short(s) => SerdeValue::String(s.to_string()),
        JsonValue::Array(arr) => {
            let vec: Vec<SerdeValue> = arr.into_iter().map(json_to_serde).collect();
            SerdeValue::Array(vec)
        }
        JsonValue::Object(obj) => {
            let json_str = obj.dump();
            serde_json::from_str(&json_str).unwrap_or(SerdeValue::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CALC_JSON: &str = include_str!("../../grammar/calc.json");

    #[test]
    fn test_serde_to_json_null() {
        let serde_val = serde_json::Value::Null;
        let json_val = serde_to_json(serde_val);
        assert!(json_val.is_null());
    }

    #[test]
    fn test_serde_to_json_bool() {
        let serde_val = serde_json::Value::Bool(true);
        let json_val = serde_to_json(serde_val);
        assert_eq!(json_val.as_bool(), Some(true));
    }

    #[test]
    fn test_serde_to_json_number_i64() {
        let serde_val = serde_json::Number::from(42_i64);
        let json_val = serde_to_json(serde_json::Value::Number(serde_val));
        assert_eq!(json_val.as_i64(), Some(42));
    }

    #[test]
    fn test_serde_to_json_string() {
        let serde_val = serde_json::Value::String("hello".to_string());
        let json_val = serde_to_json(serde_val);
        assert_eq!(json_val.as_str(), Some("hello"));
    }

    #[test]
    fn test_serde_to_json_array() {
        let serde_val = serde_json::json!([1, 2, "three"]);
        let json_val = serde_to_json(serde_val);
        assert!(json_val.is_array());
        assert_eq!(json_val.len(), 3);
    }

    #[test]
    fn test_serde_to_json_object() {
        let serde_val = serde_json::json!({"key": "value", "num": 42});
        let json_val = serde_to_json(serde_val);
        assert!(json_val.is_object());
        assert_eq!(json_val["key"].as_str(), Some("value"));
        assert_eq!(json_val["num"].as_i64(), Some(42));
    }

    #[test]
    fn test_json_to_serde_null() {
        let json_val = JsonValue::Null;
        let serde_val = json_to_serde(json_val);
        assert!(serde_val.is_null());
    }

    #[test]
    fn test_json_to_serde_bool() {
        let json_val = JsonValue::Boolean(false);
        let serde_val = json_to_serde(json_val);
        assert_eq!(serde_val.as_bool(), Some(false));
    }

    #[test]
    fn test_json_to_serde_number() {
        let json_val = JsonValue::from(std::f64::consts::PI);
        let serde_val = json_to_serde(json_val);
        assert!(serde_val.is_number());
    }

    #[test]
    fn test_json_to_serde_string() {
        let json_val = JsonValue::String("test".into());
        let serde_val = json_to_serde(json_val);
        assert_eq!(serde_val.as_str(), Some("test"));
    }

    #[test]
    fn test_roundtrip_calc_json() {
        let json_val = json::parse(CALC_JSON).expect("parse calc.json");
        let serde_val = json_to_serde(json_val);
        let roundtrip_json = serde_to_json(serde_val);
        assert_eq!(roundtrip_json["directives"]["grammar"], "CALC");
        assert_eq!(roundtrip_json["name"], "CALC");
        assert!(roundtrip_json["rules"].is_array());
        assert_eq!(roundtrip_json["rules"].len(), 9);
    }

    #[test]
    fn test_serde_parse_then_convert() {
        let serde_val: SerdeValue = serde_json::from_str(CALC_JSON).expect("parse calc.json");
        let json_val = serde_to_json(serde_val);
        assert_eq!(json_val["name"], "CALC");
        assert!(json_val["rules"].is_array());
    }
}
