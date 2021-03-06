//
//  GraphBuilder.swift
//  Signed
//
//  Created by Markus Moenig on 13/12/20.
//

import Foundation
import Combine

class SignedGraphBuilder: GraphBuilder {
    let core                : Core
    
    let selectionChanged    = PassthroughSubject<UUID?, Never>()
    let contextColorChanged = PassthroughSubject<String, Never>()

    var cursorTimer         : Timer? = nil
    var currentNode         : GraphNode? = nil
    
    var currentFunction     : ExpressionContext.ExpressionNodeItem? = nil
    var currentColumn       : Int32 = 0
        
    init(_ core: Core)
    {
        self.core = core
        super.init()
        
        branches.append(GraphNodeItem("IsometricCamera", { (_ options: [String:Any]) -> GraphNode in return GraphIsometricCameraNode(options) }))
        branches.append(GraphNodeItem("analyticalDome", { (_ options: [String:Any]) -> GraphNode in return GraphAnalyticalDomeNode(options) }))

        branches.append(GraphNodeItem("Sun", { (_ options: [String:Any]) -> GraphNode in return GraphSunLightNode(options) }))
        //branches.append(GraphNodeItem("lightSphere", { (_ options: [String:Any]) -> GraphNode in return GraphSphereLightNode(options) }))
        branches.append(GraphNodeItem("Camera", { (_ options: [String:Any]) -> GraphNode in return GraphCameraNode(options) }))
    }
    
    @discardableResult override func compile(_ asset: Asset, silent: Bool = false) -> CompileError
    {
        var error = super.compile(asset, silent: silent)
        
        if silent == false {
            
            if asset.graph?.cameraNode == nil {
                error.error = "Project must contain a Camera!"
                error.line = 0
            }
            
            if core.state == .Idle {
                if error.error != nil {
                    error.line = error.line! + 1
                    //core.scriptEditor?.setError(error)
                } else {
                    //core.scriptEditor?.clearAnnotations()
                }
            }
            
            if error.error == nil {
                core.renderPipeline.setValid(context: asset.graph!)
            } else {
                core.renderPipeline.setInvalid(error.error!)
            }
            
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
                self.core.modelChanged.send()
            }
        }
        
        return error
    }

    func startTimer(_ asset: Asset)
    {
        DispatchQueue.main.async(execute: {
            let timer = Timer.scheduledTimer(timeInterval: 0.2,
                                             target: self,
                                             selector: #selector(self.cursorCallback),
                                             userInfo: nil,
                                             repeats: true)
            self.cursorTimer = timer
        })
    }
    
    func stopTimer()
    {
        if cursorTimer != nil {
            cursorTimer?.invalidate()
            cursorTimer = nil
        }
    }
    
    var send = false
    var lastContextHelpId   : UUID? = nil
    var lastContextHelpLine : Int = 0
    let expressionContext = ExpressionContext()

    @objc func cursorCallback(_ timer: Timer) {
        
        if core.state == .Idle && core.scriptEditor != nil {
            core.scriptEditor!.getSessionCursor({ (line, column) in
                
                self.currentColumn = column
                self.currentFunction = nil
                
                let lineNr = line

                if let asset = self.core.assetFolder.current, asset.type == .Source {
                    
                    var processed = false
                    if let line = self.core.scriptProcessor.getLine(line) {
                        let word = extractWordAtOffset(line, offset: column, boundaries: " <>\",()")
                        
                        if word.starts(with: "#") && (word.count == 7 || word.count == 9) {
                            // Color ?
                            if self.send == false {
                                self.contextColorChanged.send(word)
                                self.send = true
                            }
                        } else {
                            for f in self.expressionContext.functions {
                                if f.name == word {
                                    self.currentFunction = f
                                    processed = true
                                    if f.id != self.lastContextHelpId || self.lastContextHelpLine != lineNr {
                                        
                                        if let context = asset.graph {
                                            if let node = context.lines[lineNr] {
                                                self.currentNode = node
                                                
                                                let functionNode = f.createNode()
                                                                                                
                                                self.core.contextText = self.generateNodeHelpText(functionNode)
                                                self.core.contextTextChanged.send(self.core.contextText)
                                                self.selectionChanged.send(f.id)
                                                
                                                self.lastContextHelpId = f.id
                                                self.lastContextHelpLine = Int(lineNr)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    if processed == false {
                        if let context = asset.graph {
                            if let node = context.lines[line] {
                                if node.id != self.lastContextHelpId {
                                    self.currentNode = node
                                    self.selectionChanged.send(node.id)
                                    self.core.contextText = self.generateNodeHelpText(node)
                                    self.core.contextTextChanged.send(self.core.contextText)
                                    self.lastContextHelpId = node.id
                                    if node.hasToolUI {
                                        // Validate the Tool UI Texture
                                        self.core.toolContext.validate()
                                    }
                                }
                            } else {
                                if self.lastContextHelpId != nil {
                                    self.currentNode = nil
                                    self.selectionChanged.send(nil)
                                    self.core.contextText = ""
                                    self.core.contextTextChanged.send(self.core.contextText)
                                    self.lastContextHelpId = nil
                                }
                            }
                        }
                    }
                }
            })
        }
    }
    
    /// Generates a markdown help text for the given node
    func generateNodeHelpText(_ node: GraphNode) -> AttributedString
    {
        var help = "## " + node.name + "\n"
        help += node.getHelp()
        let options = node.getOptions()
        if options.count > 0 {
            help += "\nOptional Parameters\n"
        }
        for o in options {
            help += "* **\(o.name)** (\(o.variable.getTypeName())) - " + o.help + "\n"
        }
        return try! AttributedString(markdown: help)
    }
    
    /// Generates a markdown help text for the given expression node
    func generateNodeHelpText(_ node: ExpressionNode) -> AttributedString
    {
        var help = "## " + node.name + "\n"
        help += node.getHelp()
        let options = node.getOptions()
        if options.count > 0 {
            help += "\nOptional Parameters\n"
        }
        for o in options {
            help += "* **\(o.name)** (\(o.variable.getTypeName())) - " + o.help + "\n"
        }
        return try! AttributedString(markdown: help)
    }
    
    /// Go to the line of the node
    func gotoNode(_ node: GraphNode)
    {
        if currentNode != node {
            core.scriptEditor?.gotoLine(node.lineNr+1)
            currentNode = node
        }
    }
}
