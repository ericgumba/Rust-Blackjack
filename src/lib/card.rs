pub trait ValueToString {
    fn value_to_string(&self) -> String;
}

// create a structure called card with suite and value
pub struct Card {
    pub suite: String,
    pub value: u32,
}

// implement clone for card
impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            suite: self.suite.clone(),
            value: self.value,
        }
    }
}

// implement trait ValueToString for card
impl ValueToString for Card {
    fn value_to_string(&self) -> String {
        match self.value {
            1 => "Ace".to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => self.value.to_string(),
        }
    }
}