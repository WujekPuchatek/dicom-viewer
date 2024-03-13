use std::fmt;

#[derive(Copy, Clone)]
pub struct ValueRepresentation {
    pub value: [u8; 2],
}

impl ValueRepresentation {
    fn format_value(&self) -> String {
        let chars: [char; 2] = [
            self.value[0] as char,
            self.value[1] as char,
        ];
        format!("{}{}", chars[0], chars[1])
    }
}

impl fmt::Display for ValueRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_value())
    }
}

impl fmt::Debug for ValueRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_value())
    }
}


