mod phone;

#[cfg(test)]
mod tests {
    use crate::phone::*;

    #[test]
    fn phone_ru_new() {
        let phone = PhoneRu::new("+7 (912) 345 67-89").unwrap().get();

        assert_eq!(phone, "79123456789");
    }
}
