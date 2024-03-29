package xyz.lsy969999.roll_dice

import android.content.Context
import android.graphics.Canvas
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import android.util.AttributeSet
import android.util.Log
import android.view.MotionEvent
import android.view.SurfaceHolder
import android.view.SurfaceView

class BevySurfaceView: SurfaceView, SurfaceHolder.Callback2 {
    private var rustBridge: RustBridge = RustBridge()
    private var bevy_app: Long = Long.MAX_VALUE
    private var idx: Int = 0
    private var sensorManager: SensorManager? = null
    private var mSensor: Sensor? = null
    private var sensorValues: FloatArray = FloatArray(3)
    private var inits = false
    val sensorEventListener = object : SensorEventListener {
        override fun onSensorChanged(event: SensorEvent?) {
            if (event?.sensor?.type == Sensor.TYPE_LINEAR_ACCELERATION) {
                sensorValues = event.values
//                Log.d("TAG", "onSensorChanged: x: ${event.values[0]} y: ${event.values[1]} z: ${event.values[2]}")
            }
        }

        override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {
            if (sensor?.type == Sensor.TYPE_LINEAR_ACCELERATION) {
                Log.d("TAG", "accuracy: accuracy: ${accuracy}")
            }
        }
    }
    constructor(context: Context) :super(context) {
        sensorManager = context.getSystemService(Context.SENSOR_SERVICE) as SensorManager
        mSensor = sensorManager?.getDefaultSensor(Sensor.TYPE_LINEAR_ACCELERATION)
        Log.d("TAG", "constructor")
    }
    constructor(context: Context, attrs: AttributeSet): super(context, attrs) {

    }
    constructor(context: Context, attrs: AttributeSet, defStyle: Int): super(context, attrs, defStyle) {

    }

    init {
        holder.addCallback(this)
        rustBridge.init_ndk_context(this.context)
    }
    override fun surfaceCreated(holder: SurfaceHolder) {
        Log.d("TAG", "surfaceCreated bevy_app: ${bevy_app}")
        holder.let { h ->
            val scaleFactor: Float = resources.displayMetrics.density
            bevy_app = rustBridge.create_bevy_app(this.context.assets, h.surface, scaleFactor)

            Log.d("TAG", "bevy_app: ${bevy_app}")
            setWillNotDraw(false)

            mSensor?.also { sensor ->
                sensorManager?.registerListener(sensorEventListener, sensor, SensorManager.SENSOR_DELAY_GAME)
            }
        }
    }

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {

    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {

        sensorManager?.unregisterListener(sensorEventListener)
        Log.d("TAG", "surfaceDestroyed: ")
        if (bevy_app != Long.MAX_VALUE) {
            rustBridge.release_bevy_app(bevy_app)
            bevy_app = Long.MAX_VALUE
        }
    }

    override fun surfaceRedrawNeeded(holder: SurfaceHolder) {

    }

    private var time = 0;
    override fun draw(canvas: Canvas) {
//        Log.d("TAG,", "draw: ${time}")
//        time++
        super.draw(canvas)
//        if (time > 500 ) {
//            return
//        }
        if (bevy_app == Long.MAX_VALUE) {
            return
        }
//        rustBridge.device_motion(bevy_app, sensorValues[0], sensorValues[1], sensorValues[2])
        rustBridge.enter_frame(bevy_app)
        invalidate()
    }

    override fun onTouchEvent(event: MotionEvent?): Boolean {
        val scaleFactor: Float = resources.displayMetrics.density
        if (event?.action == MotionEvent.ACTION_DOWN){
            val x = event.x / scaleFactor
            val y = event.y / scaleFactor
            Log.d("BevySurfaceView", "onTouchEvent ACTION_DOWN: x: ${x} y: ${y} scaleFactor: ${scaleFactor}")
            rustBridge.touch_started(bevy_app, x, y)
        }
        else if (event?.action == MotionEvent.ACTION_MOVE){
            val x = event.x / scaleFactor
            val y = event.y / scaleFactor
            Log.d("BevySurfaceView", "onTouchEvent ACTION_MOVE: x: ${x} y: ${y} scaleFactor: ${scaleFactor}")
            rustBridge.touch_moved(bevy_app, x, y)
        }
        else if (event?.action == MotionEvent.ACTION_UP){
            val x = event.x / scaleFactor
            val y = event.y / scaleFactor
            Log.d("BevySurfaceView", "onTouchEvent ACTION_UP: x: ${x} y: ${y} scaleFactor: ${scaleFactor}")
            rustBridge.touch_ended(bevy_app, x, y)
        }
        else if (event?.action == MotionEvent.ACTION_CANCEL){
            val x = event.x / scaleFactor
            val y = event.y / scaleFactor
            Log.d("BevySurfaceView", "onTouchEvent ACTION_CANCEL: x: ${x} y: ${y} scaleFactor: ${scaleFactor}")
            rustBridge.touch_cancelled(bevy_app, x, y)
        }
        return true
    }
}
