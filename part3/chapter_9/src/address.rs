use anyhow::Error;

pub struct UnvalidatedAddress {
    street: String,
    city: String,
    state: String,
    zip: String,
}

type AddressValidationError = Error;

struct CheckedAddress {}

trait CheckAddressExists {
    fn check_address_exists(address: &UnvalidatedAddress) -> CheckedAddress;
}
