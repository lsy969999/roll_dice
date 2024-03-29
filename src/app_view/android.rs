use jni::sys::jobject;
use jni::JNIEnv;
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle,
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle
  };
use std::{os::raw::c_void, ptr::NonNull};

#[derive(Debug, Clone, Copy)]
pub struct AndroidViewObj {
    pub native_window: *mut ndk_sys::ANativeWindow,
    pub scale_factor: f32
}

impl Default for AndroidViewObj {
    fn default() -> Self {
        Self {
            native_window: std::ptr::null_mut(),
            scale_factor: 2.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppView {
    pub view_obj: AndroidViewObj
}

impl std::ops::Deref for AppView {
    type Target = AndroidViewObj;
    fn deref(&self) -> &Self::Target {
        &self.view_obj
    }
}

impl AppView {
    pub fn get_native_window(env: *mut JNIEnv, surface: jobject) -> *mut ndk_sys::ANativeWindow {
        unsafe {
            ndk_sys::ANativeWindow_fromSurface(env as _, surface as _)
        }
    }

    pub fn new(view_obj: AndroidViewObj) -> Self {
        Self {
            view_obj
        }
    }

    pub fn logical_resolution(&self) -> (f32, f32) {
        (
            self.get_width() as f32 / self.scale_factor,
            self.get_height() as f32 / self.scale_factor,
        )
    }

    fn get_width(&self) -> u32 {
        unsafe {
            ndk_sys::ANativeWindow_getWidth(self.native_window) as u32
        }
    }

    fn get_height(&self) -> u32 {
        unsafe {
            ndk_sys::ANativeWindow_getHeight(self.native_window) as u32
        }
    }
}

// unsafe impl HasRawWindowHandle for AppView {
//     fn raw_window_handle(&self) -> RawWindowHandle {
//         let mut handle = AndroidNdkWindowHandle::empty();
//         handle.a_native_window = self.native_window as _;
//         RawWindowHandle::AndroidNdk(handle)
//     }
// }

// unsafe impl HasRawDisplayHandle for AppView {
//     fn raw_display_handle(&self) -> RawDisplayHandle {
//         RawDisplayHandle::Android(AndroidDisplayHandle::empty())
//     }
// }

impl Drop for AppView {
    fn drop(&mut self) {
        unsafe {
            ndk_sys::ANativeWindow_release(self.native_window)
        }
    }
}


impl HasWindowHandle for AppView {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = AndroidNdkWindowHandle::new(NonNull::new(self.native_window as *mut c_void).unwrap());
        
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::AndroidNdk(handle)) })
    }
}

impl HasDisplayHandle for AppView {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::Android(AndroidDisplayHandle::new())) })
    }
}