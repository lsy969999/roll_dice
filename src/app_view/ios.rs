use std::{os::raw::c_void, ptr::NonNull};
use bevy::log::info;
use core_graphics::geometry::CGRect;
use objc::{runtime::Object, *};
use raw_window_handle::{
  DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, UiKitDisplayHandle, UiKitWindowHandle, WindowHandle
};

#[derive(Debug, Copy, Clone)]
pub struct IOSViewObj {
  pub view: *mut Object,
  pub scale_factor: f32,
}

impl Default for IOSViewObj {
    fn default() -> Self {
        Self {
          view: std::ptr::null_mut(),
          scale_factor: 1.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppView {
  pub view_obj: IOSViewObj,
}

impl std::ops::Deref for AppView {
    type Target = IOSViewObj;
    fn deref(&self) -> &Self::Target {
        &self.view_obj
    }
}

impl AppView {
  pub fn new(view_obj: IOSViewObj) -> Self {
    Self { view_obj }
  }

  pub fn logical_resolution(&self) -> (f32, f32) {

    let s: CGRect = unsafe {
        msg_send![self.view, frame]
    };
    info!("logical_resolution width: {}, height: {}", s.size.width, s.size.height);
    (s.size.width as f32, s.size.height as f32)
  }
}

// unsafe impl HasRawWindowHandle for AppView {
//     fn raw_window_handle(&self) -> Result<RawWindowHandle, HandleError> {
        
//         let mut handle = UiKitWindowHandle::new(NonNull::new(self.view as *mut c_void).unwrap());
//         // handle.ui_view = self.view as _;
//         Ok(RawWindowHandle::UiKit(handle))
//     }
// }

// unsafe impl HasRawDisplayHandle for AppView {
//     fn raw_display_handle(&self) -> Result<RawDisplayHandle, HandleError> {
//         Ok(RawDisplayHandle::UiKit(UiKitDisplayHandle::new()))
//     }
// }

impl HasWindowHandle for AppView {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = UiKitWindowHandle::new(NonNull::new(self.view as *mut c_void).unwrap());
        
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::UiKit(handle)) })
    }
}

impl HasDisplayHandle for AppView {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::UiKit(UiKitDisplayHandle::new())) })
    }
}