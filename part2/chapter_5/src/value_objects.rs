#[derive(Debug, PartialEq)]
struct WidgetCode(String);

#[derive(Debug, PartialEq)]
struct PersonalName {
    first_name: String,
    last_name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_widget_code_1() {
        let widget_code1 = WidgetCode("W1234".to_string());
        let widget_code2 = WidgetCode("W1234".to_string());
        assert_eq!(widget_code1, widget_code2)
    }

    #[test]
    fn test_widget_code_2() {
        let widget_code1 = WidgetCode("W1234".to_string());
        let widget_code2 = WidgetCode("W12345".to_string());
        assert_ne!(widget_code1, widget_code2)
    }

    #[test]
    fn test_name_1() {
        let name1 = PersonalName {
            first_name: "aaa".to_string(),
            last_name: "bbb".to_string(),
        };
        let name2 = PersonalName {
            first_name: "aaa".to_string(),
            last_name: "bbb".to_string(),
        };
        assert_eq!(name1, name2)
    }

    #[test]
    fn test_name_2() {
        let name1 = PersonalName {
            first_name: "aaa".to_string(),
            last_name: "bbb".to_string(),
        };
        let name2 = PersonalName {
            first_name: "ccc".to_string(),
            last_name: "ddd".to_string(),
        };
        assert_ne!(name1, name2)
    }
}
