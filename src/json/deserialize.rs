use hashbrown::HashMap;

use crate::values::key::Key;

struct Deserializer {
    data: String,
    index: usize,
}

impl Deserializer {
    pub(crate) fn new(data: String) -> Deserializer {
        Deserializer { data, index: 0 }
    }

    fn next(&mut self) -> char {
        let c = self
            .data
            .chars()
            .nth(self.index)
            .expect("Unable to get next character");
        self.index += 1;
        c
    }

    fn peek(&self, offset: usize) -> char {
        self.data
            .chars()
            .nth(self.index + offset)
            .expect("Unable to peek")
    }

    fn skip_whitespace(&mut self) {
        while self.peek(0).is_whitespace() {
            self.next();
        }
    }

    fn parse_string(&mut self) -> String {
        let mut string = String::new();
        self.next();
        while self.peek(0) != '"' {
            string.push(self.next());
        }
        self.next();
        string
    }

    fn parse_number(&mut self) -> f64 {
        let mut number = String::new();
        while self.peek(0).is_numeric() || self.peek(0) == '.' {
            number.push(self.next());
        }
        number.parse::<f64>().expect("Unable to parse number")
    }

    fn parse_bool(&mut self) -> bool {
        let mut boolean = String::new();
        while self.peek(0).is_alphabetic() {
            boolean.push(self.next());
        }
        match boolean.as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("Unable to parse boolean"),
        }
    }

    fn parse_null(&mut self) -> () {
        let mut null = String::new();
        while self.peek(0).is_alphabetic() {
            null.push(self.next());
        }
        match null.as_str() {
            "null" => (),
            _ => panic!("Unable to parse null"),
        }
    }

    fn parse_array(&mut self) -> Vec<Key> {
        let mut array = Vec::new();
        self.next();
        self.skip_whitespace();
        while self.peek(0) != ']' {
            let value = self.parse_value();
            self.skip_whitespace();
            array.push(value);
            if self.peek(0) == ',' {
                self.next();
            }
            self.skip_whitespace();
        }
        self.next();
        array
    }

    fn parse_object(&mut self) -> HashMap<String, Key> {
        let mut object = HashMap::new();
        self.next();
        self.skip_whitespace();
        while self.peek(0) != '}' {
            let key = self.parse_string();
            self.skip_whitespace();
            self.next();
            self.skip_whitespace();
            let value = self.parse_value();
            self.skip_whitespace();
            object.insert(key, value);
            if self.peek(0) == ',' {
                self.next();
            }
            self.skip_whitespace();
        }
        self.next();
        object
    }

    fn parse_value(&mut self) -> Key {
        self.skip_whitespace();
        let value = match self.peek(0) {
            '"' => Key::String(self.parse_string()),
            '[' => Key::Array(self.parse_array()),
            '{' => Key::Object(self.parse_object()),
            '0'..='9' => Key::Number(self.parse_number()),
            't' | 'f' => Key::Boolean(self.parse_bool()),
            'n' => Key::Null(self.parse_null()),
            _ => panic!("Invalid value"),
        };
        self.skip_whitespace();
        value
    }

    pub(crate) fn deserialize(&mut self) -> HashMap<String, Key> {
        self.parse_object()
    }
}
