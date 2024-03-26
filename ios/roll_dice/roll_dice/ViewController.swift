//
//  ViewController.swift
//  roll_dice
//
//  Created by SY L on 3/22/24.
//

import UIKit
import CoreMotion

class ViewController: UIViewController {
    @IBOutlet weak var metalV: UIView!
    var bevyApp: OpaquePointer?
    var preferredFPS: Int = 60 // 원하는 FPS 설정
    
    var gravity: CMAcceleration?
    var acc: CMAcceleration?
    var rotationRate: CMRotationRate?
    
    lazy var motionManager: CMMotionManager = {
        let manager = CMMotionManager.init()
        manager.gyroUpdateInterval = 0.032
        manager.accelerometerUpdateInterval = 0.032
        manager.deviceMotionUpdateInterval = 0.032
        return manager
    } ()
    
    lazy var displayLink: CADisplayLink = {
        CADisplayLink.init(target: self, selector: #selector(enterFrame))
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.displayLink.add(to: .current, forMode: .default)
//        self.displayLink.isPaused = true
    }
    
    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        self.view.backgroundColor = .white
        if bevyApp == nil {
            self.createBevyApp()
        }
        
//        self.startDeviceMotionUpdates()
        self.startAccelerometerUpdates()
//        self.displayLink.isPaused = true
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        displayLink.isPaused = true
        self.stopAccelerometerUpdates();
//        self.stopDeviceMotionUpdates()
    }

    func createBevyApp() {
        let viewPointer = Unmanaged.passUnretained(self.metalV).toOpaque()
        let maximumFrames = Int32(UIScreen.main.maximumFramesPerSecond)
        
        bevyApp = create_bevy_app(viewPointer, maximumFrames, Float(UIScreen.main.nativeScale))
        print("-- createBevyApp --")
        print("w: \(UIScreen.main.bounds.size.width), h: \(UIScreen.main.bounds.size.height), s: \(Float(UIScreen.main.nativeScale)) ")
    }
    
    @objc func enterFrame() {
        displayLink.preferredFramesPerSecond = preferredFPS
        guard let bevy = self.bevyApp else {
            return
        }
//        if let gravity = gravity {
//            print("gravity.x: \(Float(gravity.x)),  gravity.y: \(Float(gravity.y)) gravity.z:\(Float(gravity.z))")
//            device_motion(bevy, Float(gravity.x), Float(gravity.y), Float(gravity.z))
//        }
        if let acc = acc {
//            print("acc acc.x: \(Float(acc.x)),  acc.y: \(Float(acc.y)) acc.z:\(Float(acc.z))")
            if abs(acc.x) > 1.5
            || abs(acc.y) > 1.5
            || abs(acc.z) > 1.5 {
                device_accelerometer(bevy, Float(acc.x), Float(acc.y), Float(acc.z))
            }
        }
//        print("-- enterFrame --")
        enter_frame(bevy)
    }
    
    // MARK: touch
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let bevy = self.bevyApp, let touch: UITouch = touches.first {
            let location = touch.location(in: self.metalV);
            touch_started(bevy, Float(location.x), Float(location.y));
        }
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let bevy = self.bevyApp, let touch: UITouch = touches.first {
            let location = touch.location(in: self.metalV);
            touch_moved(bevy, Float(location.x), Float(location.y));
        }
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let bevy = self.bevyApp, let touch: UITouch = touches.first {
            let location = touch.location(in: self.metalV);
            touch_ended(bevy, Float(location.x), Float(location.y));
        }
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let bevy = self.bevyApp, let touch: UITouch = touches.first {
            let location = touch.location(in: self.metalV);
            touch_cancelled(bevy, Float(location.x), Float(location.y));
        }
    }
    
    deinit {
        if let bevy = bevyApp {
            release_bevy_app(bevy)
        }
    }
}

