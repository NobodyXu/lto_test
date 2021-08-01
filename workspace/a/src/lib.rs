use std::os::raw::c_int;

mod inner {
    use super::c_int;

    extern "C" {
        pub fn f() -> c_int;
    }
}

pub fn f() -> i32 {
    unsafe { inner::f() + 255 }
}
#[cfg(test)]
mod tests {
    use super::f;

    #[test]
    fn it_works() {
        assert_eq!(f(), 256);
    }
}
