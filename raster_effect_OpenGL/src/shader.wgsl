[[stage(vertex)]]
fn vs_main([[location(0)]] position: vec2<f32>, [[location(1)]] color: vec3<f32>) -> [[builtin(position)]] vec4<f32> {
    var out: vec4<f32>;
    out.xy = position;
    out.zw = vec2(0.0, 1.0);
    return out;
}

[[stage(fragment)]]
fn fs_main([[location(0)]] color: vec3<f32>) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(color, 1.0);
}
