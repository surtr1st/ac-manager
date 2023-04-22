#[derive(Debug)]
pub struct Query {
    target: String,
    index: i32,
}

impl Query {
    pub fn from(mut self, target: String) -> Query {
        self.target = target;
        self
    }
    pub fn select(mut self, index: i32) -> Query {
        self.index = index;
        self
    }
    pub fn get(self) -> Query {
        Query {
            target: self.target,
            index: self.index,
        }
    }
}
