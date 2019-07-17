// More infos here: https://en.wikipedia.org/wiki/CHIP-8#Graphics_and_sound

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Screen {
    // Because CHIP-8 display is monochrome, we simply use a boolean to declare if each pixel
    // of the screen is active or not
    pub mem: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],

    // We store a boolean to indicate if we should draw screen at the current frame
    pub should_draw: bool
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            mem: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            should_draw: false
        }
    }
}

impl Screen {
    pub fn scroll_down(&mut self, num_lines: u8) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.mem[y][x] = match y < (num_lines as usize) {
                    true => false,
                    _ => self.mem[y - (num_lines as usize)][x]
                };
            }
        }
    }

    pub fn scroll_up(&mut self, num_lines: u8) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.mem[y][x] = match y >= (num_lines as usize) {
                    true => false,
                    _ => self.mem[y + (num_lines as usize)][x]
                };
            }
        }
    }

    pub fn scroll_left(&mut self, num_lines: u8) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.mem[y][x] = match x >= (num_lines as usize) {
                    true => false,
                    _ => self.mem[y][x + (num_lines as usize)]
                };
            }
        }
    }

    pub fn scroll_right(&mut self, num_lines: u8) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.mem[y][x] = match x < (num_lines as usize) {
                    true => false,
                    _ => self.mem[y][x - (num_lines as usize)]
                };
            }
        }
    }
}
