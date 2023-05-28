use anyhow::Error;

pub struct UnvalidatedAddress {
    street: String,
    city: String,
    state: String,
    zip: String,
}

pub struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

impl From<UnvalidatedAddress> for Address {
    fn from(unvalidated_address: UnvalidatedAddress) -> Self {
        Address {
            street: unvalidated_address.street,
            city: unvalidated_address.city,
            state: unvalidated_address.state,
            zip: unvalidated_address.zip,
        }
    }
}

type AddressValidationError = Error;

struct CheckedAddress {}

trait CheckAddressExists {
    fn check_address_exists(address: &UnvalidatedAddress) -> CheckedAddress;
}
