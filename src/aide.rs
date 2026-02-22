#[allow(dead_code)]

pub fn address<T>(t: &T) -> usize {
    (t as *const T) as usize
}

#[cfg(test)]
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
}

// cargo fmt & cargo test --release aide::
