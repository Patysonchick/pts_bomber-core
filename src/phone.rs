#[derive(Debug)]
pub struct PhoneRu {
    pub phone: String,
}

pub enum FormatterTypes {
    WithoutCode,
    WithPlus,
    WithPlusHyphen,          // +7 *** ***-**-**
    WithPlusBracketsHyphen,  // +7 (***) ***-**-**
    WithPlusBracketsHyphen2, // +7 (***) ***-****
}

#[derive(Debug)]
pub enum FormatterErrors {
    IncorrectLength,
    IncorrectPatter,
}

pub trait Phone {
    fn new(raw_phone: &str) -> Result<Self, FormatterErrors>
    where
        Self: std::marker::Sized;

    fn get(self) -> String;

    fn extract_numbers(raw_phone: &str) -> String {
        let mut phone = String::new();
        let numbers: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        for item in raw_phone.chars() {
            for number in numbers {
                if item == number {
                    phone.push(item);
                }
            }
        }

        phone
    }

    fn format(&mut self, formatter: FormatterTypes);
}

impl Phone for PhoneRu {
    fn new(raw_phone: &str) -> Result<Self, FormatterErrors> {
        let mut extracted = Self::extract_numbers(raw_phone);

        if extracted.len() != 11 {
            return Err(FormatterErrors::IncorrectLength);
        } else if &extracted[0..2] != "79" && &extracted[0..2] != "74" {
            if &extracted[0..2] == "89" || &extracted[0..2] == "84" {
                let mut new = String::from("7");
                new.push_str(&extracted[1..]);
                extracted = new;
            } else {
                return Err(FormatterErrors::IncorrectPatter);
            }
        }

        Ok(Self { phone: extracted })
    }

    fn get(self) -> String {
        self.phone
    }

    fn format(&mut self, formatter: FormatterTypes) {
        match formatter {
            FormatterTypes::WithoutCode => {
                self.phone = self.phone[1..].to_string();
            }
            FormatterTypes::WithPlus => {
                let mut formatted = String::from('+');
                formatted.push_str(self.phone.as_str());

                self.phone = formatted;
            }
            FormatterTypes::WithPlusHyphen => {
                let mut formatted = String::from('+');
                formatted.push(self.phone.chars().nth(0).unwrap());
                formatted.push(' ');
                formatted.push_str(&self.phone[1..4]);
                formatted.push(' ');
                formatted.push_str(&self.phone[4..7]);
                formatted.push('-');
                formatted.push_str(&self.phone[7..9]);
                formatted.push('-');
                formatted.push_str(&self.phone[9..11]);

                self.phone = formatted;
            }
            FormatterTypes::WithPlusBracketsHyphen => {
                let mut formatted = String::from('+');
                formatted.push(self.phone.chars().nth(0).unwrap());
                formatted.push(' ');
                formatted.push('(');
                formatted.push_str(&self.phone[1..4]);
                formatted.push(')');
                formatted.push(' ');
                formatted.push_str(&self.phone[4..7]);
                formatted.push('-');
                formatted.push_str(&self.phone[7..9]);
                formatted.push('-');
                formatted.push_str(&self.phone[9..11]);

                self.phone = formatted;
            }
            FormatterTypes::WithPlusBracketsHyphen2 => {
                let mut formatted = String::from('+');
                formatted.push(self.phone.chars().nth(0).unwrap());
                formatted.push(' ');
                formatted.push('(');
                formatted.push_str(&self.phone[1..4]);
                formatted.push(')');
                formatted.push(' ');
                formatted.push_str(&self.phone[4..7]);
                formatted.push('-');
                formatted.push_str(&self.phone[7..11]);

                self.phone = formatted;
            }
        }
    }
}
