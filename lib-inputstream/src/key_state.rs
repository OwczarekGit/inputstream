#[derive(Debug, Clone)]
pub struct KeyState {
    state: bool,
    bit: u32,
}

impl KeyState {
    pub fn new(bit: u32) -> Self {
        Self {
            state: false,
            bit: 1 << bit,
        }
    }

    pub fn bit(&self) -> u32 {
        self.bit
    }

    pub fn pressed(&self) -> bool {
        self.state
    }

    pub fn get_state(&mut self, mask: u32) -> Option<bool> {
        let current_state = (mask & (0xffff & self.bit)) > 0;

        if current_state != self.state {
            self.state = current_state;
            Some(current_state)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_fire_only_when_changed() {
        let mut state = 0b0000;
        let mut key1 = KeyState::new(0);
        assert_eq!(key1.get_state(state), None);
        state = 0b0001;
        assert_eq!(key1.get_state(state), Some(true));
        assert_eq!(key1.get_state(state), None);
        state = 0b0011;
        assert_eq!(key1.get_state(state), None);
        state = 0b1010;
        assert_eq!(key1.get_state(state), Some(false));
    }
}
