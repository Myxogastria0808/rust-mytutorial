#[derive(Debug)]

pub struct Guess {
    value: u32,
}

impl Guess {
    //*setter */
    pub fn set_value(value: u32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        };
        Guess { value }
    }
    //*getter */
    pub fn get_value(&self) -> u32 {
        self.value
    }
}
