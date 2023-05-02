use crate::main_api::{
    PlaceOrderCommand, PlaceOrderError, PlaceOrderEvent, UnvalidatedAddress, UnvalidatedCustomer,
    UnvalidatedOrder,
};

// -------------------------------------------
// Order life cycle
// -------------------------------------------

struct CustomerInfo {}

struct Address {}

// validate state
struct ValidatedOrderLine {}
struct ValidatedOrder {
    order_id: String,
    customer_info: CustomerInfo,
    sipping_address: Address,
    billing_address: Address,
    order_lines: Vec<ValidatedOrderLine>,
}

// priced state
struct PricedOrderLine {}
struct PricedOrder {
    order_id: String,
    customer_info: CustomerInfo,
    sipping_address: Address,
    billing_address: Address,
    order_lines: Vec<PricedOrderLine>,
}

// all states combined
enum Order {
    Unvalidated(UnvalidatedOrder),
    Validated(ValidatedOrder),
    Priced(PricedOrder),
}

// -------------------------------------------
// Definitions of Internal Step
// -------------------------------------------

struct ProductCode {}

// ----- Validate order -----

// services used by validate_order
trait CheckProductCodeExists {
    fn check_product_code_exists(product_code: &ProductCode) -> bool;
}

struct AddressValidationError {}

struct CheckedAddress {}

trait CheckAddressExists {
    fn check_address_exists(
        address: &UnvalidatedAddress,
    ) -> Result<CheckedAddress, AddressValidationError>;
}

struct ValidationError {}

trait ValidateOrder {
    fn validate_order(
        check_product_code_exists: impl CheckProductCodeExists,
        check_address_exists: impl CheckAddressExists,
        order: UnvalidatedOrder,
    ) -> Result<ValidatedOrder, Vec<ValidationError>>;
}

// ----- Price order -----

// services used by price_order

trait GetProductPrice {
    fn get_product_price(product_code: &ProductCode) -> f32;
}

struct PricingError {}

trait PriceOrder {
    fn price_order(
        get_product_price: impl GetProductPrice,
        order: ValidatedOrder,
    ) -> Result<PricedOrder, PricingError>;
}
