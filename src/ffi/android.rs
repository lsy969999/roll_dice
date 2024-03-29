use crate::android_asset_io::AndroidAssetManager;
use crate::app_view::android::{ AndroidViewObj, AppView };
use android_logger::Config;
use bevy::input::ButtonState;
use bevy::prelude::*;
use jni::sys::{jfloat, jlong, jobject};
use jni::JNIEnv;
use jni_fn::jni_fn;
use log::LevelFilter;
use bevy::input::touch::{TouchInput, TouchPhase};

#[link(name="c++_shared")]
extern "C" {}

#[no_mangle]
pub fn android_main(_android_app: JNIEnv) { // bevy::winit::ANDROID_APP

}
//xyz.lsy969999.roll_dice.RustBridge
#[no_mangle]
#[jni_fn("xyz.lsy969999.roll_dice.RustBridge")]
pub fn init_ndk_context(envs: JNIEnv, _: jobject, context: jobject) {
    log_panics::init();
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Info));
    let java_vm = envs.get_java_vm().unwrap();
    unsafe {
        ndk_context::initialize_android_context(java_vm.get_java_vm_pointer() as _, context as _);
    }
}

#[no_mangle]
#[jni_fn("xyz.lsy969999.roll_dice.RustBridge")]
pub fn create_bevy_app(
    envs: *mut JNIEnv,
    _: jobject,
    asset_manager: jobject,
    surface: jobject,
    scale_factor: jfloat
) -> jlong {
    let a_asset_manager = unsafe { ndk_sys::AAssetManager_fromJava(envs as _, asset_manager) };
    let android_obj = AndroidViewObj {
        native_window: AppView::get_native_window(envs, surface),
        scale_factor: scale_factor as _,
    };
    
    let mut bevy_app = crate::create_roll_dice_app(AndroidAssetManager(a_asset_manager));
    bevy_app.insert_non_send_resource(android_obj);
    crate::app_view::create_bevy_window(&mut bevy_app);
    log::info!("Bevy App created");

    Box::into_raw(Box::new(bevy_app)) as jlong
}

#[no_mangle]
#[jni_fn("xyz.lsy969999.roll_dice.RustBridge")]
pub fn enter_frame(_envs: *mut JNIEnv, _: jobject, obj: jlong) {
    let bevy_app = unsafe {
        &mut *(obj as *mut App)
    };
    bevy_app.update();
}

//device motion todo

#[no_mangle]
#[jni_fn("xyz.lsy969999.roll_dice.RustBridge")]
pub fn release_bevy_app(_envs: *mut JNIEnv, _: jobject, obj: jlong) {
    let bevy_app = unsafe {
        &mut *(obj as *mut App)
    };

    let app: Box<App> = unsafe {
        Box::from_raw(bevy_app as *mut _)
    };
    crate::close_bevy_window(app);
}


#[no_mangle]
#[jni_fn("xyz.lsy969999.roll_dice.RustBridge")]
pub fn touch_started(_envs: *mut JNIEnv, _: jobject, obj: jlong, x: f32, y: f32) {
    log::info!("touch_started x: {}, y: {}", x, y );
    touched(obj, TouchPhase::Started, Vec2::new(x, y));
}

#[no_mangle]
#[jni_fn("xyz.lsy969999.roll_dice.RustBridge")]
pub fn touch_moved(_envs: *mut JNIEnv, _: jobject, obj: jlong, x: f32, y: f32) {
    log::info!("touch_moved x: {}, y: {}", x, y );
    touched(obj, TouchPhase::Moved, Vec2::new(x, y));
}

#[no_mangle]
#[jni_fn("xyz.lsy969999.roll_dice.RustBridge")]
pub fn touch_ended(_envs: *mut JNIEnv, _: jobject, obj: jlong, x: f32, y: f32) {
    log::info!("touch_ended x: {}, y: {}", x, y );
    touched(obj, TouchPhase::Ended, Vec2::new(x, y));
}

#[no_mangle]
#[jni_fn("xyz.lsy969999.roll_dice.RustBridge")]
pub fn touch_cancelled(_envs: *mut JNIEnv, _: jobject, obj: jlong, x: f32, y: f32) {
    log::info!("touch_cancelled x: {}, y: {}", x, y );
    touched(obj, TouchPhase::Canceled, Vec2::new(x, y));
}

fn touched(obj: jlong, phase: TouchPhase, position: Vec2) {
    // let touch = TouchInput {
    //     phase,
    //     position,
    //     force: None,
    //     id: 0,
    //     window: todo!(),
    // };
    let app = unsafe {
        &mut *(obj as *mut App)
    };

    let world = &mut app.world;
   
    let mut q_win = world.query::<(Entity, &Window)>();
    let (e, _w): (Entity, &Window) = q_win.single_mut(world);
    let touch = TouchInput {
        phase,
        position,
        force: None,
        id: 0,
        window: e,
    };
    app.world.cell().send_event(touch);
    
}