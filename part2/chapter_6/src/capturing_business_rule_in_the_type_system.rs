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

fn address_validation_service(address: &UnvalidatedAddress) -> Option<ValidatedAddress> {
    let address_str = address.clone().0;
    if address_str.len() < 1 {
        return None;
    }
    Some(ValidatedAddress(address_str))
}

struct UnvalidatedOrder {
    sipping_address: UnvalidatedAddress,
}

struct ValidatedOrder {
    sipping_address: ValidatedAddress,
}
