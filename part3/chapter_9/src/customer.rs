use crate::email_address::EmailAddress;
use crate::strings::String50;

pub struct UnvalidatedCustomer {
    first_name: String,
    last_name: String,
    email: String,
}

pub struct CustomerInfo {
    name: PersonalName,
    email: EmailAddress,
}

pub struct PersonalName {
    first_name: String50,
    last_name: String50,
}

impl From<UnvalidatedCustomer> for CustomerInfo {
    fn from(unvalidated_customer: UnvalidatedCustomer) -> Self {
        let name = PersonalName {
            first_name: String50::new(unvalidated_customer.first_name),
            last_name: String50::new(unvalidated_customer.last_name),
        };
        let email = EmailAddress(unvalidated_customer.email);
        CustomerInfo { name, email }
    }
}
