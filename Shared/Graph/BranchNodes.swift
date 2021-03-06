//
//  GraphBranchNodes.swift
//  Signed
//
//  Created by Markus Moenig on 13/12/20.
//

import Foundation

/// Distance Base Node
class GraphTransformationNode : GraphNode
{
    var position        = Float3(0,0,0)
    var rotation        = Float3(0,0,0)
    var scale           = Float1(1)
    
    func verifyTranslationOptions(context: GraphContext, error: inout CompileError) {
        if let value = extractFloat3Value(options, container: context, error: &error, name: "position", isOptional: true) {
            position = value
        }
        if let value = extractFloat3Value(options, container: context, error: &error, name: "rotation", isOptional: true) {
            rotation = value
        }
        if let value = extractFloat1Value(options, container: context, error: &error, name: "scale", isOptional: true) {
            scale = value
        }
    }
    
    /// Generates the transform code
    func generateMetalTransformCode(context: GraphContext) -> String
    {
        context.addDataVariable(position)

        let code =
        """

                float3 transformedPosition = (position - objectPosition) / objectScale * \(scale.toMetal());
                
                transformedPosition = translate(transformedPosition, \(position.toMetal()));
                float3 offsetFromCenter = objectPosition - \(position.toMetal());

                float3 rotation = objectRotation + \(rotation.toMetal());

                transformedPosition.yz = rotatePivot(transformedPosition.yz, radians(rotation.x), offsetFromCenter.yz );
                transformedPosition.xz = rotatePivot(transformedPosition.xz, radians(rotation.y), offsetFromCenter.xz );
                transformedPosition.xy = rotatePivot(transformedPosition.xy, radians(rotation.z), offsetFromCenter.xy );
        """
        
        return code
    }
    
    static func getTransformationOptions() -> [GraphOption]
    {
        return [
            GraphOption(Float3(0,0,0), "Position", "The position of the object."),
            GraphOption(Float3(0,0,0), "Rotation", "The rotation of the object."),
            GraphOption(Float1(1), "Scale", "The scale of the object.")
        ]
    }
    
    /// Checks in the transformation settings of this node
    func checkIn(context: GraphContext) -> String
    {
        let code =
        """
        
        objectPosition += \(position.toMetal());
        objectRotation += \(rotation.toMetal());
        objectScale *= \(scale.toMetal());

        """
        
        context.position += position.toSIMD()
        context.rotation += rotation.toSIMD()
        context.scale *= scale.toSIMD()

        if let index = position.dataIndex, index < context.data.count {
            context.data[index] = float4(context.position.x, context.position.y, context.position.z, 0)
        }
        
        if let index = rotation.dataIndex, index < context.data.count {
            context.data[index] = float4(context.rotation.x, context.rotation.y, context.rotation.z, 0)
        }
        
        if let index = scale.dataIndex, index < context.data.count {
            context.data[index] = float4(context.scale, 0, 0, 0)
        }
        
        return code
    }
    
    /// Checks out the transformation settings of this node
    func checkOut(context: GraphContext) -> String
    {
        let code =
        """
        
        objectPosition -= \(position.toMetal());
        objectRotation -= \(rotation.toMetal());
        objectScale /= \(scale.toMetal());

        """
        
        context.position -= position.toSIMD()
        context.rotation -= rotation.toSIMD()
        context.scale /= scale.toSIMD()
        
        return code
    }
    
    override func getToolViewButtons() -> [ToolViewButton]
    {
        return [ToolViewButton(name: "Move"), ToolViewButton(name: "Rotate"), ToolViewButton(name: "Scale")]
    }
    
