// -------------------------------------------
// Input data
// -------------------------------------------
pub struct UnvalidatedOrder {
    order_id: i32,
    customer_info: UnvalidatedCustomer,
    sipping_address: UnvalidatedAddress,
}

pub struct UnvalidatedCustomer {
    name: String,
    email: String,
}

pub struct UnvalidatedAddress {
    street: String,
    city: String,
    state: String,
    zip: String,
}

// -------------------------------------------
// Input Command
// -------------------------------------------
pub struct Command<Data> {
    data: Data,
    timestamp: String,
    user_id: String,
}

pub type PlaceOrderCommand = Command<UnvalidatedOrder>;

// -------------------------------------------
// Public API
// -------------------------------------------

pub struct OrderPlaced {
    order_id: i32,
}

pub struct BillableOrderPlaced {
    order_id: i32,
}

pub struct OrderAcknowledgementSent {
    order_id: i32,
}

/// Success output of PlaceOrder workflow
pub enum PlaceOrderEvent {
    OrderPlaced(OrderPlaced),
    BillableOrderPlaced(BillableOrderPlaced),
    OrderAcknowledgementSent(OrderAcknowledgementSent),
}

/// Failure output of PlaceOrder workflow
pub struct PlaceOrderError {
    order_id: i32,
    reason: String,
}

pub trait PlaceOrderWorkflow {
    fn place_order(command: PlaceOrderCommand) -> Result<Vec<PlaceOrderEvent>, PlaceOrderError>;
}
