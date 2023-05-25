use anyhow::{bail, Result};

#[derive(Debug, PartialEq)]
struct OrderId(i64);

impl OrderId {
    fn try_new(id_str: String) -> Result<Self> {
        if id_str.is_empty() {
            bail!("id_str is empty");
        }

        if id_str.len() > 50 {
            bail!("id_str is too long");
        }
        let id = id_str.parse::<i64>()?;
        Ok(Self(id))
    }

    fn value(&self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_order_id() {
        let actual = OrderId::try_new("42".to_string()).unwrap();
        assert_eq!(actual, OrderId(42));
    }

    proptest! {
        #[test]
        fn doesnt_crash(s: i64) {
            OrderId::try_new(s.to_string()).unwrap();
        }
    }
}
