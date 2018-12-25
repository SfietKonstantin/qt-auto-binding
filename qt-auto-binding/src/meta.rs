use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
};

pub struct Object {
    ptr: *mut c_void,
}

impl Object {
    pub fn new(type_name: &str) -> Option<Object> {
        let type_name = format!("qt_auto_binding::{}*", type_name);
        let type_name = CString::new(type_name).ok()?;
        let ptr = unsafe { qt_auto_binding_meta_new_object(type_name.as_ptr()) };

        if !ptr.is_null() {
            Some(Object { ptr })
        } else {
            None
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { qt_auto_binding_meta_delete_object(self.ptr) }
    }
}

extern "C" {
    fn qt_auto_binding_meta_new_object(type_name: *const c_char) -> *mut c_void;
    fn qt_auto_binding_meta_delete_object(object: *const c_void);
}
