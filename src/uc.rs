use std::{cell::UnsafeCell, ops::Deref};

#[cfg(test)]
impl<T> PartialEq for UC<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_ref().eq(other.get_ref())
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct UC<T>(UnsafeCell<T>);

impl<T> UC<T> {
    pub const fn get_ref(&self) -> &T {
        let t = self.0.get();
        unsafe { t.as_mut().unwrap_unchecked() }
    }

    pub const fn get_mut(&self) -> &mut T {
        let t = self.0.get();
        unsafe { t.as_mut().unwrap_unchecked() }
    }

    pub const fn new(t: T) -> Self {
        Self(UnsafeCell::new(t))
    }
}

impl<T> Deref for UC<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.get_ref()
    }
}

#[cfg(test)]
mod tests_of_units {
    use std::ops::Deref;

    use super::UC;

    mod partial_eq {
        use super::super::UC;

        #[test]
        fn eq() {
            let uc1 = UC::new(vec![0; 1]);
            let uc2 = UC::new(vec![0; 1]);
            assert_eq!(true, uc1.eq(&uc2));
        }

        #[test]
        fn not_eq() {
            let uc1 = UC::new(vec![0; 1]);
            let uc2 = UC::new(vec![0; 2]);
            assert_eq!(false, uc1.eq(&uc2));
        }
    }

    #[test]
    fn get_ref() {
        let zero = &0usize as *const usize;
        let uc = UC::new(zero);
        let test = uc.get_ref();

        assert_eq!(zero as usize, *test as usize);
    }

    #[test]
    fn get_mut() {
        let zero = &0usize as *const usize;
        let uc = UC::new(zero);
        let test = uc.get_mut();

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
        assert_eq!(uc.get_ref(), uc.deref());
    }
}

// cargo test --release
