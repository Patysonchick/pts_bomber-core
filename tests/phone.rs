mod ru {
    use pts_bomber_core::phone::*;

    #[test]
    fn new() {
        let phone = PhoneRu::new("+7 (912) 345 67-89").unwrap().get();

        assert_eq!(phone, "79123456789");
    }

    #[test]
    fn format_with_plus_brackets_hyphen() {
        let mut phone = PhoneRu::new("+7 (912) 345 67-89").unwrap();
        phone.format(FormatterTypes::WithPlusBracketsHyphen);

        assert_eq!(phone.get(), "+7 (912) 345-67-89");
    }
}
