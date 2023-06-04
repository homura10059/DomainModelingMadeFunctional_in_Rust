use crate::strings::String50;
use anyhow::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct UnvalidatedAddress {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, PartialEq)]
pub struct ZipCode(String);

impl ZipCode {
    fn new(zip_code: String) -> Self {
        Self(zip_code)
    }
}

pub struct CheckedAddress {
    address_line1: String,
    address_line2: Option<String>,
    address_line3: Option<String>,
    address_line4: Option<String>,
    city: String,
    zip_code: String,
}

pub struct Address {
    address_line1: String50,
    address_line2: Option<String50>,
    address_line3: Option<String50>,
    address_line4: Option<String50>,
    city: String50,
    zip_code: ZipCode,
}

pub fn to_address<F>(check_address_exists: F, unvalidated_address: UnvalidatedAddress) -> Address
where
    F: Fn(&UnvalidatedAddress) -> CheckedAddress,
{
    let checked_address = check_address_exists(&unvalidated_address);
    let address_line1 = String50::new(checked_address.address_line1);
    let address_line2 = String50::try_new(checked_address.address_line2);
    let address_line3 = String50::try_new(checked_address.address_line3);
    let address_line4 = String50::try_new(checked_address.address_line4);
    let city = String50::new(checked_address.city);
    let zip_code = ZipCode::new(checked_address.zip_code);

    Address {
        address_line1,
        address_line2,
        address_line3,
        address_line4,
        city,
        zip_code,
    }
}

type AddressValidationError = Error;
