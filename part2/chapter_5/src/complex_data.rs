use std::error::Error;

type Undefined = dyn Error;

type CustomerInfo = Undefined;

struct Order {
    customer_info: CustomerInfo,
}
