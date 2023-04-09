#[derive(Debug, Clone)]
struct ContactId(i64);

#[derive(Debug, Clone)]
struct Contact {
    contact_id: ContactId,
    phone_number: String,
    email_address: String,
}

impl PartialEq for Contact {
    fn eq(&self, other: &Self) -> bool {
        self.contact_id.0 == other.contact_id.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equal() {
        let contact_a = Contact {
            contact_id: ContactId(1),
            phone_number: "123-4567".to_string(),
            email_address: "aaaaa@example.com".to_string(),
        };
        let contact_b = Contact {
            phone_number: "890-1234".to_string(),
            email_address: "bbbb@example.com".to_string(),
            ..contact_a.clone()
        };
        assert_eq!(contact_a, contact_b)
    }

    #[test]
    fn test_not_equal() {
        let contact_a = Contact {
            contact_id: ContactId(1),
            phone_number: "123-4567".to_string(),
            email_address: "aaaaa@example.com".to_string(),
        };
        let contact_b = Contact {
            contact_id: ContactId(2),
            ..contact_a.clone()
        };
        assert_ne!(contact_a, contact_b)
    }
}
