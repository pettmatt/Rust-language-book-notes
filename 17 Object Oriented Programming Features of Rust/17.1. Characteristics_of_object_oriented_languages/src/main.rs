// Object is public, but fields are not.
pub struct AveragedCollection {
    // Remember, on default everything in rust is private.
    // The fields need to be manually assigned to be public (if we wanted).
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // Adding public methods to the struct/object
    // which are used to modifying the values.
    // With this kind of implementation the object contains
    // specific structure and won't be affected by external code.
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Hello, world!");
}