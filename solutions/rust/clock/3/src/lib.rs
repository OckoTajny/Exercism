use core::fmt;
use fmt::{Display, Formatter};

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
impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<>) -> fmt::Result{
        let mins = self.mins_from_midnight % 60;
        let mut hours = (self.mins_from_midnight - mins) / 60; 
        while true {
            if hours >= 24{
                hours = hours - 24
            }
            else {
                break;
            }            
        }
        write!(f, "{:02}:{:02}", hours, mins)
    }
}