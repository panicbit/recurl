
pub trait BorrowRaw<T> {
    unsafe fn borrow_raw<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R
    {
        self.borrow_raw_opt(|value| value.map(f))
    }

    unsafe fn borrow_raw_opt<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&T>) -> R;
}

impl<T> BorrowRaw<T> for *const T {
    unsafe fn borrow_raw_opt<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&T>) -> R
    {
        if self.is_null() {
            f(None)
        } else {
            f(Some(&**self))
        }
    }
}

impl<T> BorrowRaw<T> for *mut T {
    unsafe fn borrow_raw_opt<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&T>) -> R
    {
        if self.is_null() {
            f(None)
        } else {
            f(Some(&**self))
        }
    }
}

pub trait BorrowRawMut<T> {
    unsafe fn borrow_raw_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R
    {
        self.borrow_raw_mut_opt(|value| value.map(f))
    }

    unsafe fn borrow_raw_mut_opt<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&mut T>) -> R;
}

impl<T> BorrowRawMut<T> for *mut T {
    unsafe fn borrow_raw_mut_opt<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&mut T>) -> R
    {
        if self.is_null() {
            f(None)
        } else {
            f(Some(&mut **self))
        }
    }
}
