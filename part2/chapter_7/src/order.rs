#[derive(Debug, PartialEq, Clone)]
struct EmailAddress(String);

#[derive(Debug, PartialEq, Clone)]
struct VerifiedEmailAddress(String);

enum CustomerEmail {
    Verified(VerifiedEmailAddress),
    Unverified(EmailAddress),
}

struct EmailContactInfo {
    email: VerifiedEmailAddress,
}

struct PostalContactInfo {
    postal_code: String,
}

enum ContactInfo {
    Email(EmailContactInfo),
    Postal(PostalContactInfo),
    Both(EmailContactInfo, PostalContactInfo),
}

struct Contact {
    name: String,
    contact_info: ContactInfo,
}

#[derive(Debug, PartialEq, Clone)]
struct UnvalidatedAddress(String);

#[derive(Debug, PartialEq, Clone)]
struct ValidatedAddress(String);

struct UnvalidatedCustomerInfo {}
struct ValidatedCustomerInfo {}

struct UnvalidatedOrder {
    order_id: String,
    customer_info: UnvalidatedCustomerInfo,
    sipping_address: UnvalidatedAddress,
}

struct ValidatedOrder {
    order_id: String,
    customer_info: ValidatedCustomerInfo,
    sipping_address: ValidatedAddress,
}

struct Command<Data> {
    data: Data,
    timestamp: String,
    user_id: String,
}

type PlaceOrder = Command<UnvalidatedOrder>;
type ChangeOrder = Command<String>;
type CancelOrder = Command<String>;

enum OrderTakingCommand {
    Place(PlaceOrder),
    Change(ChangeOrder),
    Cancel(CancelOrder),
}
