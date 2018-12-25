//! `qt-binding-core` build test
//!
//! This crate contains one test, that is used to test `qt-binding-core`'s capability to build
//! a Qt project. This crate's test should run on a CI containing a Qt install.
//!
//! This test only supports Qt 5. If Qt 4 is available, this test will not run.

#[cfg(qt5)]
pub mod qt5 {
    use std::os::raw::c_void;

    pub struct Object {
        object: *mut c_void,
    }

    impl Object {
        pub fn new() -> Self {
            let object = unsafe { new_object() };

            Object { object }
        }

        pub fn value(&self) -> i32 {
            unsafe { object_value(self.object) }
        }

        pub fn set_value(&mut self, value: i32) {
            unsafe { set_object_value(self.object, value) }
        }
    }

    impl Default for Object {
        fn default() -> Self {
            Object::new()
        }
    }

    impl Drop for Object {
        fn drop(&mut self) {
            unsafe { delete_object(self.object) }
        }
    }

    pub struct TestApp;

    impl TestApp {
        pub fn run() {
            let result = unsafe { run_test() };
            assert_eq!(result, 0);
        }
    }

    extern "C" {
        fn new_object() -> *mut c_void;
        fn delete_object(object: *const c_void);
        fn object_value(object: *const c_void) -> i32;
        fn set_object_value(object: *const c_void, value: i32);
        fn run_test() -> i32;
    }
}
