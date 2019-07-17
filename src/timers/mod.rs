// More infos here: https://en.wikipedia.org/wiki/CHIP-8#Timers

pub struct Timers {
    // A timer used for timing the events of games
    pub delay_timer: u8,

    // A timer used for sound effects
    pub sound_timer: u8,
}

impl Default for Timers {
    fn default() -> Self {
        Self {
            delay_timer: 0,
            sound_timer: 0
        }
    }
}
