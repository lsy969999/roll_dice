use bevy::prelude::*;
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::log::info;
use crate::app_view::ios::IOSViewObj;
use crate::app_view::create_bevy_window;
use crate::create_roll_dice_app;
use crate::game::dice::AccelerometerEvent;

#[no_mangle]
pub fn create_bevy_app(
  view: *mut objc::runtime::Object,
  scale_factor: f32
) -> *mut libc::c_void {
  let mut bevy_app = create_roll_dice_app();
  let ios_obj = IOSViewObj {
    view,
    scale_factor
  };
  bevy_app.insert_non_send_resource(ios_obj);

  create_bevy_window(&mut bevy_app);
  info!("Bevy App created!");
  let box_obj = Box::new(bevy_app);
  Box::into_raw(box_obj) as *mut libc::c_void
}

#[no_mangle]
pub fn enter_frame(obj: *mut libc::c_void) {
  let app: &mut App = unsafe {
      &mut *(obj as *mut App)
  };
  app.update()
}

#[no_mangle]
pub fn release_bevy_app(
  obj: *mut libc::c_void
) {
  let app: Box<App> = unsafe {
      Box::from_raw(obj as *mut _)
  };
  crate::close_bevy_window(app);
}



#[no_mangle]
pub fn touch_started(obj: *mut libc::c_void, x: f32, y: f32) {
    // info!("touch_started");
    touched(obj, TouchPhase::Started, Vec2::new(x, y));
}

#[no_mangle]
pub fn touch_moved(obj: *mut libc::c_void, x: f32, y: f32) {
    // info!("touch_moved");
    touched(obj, TouchPhase::Moved, Vec2::new(x, y));
}

#[no_mangle]
pub fn touch_ended(obj: *mut libc::c_void, x: f32, y: f32) {
    // info!("touch_ended");
    touched(obj, TouchPhase::Ended, Vec2::new(x, y));
}

#[no_mangle]
pub fn touch_cancelled(obj: *mut libc::c_void, x: f32, y: f32) {
    // info!("touch_cancelled");
    touched(obj, TouchPhase::Canceled, Vec2::new(x, y));
}

fn touched(obj: *mut libc::c_void, phase: TouchPhase, position: Vec2) {
    // let touch = TouchInput {
    //     phase,
    //     position,
    //     force: None,
    //     id: 0,
    //     window: todo!(),
    // };
    let app = unsafe { &mut *(obj as *mut App) };

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

#[no_mangle]
pub fn device_motion(_obj: *mut libc::c_void, _x: f32, _y: f32, _z: f32) {
    // info!("device motion! {}, {}, {}", x, _y, _z);
    // let app = unsafe { &mut *(obj as *mut App) };
    // if x > 0.005 {
    //     crate::change_input(app, KeyCode::Left, ButtonState::Released);
    //     crate::change_input(app, KeyCode::Right, ButtonState::Pressed);
    // } else if x < -0.005 {
    //     crate::change_input(app, KeyCode::Right, ButtonState::Released);
    //     crate::change_input(app, KeyCode::Left, ButtonState::Pressed);
    // } else {
    //     crate::change_input(app, KeyCode::Left, ButtonState::Released);
    //     crate::change_input(app, KeyCode::Right, ButtonState::Released);
    // }
}

#[no_mangle]
pub fn device_accelerometer(obj: *mut libc::c_void, x: f32, y: f32, z: f32) {
    // info!("device device_accelerometer! x: {}, y: {}, z: {}", x, y, z);
    let app = unsafe { &mut *(obj as *mut App) };
    let acc = AccelerometerEvent {
        x,
        y,
        z,
    };
    app.world.cell().send_event(acc);
}