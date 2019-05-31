
pub trait BorrowRaw<T> {
    unsafe fn borrow_raw<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R;
}

impl<T> BorrowRaw<T> for *const T {
    unsafe fn borrow_raw<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R
    {
        if self.is_null() {
            return None;
        }

        Some(f(&**self))
    }
}

impl<T> BorrowRaw<T> for *mut T {
    unsafe fn borrow_raw<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R
    {
        if self.is_null() {
            return None;
        }

        Some(f(&**self))
    }
}

pub trait BorrowRawMut<T> {
    unsafe fn borrow_raw_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R;
}

impl<T> BorrowRawMut<T> for *mut T {
    unsafe fn borrow_raw_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R
    {
        if self.is_null() {
            return None;
        }

        Some(f(&mut **self))
    }
}
