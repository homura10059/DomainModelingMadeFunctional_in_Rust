use crate::address::UnvalidatedAddress;
use crate::customer::UnvalidatedCustomer;
use anyhow::{bail, Error, Result};

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

type ValidationError = Error;

pub struct UnvalidatedOrder {
    order_id: i64,
    customer_info: UnvalidatedCustomer,
    sipping_address: UnvalidatedAddress,
}

struct ValidatedOrderLine {}
struct ValidatedOrder {
    order_id: String,
    customer_info: CustomerInfo,
    sipping_address: Address,
    billing_address: Address,
    order_lines: Vec<ValidatedOrderLine>,
}

trait ValidateOrder {
    fn validate_order(
        check_product_code_exists: impl CheckProductCodeExists,
        check_address_exists: impl CheckAddressExists,
        unvalidated_order: UnvalidatedOrder,
    ) -> Result<ValidatedOrder, Vec<ValidationError>>;
}

struct ValidateOrderImpl;
impl ValidateOrder for ValidatedOrderImple {
    fn validate_order(
        check_product_code_exists: impl CheckProductCodeExists,
        check_address_exists: impl CheckAddressExists,
        unvalidated_order: UnvalidatedOrder,
    ) -> Result<ValidatedOrder, Vec<ValidationError>> {
        let order_id = unvalidated_order.order_id;
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
