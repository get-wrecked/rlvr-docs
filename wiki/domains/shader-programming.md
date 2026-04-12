---
domain: Shader Programming (GLSL/HLSL/WGSL)
category: creative-code
verification_type: diff
dataset_scale: 100K+ (Shadertoy corpus)
difficulty_range: easy/medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Shader Programming (GLSL/HLSL/WGSL)

## Overview
Write GPU shaders that produce specified visual output: procedural textures, ray marchers, particle effects, post-processing filters. Verification: render the shader and compare output pixels/frames to reference. Shader output is deterministic — same code always produces same pixels.

## Verification Mechanism
```python
import numpy as np

def verify(shader_code: str, reference_image: np.array, 
           resolution: tuple = (512, 512), time: float = 0.0) -> float:
    # Render shader using headless OpenGL/WebGPU
    rendered = render_shader(shader_code, resolution=resolution, 
                            iTime=time, timeout=10)
    if rendered is None:
        return 0.0  # Shader doesn't compile/render
    
    # Pixel comparison
    rendered_arr = np.array(rendered, dtype=np.float32) / 255.0
    reference_arr = np.array(reference_image, dtype=np.float32) / 255.0
    
    # Mean squared error
    mse = np.mean((rendered_arr - reference_arr) ** 2)
    
    # SSIM
    from skimage.metrics import structural_similarity
    ssim = structural_similarity(rendered_arr, reference_arr, multichannel=True)
    
    return 0.5 * max(0, 1 - mse * 100) + 0.5 * ssim
```

## Dataset Sources
- **Shadertoy**: 100K+ public shaders with rendered previews. Can scrape code + screenshot pairs.
- **The Book of Shaders**: Educational examples with known outputs.
- **GLSL Sandbox**: 50K+ fragment shaders.
- **ShaderToyMark**: Benchmark shader collection.
- **Procedural generation**: Specify visual targets (checkerboard, gradient, noise pattern), verify pixel output.

## Task Format
- **Input**: "Write a GLSL fragment shader that renders a Mandelbrot set with smooth coloring, centered at (-0.5, 0) with zoom level 2.0" (+ reference image)
- **Output**: GLSL code

## Difficulty Curriculum
- Level 1: Solid colors, gradients, simple shapes (circle, rectangle)
- Level 3: Procedural textures (checkerboard, noise, wood grain)
- Level 5: Signed distance functions, ray marching basics
- Level 7: Complex ray marching scenes, reflections, shadows
- Level 9: Full procedural worlds, volumetric effects, physically-based rendering

## Limitations & Risks
- GPU rendering determinism varies slightly across hardware. Test on standardized virtual GPU (SwiftShader/Mesa).
- Floating-point precision differences can cause pixel-level discrepancies. Use tolerance in comparison.
- Visual similarity is approximate — but for constrained tasks (render exact mathematical object), verification can be exact.

## Connections
- [[image-generation-constrained]] — code-based image generation
- [[code-generation]] — shader code is code
- [[svg-generation]] — 2D vector graphics alternative
- [[cad-modeling]] — 3D visualization