    var maxDepthBuffer  : Int = 1
    override func toolViewButtonAction(_ button: ToolViewButton, state: ToolViewButton.State, delta: float2, toolContext: GraphToolContext)
    {
        /*
        if state == .Down || delta == float2(0,0) {
            toolContext.validate()
            maxDepthBuffer = toolContext.core.renderPipeline.maxDepth
            toolContext.core.renderPipeline.maxDepth = 1
        } else
        if state == .Move {
            if button.name == "Move" {
                var p = position.toSIMD()
                
                p.x += delta.x * 0.1
                
                position.fromSIMD(p)
            } else
            if button.name == "Rotate" {
                var p = rotation.toSIMD()
                
                p.x += delta.x
                
                rotation.fromSIMD(p)
            } else
            if button.name == "Scale" {
                var p = scale.toSIMD()
                
                p += delta.x * 0.1
                
                scale.fromSIMD(p)
            }
            toolContext.core.renderPipeline.restart()
        } else
        if state == .Up {
            toolContext.core.scriptProcessor.replaceFloat3InLine(["Position": position, "Rotation": rotation])
            toolContext.core.scriptProcessor.replaceFloat1InLine(["Scale": scale])
            toolContext.core.renderPipeline.maxDepth = maxDepthBuffer
            toolContext.core.renderPipeline.restart()
        }
        */
    }
}

/// SDFObject
final class GraphSDFObject : GraphTransformationNode
{
    var maxBox        : Float3? = nil
    var positionStore : Float3!
    
    var materialName  : String? = nil

    var steps         : Float1 = Float1(70)
    var stepSize      : Float1 = Float1(1)

    init(_ options: [String:Any] = [:])
    {
        super.init(.Utility, .SDF, options)
        name = "sdfObject"
        leaves = []
    }
    
    override func verifyOptions(context: GraphContext, error: inout CompileError) {
        verifyTranslationOptions(context: context, error: &error)
        if let name = options["name"] as? String {
            self.givenName = name.replacingOccurrences(of: "\"", with: "", options: NSString.CompareOptions.literal, range: nil)
        } else {
            self.givenName = "SDF Object"
        }
        if let materialName = options["material"] as? String {
            self.materialName = materialName.replacingOccurrences(of: "\"", with: "", options: NSString.CompareOptions.literal, range: nil)
        }
        if let value = extractFloat3Value(options, container: context, error: &error, name: "maxbox", isOptional: true) {
            maxBox = value
        }
        if let value = extractFloat1Value(options, container: context, error: &error, name: "steps", isOptional: true) {
            steps = value
        }
        if let value = extractFloat1Value(options, container: context, error: &error, name: "stepsize", isOptional: true) {
            stepSize = value
        }
    }
    
    @discardableResult @inlinable public override func execute(context: GraphContext) -> Result
    {
        _ = checkIn(context: context)
        
        if let materialName = materialName {
            context.activeMaterial = context.getMaterial(materialName)
        }
        
        for leave in leaves {
            leave.execute(context: context)
        }

        _ = checkOut(context: context)
        return .Success
    }
    
    override func generateMetalCode(context: GraphContext) -> String
    {        
        if let materialName = materialName {
            context.activeMaterial = context.getMaterial(materialName)
        }
                        
        var code = checkIn(context: context)
        
        for leave in leaves {
            code += leave.generateMetalCode(context: context)
        }
        
        code += checkOut(context: context)
                
        return code
    }
    
    override func setEnvironmentVariables(context: GraphContext)
    {
        context.funcParameters = []
        
        context.variables = ["rayPosition": Float3("rayPosition", 0, 0, 0, .System)]
        context.variables["viewSize"] = Float2("viewSize", 0, 0, .System)
        context.variables["uv"] = Float2("uv", 0, 0, .System)
        context.variables["hash"] = Float1("hash", 0, .System)
    }
    
    override func getHelp() -> String
    {
        return "Defines an SDF Object. SDF objects contain lists of SDF primitives and booleans and can also contain child objects."
    }
    
    override func getOptions() -> [GraphOption]
    {
        let options = [
            GraphOption(Text1("Object"), "Name", "The name of the object."),
            GraphOption(Float1(70), "Steps", "The amount of ray-marching steps for this object."),
            GraphOption(Float1(1), "StepSize", "The ray-marching step size. Valid values are between 0..1. Lower values increase quality but need more steps.")
        ]
        return options + GraphTransformationNode.getTransformationOptions()
    }
}

/// AnalyticalObject
final class GraphAnalyticalObject : GraphTransformationNode
{
    var positionStore : Float3!

    var materialName  : String? = nil

    init(_ options: [String:Any] = [:])
    {
        super.init(.Utility, .Analytical, options)
        name = "analyticalObject"
        leaves = []
    }
    
