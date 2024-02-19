use std::{cell::RefCell, rc::Rc};

struct SharedCommunication {
    message: Rc<RefCell<Option<String>>>,
}

impl SharedCommunication {
    fn new() -> Self {
        Self {
            message: Rc::new(RefCell::new(None)),
        }
    }

    fn new_form(other: &Self) -> Self {
        Self {
            message: other.message.clone(),
        }
    }

    fn send(&mut self, message: String) -> Result<(), ()> {
        if self.message.borrow().is_none() {
            *self.message.borrow_mut() = Some(message);
            Ok(())
        } else {
            Err(())
        }
    }

    fn receive(&mut self) -> Option<String> {
        self.message.borrow_mut().take()
    }
}

#[cfg(test)]
mod shared_communication_tests {
    use super::*;

    #[test]
    fn test_send_receive() {
        let mut first = SharedCommunication::new();
        let mut second = SharedCommunication::new_form(&first);

        assert_eq!(first.send("Hello".to_string()), Ok(()));
        assert_eq!(first.send("World".to_string()), Err(()));
        println!("{:?}", first.message.borrow());
        println!("{:?}", second.message.borrow());
        assert_eq!(second.receive(), Some("Hello".to_string()));
        assert_eq!(second.receive(), None);
        assert_eq!(first.receive(), None);
        assert_eq!(first.send("World".to_string()), Ok(()));
    }
}
