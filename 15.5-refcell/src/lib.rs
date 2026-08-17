pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        match percentage_of_max {
            x if x >= 1.0 => self.messenger.send("Error: You are over your quota!"),
            x if x >= 0.9 => self
                .messenger
                .send("Urgent warning: You've used up over 90% of your quota!"),
            x if x >= 0.75 => self
                .messenger
                .send("Warning: You've used up over 75% of your quota!"),
            _ => {}
        }

        if percentage_of_max >= 1.0 {
            // self.messenger.sen
        }
    }
}
