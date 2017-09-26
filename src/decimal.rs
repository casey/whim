use serde::{de, Deserialize, Serialize, Deserializer, Serializer};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use extprim::u128::u128;
use std::u64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Decimal {
  units: u128,
}

static SCALING_EXPONENT: u32 = 18;

fn scaling_factor() -> u128 {
  ten().pow(SCALING_EXPONENT)
}

fn ten() -> u128 {
  u128::new(10)
}

fn zero() -> u128 {
  u128::zero()
}

impl Decimal {
  fn from_units(units: u128) -> Decimal {
    Decimal{units}
  }

}

impl Display for Decimal {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let scaling_factor  = scaling_factor();
    let ten             = ten();
    let zero            = zero();
    let whole          = self.units / scaling_factor;
    let mut fractional = self.units % scaling_factor;
    let mut width       = SCALING_EXPONENT;
    while width > 1 && fractional % ten == zero {
      fractional /= ten;
      width -= 1;
    }
    write!(f, "{}.{:0>width$}", whole, fractional, width=width as usize)
  }
}

impl FromStr for Decimal {
  type Err = (char, usize);
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let zero = zero();
    let ten = ten();

    let mut after_decimal_point = false;
    let mut position            = 0;
    let mut n                   = zero;
    let mut exponent_reduction  = 0;

    for character in s.trim_right_matches('0').chars() {
      match character {
        '0'...'9' => {
          let value = character as u64 - '0' as u64;
          n =  n * ten + u128::new(value);
          if after_decimal_point {
            exponent_reduction += 1;
          }
        }
        '.' if !after_decimal_point => after_decimal_point = true,
        _ => {
          return Err((character, position));
        }
      }
      position += 1;
    }

    Ok(Decimal::from_units(n * ten.pow(SCALING_EXPONENT - exponent_reduction)))
  }
}

impl Serialize for Decimal {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: Serializer
  {
    serializer.collect_str(&self)
  }
}

impl<'d> Deserialize<'d> for Decimal{
  fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
    deserializer.deserialize_str(DecimalVisitor)
  }
}

struct DecimalVisitor;

impl<'de> de::Visitor<'de> for DecimalVisitor {
  type Value = Decimal;

  fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "text of the form 1234.5678")
  }

  fn visit_str<E>(self, value: &str) -> Result<Decimal, E>
    where E: de::Error
  {
    value.parse().map_err(|(character, position)| {
      de::Error::custom(format!("bad character in decimal at position {}: {:?}", position, character))
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn from_digits_places(digits: u64, decimal_places: u32) -> Decimal {
    let digits = u128::new(digits);
    let units = digits * ten().pow(SCALING_EXPONENT - decimal_places);
    Decimal{units}
  }

  fn test_format(digits: u64, decimal_places: u32, expected: &str) -> String {
    let input = Decimal::from_digits_places(digits, decimal_places);
    let formatted = input.to_string();
    assert_eq!(formatted, expected, "formatting {:?} failed: {} != {}", input, formatted, expected);
    formatted
  }

  fn test_parse(input: &str, digits: u64, decimal_places: u32) -> Decimal {
    let parsed: Decimal = input.parse().unwrap();
    let expected = Decimal::from_digits_places(digits, decimal_places);
    assert_eq!(parsed, expected, "deserializing {} failed: {:?} != {:?}", input, parsed, expected);
    parsed
  }

  fn test_round_trip(digits: u64, decimal_places: u32) {
    let input = Decimal::from_digits_places(digits, decimal_places);
    let output: Decimal = input.to_string().parse().unwrap();
    assert_eq!(output, input, "round-trip failed, input {:?} != output {:?}", input, output);
  }

  fn test_error(raw: &str, character: char, position: usize) {
    let actual_result: Result<Decimal, _> = raw.parse();
    let expected_result = Err((character, position));
    assert_eq!(actual_result, expected_result, "unexpected result: actual {:?} != expected {:?}", actual_result, expected_result);
  }

  fn test(raw: &str, digits: u64, decimal_places: u32, formatted: &str) {
    test_parse(raw, digits, decimal_places);
    test_format(digits, decimal_places, formatted);
    test_round_trip(digits, decimal_places);
  }

  #[test]
  fn zero() {
    test("0.0"               , 0, 0, "0.0");
    test("000000.00000000000", 0, 0, "0.0");
    test("0."                , 0, 0, "0.0");
    test(".0"                , 0, 0, "0.0");
    test("."                 , 0, 0, "0.0");
  }

  #[test]
  fn one() {
    test("1.0", 1, 0, "1.0");
  }

  #[test]
  fn mixed() {
    test("1.5"     , 15,     1, "1.5");
    test("10000.50", 100005, 1, "10000.5");
  }

  #[test]
  fn fraction() {
    test("0.50"    , 5, 1, "0.5"    );
    test("0.050"   , 5, 2, "0.05"   );
    test("0.0050"  , 5, 3, "0.005"  );
    test("0.00050" , 5, 4, "0.0005" );
    test("0.000050", 5, 5, "0.00005");
  }

  #[test]
  fn big() {
    test("123456789.987654321", 123456789987654321, 9, "123456789.987654321");
    test("0000000000000000000000000000000.00000000000000000000000", 0, 0, "0.0");
  }

  #[test]
  fn errors() {
    test_error("x"      , 'x', 0);
    test_error("0.0.0"  , '.', 3);
    test_error("123.ayz", 'a', 4);
    test_error(" 1"     , ' ', 0);
    test_error("1 "     , ' ', 1);
  }

  #[test]
  fn limits() {
    let max = scaling_factor() * scaling_factor();
    assert!(max < u128::max_value());
  }
}