    override func verifyOptions(context: GraphContext, error: inout CompileError) {
        verifyTranslationOptions(context: context, error: &error)
        if let name = options["name"] as? String {
            self.givenName = name.replacingOccurrences(of: "\"", with: "", options: NSString.CompareOptions.literal, range: nil)
        } else {
            self.givenName = "Analytical Object"
        }
        if let materialName = options["material"] as? String {
            self.materialName = materialName.replacingOccurrences(of: "\"", with: "", options: NSString.CompareOptions.literal, range: nil)
        }
    }
    
    @discardableResult @inlinable public override func execute(context: GraphContext) -> Result
    {
        return .Success
    }
    
    override func generateMetalCode(context: GraphContext) -> String
    {
        var code = checkIn(context: context)
        
        if let materialName = materialName {
            context.activeMaterial = context.getMaterial(materialName)
        }
        
        for leave in leaves {
            code += leave.generateMetalCode(context: context)
        }
        
        code += checkOut(context: context)
                
        return code
    }
    
    override func getHelp() -> String
    {
        return "Defines an analytical object. Analytical objects contain lists of analytical primitives and also contain child objects."
    }
    
    override func getOptions() -> [GraphOption]
    {
        let options = [
            GraphOption(Text1("Object"), "Name", "The name of the object.")
        ]
        return options + GraphTransformationNode.getTransformationOptions()
    }
}

/// MaterialNode
final class GraphMaterialNode : GraphNode
{
    /// Material has displacement
    var hasDisplacement : Bool = false

    /// Material emits light
    var isEmitter       : Bool = false
    var emission        = float3()

    /// Material should only output displacement
    var onlyDisplacement : Bool = false

    init(_ options: [String:Any] = [:])
    {
        super.init(.Utility, .Material, options)
        name = "Material"
        leaves = []
    }
    
    override func verifyOptions(context: GraphContext, error: inout CompileError) {
        if let name = options["name"] as? String {
            self.givenName = name.replacingOccurrences(of: "\"", with: "", options: NSString.CompareOptions.literal, range: nil)
        } else {
            error.error = "Material needs a 'Name' parameter"
        }
    }
    
    @discardableResult @inlinable public override func execute(context: GraphContext) -> Result
    {
        let buffer = context.variables["outColor"]
        if onlyDisplacement {
            context.variables["outColor"] = Float4()
        }
        for leave in leaves {
            leave.execute(context: context)
        }
        context.variables["outColor"] = buffer
        return .Success
    }
    
    override func generateMetalCode(context: GraphContext) -> String
    {
        setEnvironmentVariables(context: context)

        var code = ""
        
        for leave in leaves {
            code += leave.generateMetalCode(context: context)
        }
                
        return code
    }
    
    override func setEnvironmentVariables(context: GraphContext)
    {
        context.funcParameters = []
        
        context.variables = ["rayPosition": Float3("rayPosition", 0, 0, 0, .System)]
        context.variables["viewSize"] = Float2("viewSize", 0, 0, .System)
        context.variables["uv"] = Float2("uv", 0, 0, .System)
        context.variables["hash"] = Float1("hash", 0, .System)
        context.variables["gradient"] = Float1("gradient", 0, .System)
    }
    
    override func getHelp() -> String
    {
        return "Defines an Material."
    }
    
    override func getOptions() -> [GraphOption]
    {
        let options = [
            GraphOption(Text1("Material"), "Name", "The name of the material.")
        ]
        return options
    }
}

/// RenderNode
final class GraphRenderNode : GraphNode
{
    init(_ options: [String:Any] = [:])
    {
        super.init(.Render, .None, options)
        name = "Render"
        leaves = []
    }
    
    override func verifyOptions(context: GraphContext, error: inout CompileError) {
        if let name = options["name"] as? String {
            self.givenName = name.replacingOccurrences(of: "\"", with: "", options: NSString.CompareOptions.literal, range: nil)
        } else {
            error.error = "Render node needs a 'Name' parameter"
        }
    }
    
    @discardableResult @inlinable public override func execute(context: GraphContext) -> Result
    {
        for leave in leaves {
            leave.execute(context: context)
        }
        return .Success
    }
    
    override func getHelp() -> String
    {
        return "Renders the scene."
    }
    
    override func getOptions() -> [GraphOption]
    {
        let options = [
            GraphOption(Text1("Render"), "Name", "The name of the render node.")
        ]
        return options
    }
}
