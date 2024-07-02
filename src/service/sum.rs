pub struct Sum { 
    first: i64,
    second: i64
}

impl Sum {
    pub fn new(first: i64, second: i64) -> Sum {
        return  Sum{first: first, second: second};
    }

    pub fn add(&self) -> Result<i64, String> {
        if self.first > 0 && self.second > 0 {
            Ok(self.first + self.second)
        } else {
            Err("both the values must be positive".to_string())
        }
    }
}