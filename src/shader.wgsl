struct Uniforms {
    time: f32,
    camera_pos: vec3<f32>,
    camera_rotation: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

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
    
    // Calculate view direction based on camera position and rotation
    let angle = uniforms.camera_rotation;
    let rot_matrix = mat3x3<f32>(
        cos(angle), 0.0, -sin(angle),
        0.0, 1.0, 0.0,
        sin(angle), 0.0, cos(angle)
    );
    
    out.tex_coords = (rot_matrix * vec3<f32>(pos, 1.0)).xy;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.tex_coords * 2.0 - 1.0;
    
    // Calculate view matrix
    let angle = uniforms.camera_rotation;
    let rot_matrix = mat3x3<f32>(
        cos(angle), 0.0, -sin(angle),
        0.0, 1.0, 0.0,
        sin(angle), 0.0, cos(angle)
    );
    
    // Ray setup with camera looking at origin
    let ro = uniforms.camera_pos;
    let forward = normalize(-ro); // Direction to origin (0,0,0)
    let right = normalize(cross(forward, vec3<f32>(0.0, 1.0, 0.0)));
    let up = normalize(cross(right, forward));
    
    // Construct ray direction with adjusted field of view
    let rd = normalize(forward + right * uv.x * 1.5 + up * uv.y * 1.5);
    
    // Ray marching parameters
    let max_steps = 100;
    let max_dist = 200.0;
    let surf_dist = 0.005;
    
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
    // Animate the Julia set with bounded parameters
    let c = vec3<f32>(
        0.1 + 0.2 * sin(uniforms.time * 0.2),
        0.2 + 0.2 * cos(uniforms.time * 0.15),
        0.3 + 0.15 * sin(uniforms.time * 0.1)
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
