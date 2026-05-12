#![allow(dead_code)] // leveraged by consumers
use std::isize;

pub fn address<T>(t: &T) -> usize {
    (t as *const T) as usize
}

pub fn vec_of_cap_or<F, T>(cap: usize, f: F, #[cfg(test)] grade: &mut usize) -> Vec<T>
where
    F: FnOnce() -> Vec<T>,
{
    let max_cap = isize::MAX as usize;
    if cap <= max_cap {
        #[cfg(test)]
        set_grade(grade, 1);

        let size = size_of::<T>();
        if let Some(b) = usize::checked_mul(size, cap) {
            #[cfg(test)]
            set_grade(grade, 2);

            if b <= max_cap {
                #[cfg(test)]
                set_grade(grade, 3);

                return Vec::<T>::with_capacity(cap);
            }
        }
    }

    return f();

    #[cfg(test)]
    const fn set_grade(grade: &mut usize, g: usize) {
        *grade = g;
    }
}

#[cfg(test)]
#[cfg(feature = "rh-aide-tests")]
mod tests_of_unit {
    use super::*;

    #[test]
    fn address_test() {
        let addr = 333;
        let ptr = addr as *const Vec<usize>;
        let _ref = unsafe { ptr.as_ref() }.unwrap();

        let test = address(_ref);
        assert_eq!(addr, test);
    }

    mod vec_of_cap_or {
        use super::super::vec_of_cap_or;

        #[test]
        fn within_max_capicity() {
            let max_cap = isize::MAX as usize;
            let mut grade = 0;
            let vec: Vec<u8> = vec_of_cap_or(max_cap, || Vec::with_capacity(0), &mut grade);
            assert_eq!(true, vec.capacity() == max_cap);
            assert_eq!(3, grade);
        }

        #[test]
        fn over_max_capicity_a() {
            let over_max_cap = (isize::MAX as usize) + 1;
            let mut grade = 0;
            let vec: Vec<u8> = vec_of_cap_or(over_max_cap, || Vec::with_capacity(0), &mut grade);
            assert_eq!(true, vec.capacity() == 0);
            assert_eq!(0, grade);
        }

        #[test]
        fn over_max_capicity_b() {
            let cap = isize::MAX as usize;
            let mut grade = 0;
            let vec: Vec<u64> = vec_of_cap_or(cap, || Vec::with_capacity(0), &mut grade);
            assert_eq!(true, vec.capacity() == 0);

            assert_eq!(1, grade);
        }

        #[test]
        fn over_max_capicity_c() {
            let cap = isize::MAX as usize;
            let mut grade = 0;
            let vec: Vec<u16> = vec_of_cap_or(cap, || Vec::with_capacity(0), &mut grade);
            assert_eq!(true, vec.capacity() == 0);

            assert_eq!(2, grade);
        }
    }
}

// cargo fmt & cargo test --release --features rh-aide-tests
