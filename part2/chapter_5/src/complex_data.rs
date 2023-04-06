use std::error::Error;

type Undefined = dyn Error;

type CustomerInfo = Undefined;
type SippingAddress = Undefined;
type BillingAddress = Undefined;
type OrderLine = Undefined;
type BillingAmount = Undefined;

struct Order {
    customer_info: CustomerInfo,
    sipping_address: SippingAddress,
    billing_address: BillingAddress,
    order_lines: Vec<OrderLine>,
    billing_amount: BillingAmount,
}

enum ProductCode {
    WidgetCode(i32),
    GizmoCode(i32),
}

enum OrderQuantity {
    UnitQuantity(i32),
    Kilogram(i32),
}
