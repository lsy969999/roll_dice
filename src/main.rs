#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    use roll_dice::create_roll_dice_app;

    let mut bevy_app = create_roll_dice_app();

    bevy_app.run();
}