//
//  ViewController+CoreMotion.swift
//  roll_dice
//
//  Created by SY L on 3/24/24.
//

import CoreMotion

extension ViewController {
    func startGyroUpdates() {
        if !motionManager.isGyroAvailable {
            return
        }
        if motionManager.isGyroActive {
            return
        }
        motionManager.startGyroUpdates(to: OperationQueue.init()) { gyroData, error in
            guard let gyroData = gyroData else {
                print("startGyroUpdates error: \(error!)")
                return;
            }
            self.rotationRate = gyroData.rotationRate
        }
    }
    
    func stopGyroUpdates() {
        motionManager.stopGyroUpdates();
    }
    
    func startAccelerometerUpdates() {
        
        motionManager.startAccelerometerUpdates(to: OperationQueue.init()) { accData, error in
            guard let acc = accData?.acceleration else {
                print("startDeviceMotionUpdates error: \(error!)")
                return
            }
            self.acc = acc
        }
    }
    
    func stopAccelerometerUpdates() {
        motionManager.stopAccelerometerUpdates()
    }
    
    func stopDeviceMotionUpdates() {
        motionManager.stopDeviceMotionUpdates();
    }
    
    func startDeviceMotionUpdates() {
        motionManager.startDeviceMotionUpdates(to: OperationQueue.init()) { deviceMotion, error in
            guard let deviceMotion = deviceMotion else {
                print("startDeviceMotionUpdates error: \(error!)")
                return
            }
            self.gravity = deviceMotion.gravity
        }
    }
}
