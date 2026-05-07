use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct RcCell<T>(Rc<RefCell<T>>);

impl<T> RcCell<T> {
    pub fn new(value: T) -> Self {
        RcCell(Rc::new(RefCell::new(value)))
    }

    // Get an immutable reference wrapper
    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    // Get a mutable reference wrapper
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

// Cloning only copies the pointer and increments the Rc reference count
impl<T> Clone for RcCell<T> {
    #[inline]
    fn clone(&self) -> Self {
        RcCell(Rc::clone(&self.0))
    }
}
