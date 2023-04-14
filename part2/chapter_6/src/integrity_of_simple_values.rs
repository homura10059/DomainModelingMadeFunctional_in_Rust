use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Clone)]
struct UnitQuantity(i64);

impl UnitQuantity {
    fn try_new(quantity: i64) -> Result<Self> {
        if quantity < 1 {
            return Err(anyhow!("UnitQuantity can not be negative"));
        } else if quantity > 1_000 {
            return Err(anyhow!("UnitQuantity can not be more then 1_000"));
        }
        Ok(UnitQuantity(quantity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_quantity() {
        let actual = UnitQuantity::try_new(100).unwrap();
        assert_eq!(actual, UnitQuantity(100));
    }

    #[test]
    fn test_unit_quantity_negative() {
        let actual = UnitQuantity::try_new(-1);
        assert!(actual.is_err());
    }
}
