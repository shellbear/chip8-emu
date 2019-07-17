// More infos here: https://en.wikipedia.org/wiki/CHIP-8#Graphics_and_sound

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Screen {
    // Because CHIP-8 display is monochrome, we simply use a boolean to declare if each pixel
    // of the screen is active or not
    pub mem: [[bool; WIDTH]; HEIGHT],

    // We store a boolean to indicate if we should draw screen at the current frame
    pub should_draw: bool
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            mem: [[false; WIDTH]; HEIGHT],
            should_draw: false
        }
    }
}

impl Screen {
    pub fn scroll_down(&mut self, num_lines: u8) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.mem[y][x] = match y < (num_lines as usize) {
                    true => false,
                    _ => self.mem[y - (num_lines as usize)][x]
                };
            }
        }
    }

    pub fn scroll_up(&mut self, num_lines: u8) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.mem[y][x] = match y >= (num_lines as usize) {
                    true => false,
                    _ => self.mem[y + (num_lines as usize)][x]
                };
            }
        }
    }

    pub fn scroll_left(&mut self, num_lines: u8) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.mem[y][x] = match x >= (num_lines as usize) {
                    true => false,
                    _ => self.mem[y][x + (num_lines as usize)]
                };
            }
        }
    }

    pub fn scroll_right(&mut self, num_lines: u8) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.mem[y][x] = match x < (num_lines as usize) {
                    true => false,
                    _ => self.mem[y][x - (num_lines as usize)]
                };
            }
        }
    }
}
