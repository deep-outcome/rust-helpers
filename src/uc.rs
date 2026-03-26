use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

#[cfg(test)]
impl<T> PartialEq for UC<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self, other)
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct UC<T>(pub UnsafeCell<T>);

impl<T> UC<T> {
    pub const fn aq_ref(&self) -> &T {
        let t = self.0.get();
        unsafe { t.as_mut().unwrap_unchecked() }
    }

    pub const fn promote(&self) -> &mut T {
        let t = self.0.get();
        unsafe { t.as_mut().unwrap_unchecked() }
    }

    pub const fn aq_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }

    pub const fn new(t: T) -> Self {
        Self(UnsafeCell::new(t))
    }
}

impl<T> Deref for UC<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.aq_ref()
    }
}

impl<T> DerefMut for UC<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.aq_mut()
    }
}

#[cfg(test)]
mod tests_of_units {
    use crate::aide::address;
    use std::ops::{Deref, DerefMut};

    use super::UC;

    mod partial_eq {
        use super::super::UC;

        #[test]
        fn eq() {
            let uc = UC::new(vec![0; 1]);
            assert_eq!(true, uc.eq(&uc));
        }

        #[test]
        fn not_eq() {
            let uc1 = UC::new(vec![0; 1]);
            let uc2 = UC::new(vec![0; 1]);
            assert_eq!(false, uc1.eq(&uc2));
        }
    }

    #[test]
    fn aq_ref() {
        let zero = &0usize as *const usize;
        let uc = UC::new(zero);
        let test = uc.aq_ref();

        assert_eq!(zero as usize, *test as usize);
    }

    #[test]
    fn aq_mut() {
        let zero = &0usize as *const usize;
        let mut uc = UC::new(zero);
        let test = uc.aq_mut();

        assert_eq!(zero as usize, *test as usize);
    }

    #[test]
    fn promote() {
        let zero = &0usize as *const usize;
        let uc = UC::new(zero);
        let test = uc.promote();

        assert_eq!(zero as usize, *test as usize);
    }

    #[test]
    fn new() {
        let uc = UC::new(333);
        let mut test = uc.0;
        assert_eq!(333, *test.get_mut());
    }

    #[test]
    fn deref() {
        let uc = UC::new(11);
        assert_eq!(uc.aq_ref(), uc.deref());
    }

    #[test]
    fn deref_mut() {
        let mut uc = UC::new(11);
        let aq_add = address(uc.aq_mut());
        let deref_add = address(uc.deref_mut());

        assert_eq!(aq_add, deref_add);
    }
}

// cargo test --release
