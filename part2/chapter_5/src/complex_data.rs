struct CustomerInfo();
struct SippingAddress();
struct BillingAddress();
struct OrderLine();
struct BillingAmount();

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
