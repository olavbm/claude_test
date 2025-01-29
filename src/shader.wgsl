struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x + 1.0, -y + 1.0) * 0.5;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.tex_coords * 2.0 - 1.0;
    
    // Ray origin and direction
    let ro = vec3<f32>(0.0, 0.0, -2.0);
    let rd = normalize(vec3<f32>(uv.x, uv.y, 1.0));
    
    // Ray marching parameters
    let max_steps = 100;
    let max_dist = 10.0;
    let surf_dist = 0.001;
    
    var total_dist = 0.0;
    var i = 0;
    
    // Ray marching loop
    while (i < max_steps && total_dist < max_dist) {
        let p = ro + rd * total_dist;
        let dist = julia_de(p);
        
        total_dist += dist;
        
        if (dist < surf_dist) {
            // Hit surface - calculate color based on iterations
            let t = f32(i) / f32(max_steps);
            return vec4<f32>(t, t * t, 1.0 - t, 1.0);
        }
        
        i += 1;
    }
    
    // Background color
    return vec4<f32>(0.1, 0.2, 0.3, 1.0);
}

fn julia_de(p: vec3<f32>) -> f32 {
    var z = p;
    let c = vec3<f32>(0.4, 0.5, 0.6); // Julia set parameter
    
    var dr = 1.0;
    var r = 0.0;
    
    for(var i = 0; i < 15; i++) {
        r = length(z);
        if (r > 2.0) { break; }
        
        // Convert to polar coordinates
        var theta = acos(z.z / r);
        var phi = atan2(z.y, z.x);
        
        // Scale and rotate the point
        dr = pow(r, 2.0) * 2.0 * dr;
        
        // Convert back to Cartesian coordinates
        var zr = r * r;
        theta = theta * 2.0;
        phi = phi * 2.0;
        
        z = vec3<f32>(
            sin(theta) * cos(phi),
            sin(theta) * sin(phi),
            cos(theta)
        ) * zr + c;
    }
    
    return 0.5 * log(r) * r / dr;
}
