use std::ffi::CString;
use std::ptr;

use objc::class;
use objc::msg_send;
use objc::rc::autoreleasepool;
use objc::runtime::Class;
use objc::runtime::Object;
use objc::sel;
use objc::sel_impl;


#[derive(Debug)]
#[repr(C)]
pub struct UIEdgeInsets {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

type MutObjPtr = *mut Object;

pub fn safe_area() -> Option<UIEdgeInsets> {
    if cfg!(target_os = "ios") {
        autoreleasepool(|| unsafe {
            let class = class!(UIApplication);
            let shared: MutObjPtr = msg_send![class, sharedApplication];
            let windows: MutObjPtr = msg_send![shared, windows];
            let first_window: MutObjPtr = msg_send![windows, firstObject];
            let area_insets: UIEdgeInsets = msg_send![first_window, safeAreaInsets];
            let window_scene: MutObjPtr = msg_send![first_window, windowScene];
            let statu_bar_manager: MutObjPtr = msg_send![window_scene, statusBarManager];
            if statu_bar_manager != ptr::null_mut() {
               let status_bar_frame: MutObjPtr = msg_send![statu_bar_manager, statusBarFrame];
               let status_bar_height: f32 = msg_send![status_bar_frame, height];
               println!("******** status_bar_height = {}", status_bar_height);
            } 
            Some(area_insets)
        })
    } else {
        None
    }
}

pub fn store_install_id(id: &str) {
    autoreleasepool(|| {
        let ns_string :  *mut Object= {
            let cls = Class::get("NSString").unwrap();
            let cstr = CString::new(id).unwrap();
            unsafe { msg_send![cls, stringWithUTF8String: cstr.as_ptr()] }
        };
 
        let user_defaults:  *mut Object = {
            let cls = Class::get("NSUserDefaults").unwrap();
            unsafe { msg_send![cls, standardUserDefaults] }
        };
 
        let _: () = unsafe { msg_send![user_defaults, setObject: ns_string forKey: "install_id"] };
    });
 }
 
 pub fn retrieve_install_id() -> Option<String> {
    autoreleasepool(|| {
        let user_defaults:  *mut Object = {
            let cls = Class::get("NSUserDefaults").unwrap();
            unsafe { msg_send![cls, standardUserDefaults] }
        };
 
        let ns_string: *mut Object = unsafe { msg_send![user_defaults, stringForKey: "install_id"] };
 
        if ns_string.is_null() {
            None
        } else {
            let c_str: *const i8 = unsafe { msg_send![ns_string, UTF8String] };
            let rust_str = unsafe { std::ffi::CStr::from_ptr(c_str) }.to_str().unwrap().to_string();
            Some(rust_str)
        }
    })
 }
 
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        println!("{:#?}", safe_area());
    }

    #[test]
    fn test_retrieve_install_id() {
        super::retrieve_install_id();
    }
}
