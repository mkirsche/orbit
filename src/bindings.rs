/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Aligner {
    _unused: [u8; 0],
}
extern "C" {
    pub fn align_read(
        arg1: *mut Aligner,
        arg2: *mut ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_char,
        arg4: ::std::os::raw::c_ulonglong,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn align_read_pair(
        arg1: *mut Aligner,
        arg2: *mut ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_char,
        arg4: *mut ::std::os::raw::c_char,
        arg5: *mut ::std::os::raw::c_char,
        arg6: ::std::os::raw::c_ulonglong,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn init_aligner_clone(arg1: *mut Aligner) -> *mut Aligner;
}
extern "C" {
    pub fn init_aligner(
        arg1: ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> *mut Aligner;
}
extern "C" {
    pub fn destroy_aligner(arg1: *mut Aligner);
}