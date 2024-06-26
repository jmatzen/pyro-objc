use objc::class;
use objc::msg_send;
use objc::rc::autoreleasepool;
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
            Some(area_insets)
        })
    } else {
        None
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        println!("{:#?}", safe_area());
    }
}
/*
       let ui_application = Class::get("UIApplication").unwrap();
       let shared_application: *mut Object = unsafe { msg_send![ui_application, sharedApplication] };
       let windows: *mut Object = unsafe { msg_send![shared_application, windows] };
       let first_window: *mut Object = unsafe { msg_send![windows, firstObject] };

       let safe_area_insets: *mut Object = unsafe { msg_send![first_window, safeAreaInsets] };

       let top: f64 = unsafe { msg_send![safe_area_insets, top] };
       let left: f64 = unsafe { msg_send![safe_area_insets, left] };
       let bottom: f64 = unsafe { msg_send![safe_area_insets, bottom] };
       let right: f64 = unsafe { msg_send![safe_area_insets, right] };
*/
