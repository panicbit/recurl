use std::rc::Rc;
use std::cell::RefCell;

pub struct Handle<T>(RefCell<T>);

impl<T> Handle<T> {
    pub fn new(value: T) -> Rc<Self> {
        Rc::new(Handle(RefCell::new(value)))
    }

    pub unsafe fn borrow_raw<F, R>(this: *const Self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R
    {
        Self::borrow_handle_raw(this, |this| {
            f(&this.0.borrow())
        })
    }

    pub unsafe fn borrow_raw_mut<F, R>(this: *const Self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R
    {
        Self::borrow_handle_raw(this, |this| {
            f(&mut this.0.borrow_mut())
        })
    }

    pub unsafe fn borrow_handle_raw<F, R>(this: *const Self, f: F) -> Option<R>
    where
        F: FnOnce(&Rc<Self>) -> R
    {
        let this = Self::from_raw(this)?;
        let res = f(&this);
        this.into_raw(); // forget
        Some(res)
    }

    pub fn into_raw(self: Rc<Self>) -> *mut Self {
        Rc::into_raw(self) as *mut _
    }

    pub unsafe fn from_raw(this: *const Self) -> Option<Rc<Self>> {
        if this.is_null() {
            return None;
        }
        
        Some(Rc::from_raw(this))
    }

    pub unsafe fn clone_handle_from_raw(this: *const Self) -> Option<Rc<Self>> {
        Self::borrow_handle_raw(this, |this| this.clone())
    }
}
