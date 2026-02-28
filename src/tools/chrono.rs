struct Chrono {
    hour : u8,
    minute : u8,
    second : u8

}

impl Chrono {
    fn new(hour : u8, minute : u8, second : u8) -> Chrono {
        Chrono {
            hour,
            minute,
            second
        }
    }

    fn set_hour(&mut self, hour : u8) {
        self.hour = hour;
    }

    fn set_minute(&mut self, minute : u8) {
        self.minute = minute;
    }

    fn set_second(&mut self, second : u8) {
        self.second = second;
    }

    pub fn add_hour(&mut self, hour : u8) {
        let new_hour = self.hour.saturating_add(hour);
        self.hour = new_hour;
    }

    pub fn add_minute(&mut self, hour : u8) {
        let new_hour = self.hour.saturating_add(hour);
        self.hour = new_hour;
    }

    pub fn add_second(&mut self, hour : u8) {
        let new_hour = self.hour.saturating_add(hour);
        self.hour = new_hour;
    }

}