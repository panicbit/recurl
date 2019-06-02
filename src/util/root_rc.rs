use std::rc;
use std::ops::Deref;

#[derive(Default)]
pub struct RootRc<T> {
    inner: rc::Rc<T>,
}

impl<T> RootRc<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: rc::Rc::new(value),
        }
    }

    pub fn weak(&self) -> Weak<T> {
        Weak::new(self)
    }
}

impl<T> Deref for RootRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

pub struct Weak<T> {
    inner: rc::Weak<T>,
}

impl<T> Weak<T> {
    pub fn new(root_rc: &RootRc<T>) -> Self {
        Self {
            inner: rc::Rc::downgrade(&root_rc.inner),
        }
    }
    pub fn with_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.inner.upgrade().map(|value| f(&value))
    }
}