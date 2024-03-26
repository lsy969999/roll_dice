//
//  MetalView.swift
//  roll_dice
//
//  Created by SY L on 3/22/24.
//

import Foundation
import UIKit

class MetalView: UIView {
    override class var layerClass: AnyClass {
        return CAMetalLayer.self
    }
    
    override func awakeFromNib() {
        super.awakeFromNib()
        configLayer()
        self.layer.backgroundColor = UIColor.clear.cgColor
    }
    
    private func configLayer() {
        guard let layer = self.layer as? CAMetalLayer else {
            return
        }
        layer.presentsWithTransaction = false
        layer.framebufferOnly = true
        self.contentScaleFactor = UIScreen.main.nativeScale
    }
}
