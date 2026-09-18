pub struct Clock{
    mins_from_midnight: i32
}

impl Clock {
    pub fn new( hours: i32, minutes: i32) -> Self {
        Clock{
            mins_from_midnight: minutes + (hours * 60)
        }

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { mins_from_midnight: self.mins_from_midnight + minutes }
    }
}
