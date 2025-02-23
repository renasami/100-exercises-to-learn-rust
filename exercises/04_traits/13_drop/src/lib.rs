// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

pub trait New {
    fn new() -> Self;
}

pub trait Defuse {
    fn defuse(&mut self);
}
#[derive(Debug)]
pub struct DropBomb {
    defused: bool,
}

impl New for DropBomb {
    fn new() -> Self {
        Self { defused: false }
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!()
        }
    }
}

impl Defuse for DropBomb {
    fn defuse(&mut self) {
        self.defused = true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let _bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
