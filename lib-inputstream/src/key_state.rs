#[derive(Debug, Clone)]
pub struct KeyState {
    just_changed: bool,
    state: bool,
    bit: u32,
}

impl KeyState {
    pub fn new(bit: u32) -> Self {
        Self {
            just_changed: false,
            state: false,
            bit,
        }
    }

    pub fn bit(&self) -> u32 {
        self.bit
    }

    pub fn pressed(&self) -> bool {
        self.state
    }

    pub fn should_fire(&self) -> bool {
        self.just_changed
    }

    pub fn get_state(&mut self, mask: u32) -> Option<bool> {
        self.just_changed = false;
        let current_state = (mask & (0xffffffff & (1 << self.bit))) > 0;

        if current_state != self.state {
            self.just_changed = true;
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
