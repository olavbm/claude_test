@group(0) @binding(0)
var<uniform> time: f32;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    var pos = vec2<f32>(0.0, 0.0);
    switch(in_vertex_index) {
        case 0u: { pos = vec2<f32>(-1.0, -1.0); }
        case 1u: { pos = vec2<f32>( 3.0, -1.0); }
        case 2u: { pos = vec2<f32>(-1.0,  3.0); }
        default: { pos = vec2<f32>(0.0, 0.0); }
    }
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.tex_coords = pos * 0.5 + 0.5;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.tex_coords * 2.0 - 1.0;
    
    // Ray origin and direction
    let ro = vec3<f32>(0.0, 0.0, -4.0);
    let rd = normalize(vec3<f32>(uv.x, uv.y, 1.0));
    
    // Ray marching parameters
    let max_steps = 50;
    let max_dist = 8.0;
    let surf_dist = 0.002;
    
    var total_dist = 0.0;
    var i = 0;
    
    // Ray marching loop
    while (i < max_steps && total_dist < max_dist) {
        let p = ro + rd * total_dist;
        let dist = julia_de(p);
        
        total_dist += dist;
        
        if (dist < surf_dist) {
            // Calculate normal for better shading
            let eps = vec3<f32>(0.001, 0.0, 0.0);
            let normal = normalize(vec3<f32>(
                julia_de(p + eps.xyy) - julia_de(p - eps.xyy),
                julia_de(p + eps.yxy) - julia_de(p - eps.yxy),
                julia_de(p + eps.yyx) - julia_de(p - eps.yyx)
            ));
            
            // Create more interesting coloring based on normal and iterations
            let t = f32(i) / f32(max_steps);
            // Enhanced coloring with better depth perception
            let color = vec3<f32>(
                0.5 + 0.5 * normal.x + 0.2 * t,
                0.3 + 0.5 * normal.y + 0.1 * t,
                0.4 + 0.5 * normal.z + 0.3 * t
            );
            
            return vec4<f32>(color, 1.0);
        }
        
        i += 1;
    }
    
    // Background color
    return vec4<f32>(0.1, 0.2, 0.3, 1.0);
}

fn julia_de(p: vec3<f32>) -> f32 {
    var z = p;
    // Animate the Julia set parameters using sin waves
    let c = vec3<f32>(
        0.1 + 0.3 * sin(time * 0.5),
        0.2 + 0.3 * cos(time * 0.3),
        0.3 + 0.2 * sin(time * 0.4)
    );
    
    var dr = 1.0;
    var r = 0.0;
    
    // Reduced iterations for better performance
    for(var i = 0; i < 15; i++) {
        r = length(z);
        if (r > 2.0) { break; }
        
        // Convert to polar coordinates
        var theta = acos(z.z / r);
        var phi = atan2(z.y, z.x);
        
        dr = pow(r, 2.0) * 2.0 * dr;
        
        // Simplify calculations
        theta = theta * 2.0;
        phi = phi * 2.0;
        
        let sin_theta = sin(theta);
        z = vec3<f32>(
            sin_theta * cos(phi),
            sin_theta * sin(phi),
            cos(theta)
        ) * (r * r) + c;
    }
    
    return 0.5 * log(r) * r / dr;
}
