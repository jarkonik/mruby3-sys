use std::ffi::CString;

use mruby3_sys::{mrb_load_string, mrb_open};

fn main() {
    unsafe {
        let mrb_state = mrb_open();
        if mrb_state.is_null() {
            panic!("mrb_open failed");
        }
        let code = CString::new("puts 'hello world'").unwrap();
        mrb_load_string(mrb_state, code.as_ptr());
    }
}
