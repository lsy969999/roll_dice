package xyz.lsy969999.roll_dice

import android.content.Context
import android.content.res.AssetManager
import android.view.Surface

class RustBridge {
    init {
        System.loadLibrary("roll_dice")
    }

    external fun init_ndk_context(ctx: Context)
    external fun create_bevy_app(asset_manager: AssetManager, surface: Surface, scale_factor: Float): Long
    external fun enter_frame(bevy_app: Long)
    external fun release_bevy_app(bevy_app: Long)

    external fun touch_started(bevy_app: Long, x: Float, y: Float)
    external fun touch_moved(bevy_app: Long, x: Float, y: Float)
    external fun touch_ended(bevy_app: Long, x: Float, y: Float)
    external fun touch_cancelled(bevy_app: Long, x: Float, y: Float)

    external fun device_accelerometer(bevy_app: Long, x: Float, y: Float, z: Float)
}