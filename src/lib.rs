#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn ruby_hello_world() {
        unsafe {
            let mrb_state = mrb_open();
            if mrb_state.is_null() {
                panic!("mrb_open failed");
            }
            let code = CString::new("puts 'hello world'").unwrap();
            mrb_load_string(mrb_state, code.as_ptr());
        }
    }
}
