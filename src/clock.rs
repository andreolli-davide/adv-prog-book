use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct MasterClock {
    count: Rc<RefCell<usize>>,
}

#[derive(Debug)]
struct SlaveClock {
    count: Rc<RefCell<usize>>,
}

impl MasterClock {
    fn new() -> Self {
        Self {
            count: Rc::new(RefCell::new(0)),
        }
    }

    fn tick(&self) -> () {
        *self.count.borrow_mut() += 1;
    }

    fn get_slave(&self) -> SlaveClock {
        SlaveClock {
            count: self.count.clone(),
        }
    }
}

impl SlaveClock {
    fn get_clock(&self) -> usize {
        *self.count.borrow()
    }
}

#[cfg(test)]
mod clock_tests {
    use super::*;

    #[test]
    fn clock_fulltest() {
        let master = MasterClock::new();
        let slave = master.get_slave();

        master.tick();
        master.tick();

        let slave2 = master.get_slave();

        master.tick();

        assert_eq!(slave.get_clock(), 3);
        assert_eq!(slave2.get_clock(), 3);
    }
}
