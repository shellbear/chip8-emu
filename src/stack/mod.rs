// More infos here: https://en.wikipedia.org/wiki/CHIP-8#The_stack

pub const STACK_SIZE: usize = 16;

pub struct Stack {
    // 16 stack addresses
    pub ret_addresses: [u16; STACK_SIZE],

    // The stack pointer
    pub sp: u8
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            ret_addresses: [0; STACK_SIZE],
            sp: 0
        }
    }
}
