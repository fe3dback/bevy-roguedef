fn rgb2hsv(col: vec3<f32>) -> vec3<f32> {
    let K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    let p = mix(vec4(col.bg, K.wz), vec4(col.gb, K.xy), step(col.b, col.g));
    let q = mix(vec4(p.xyw, col.r), vec4(col.r, p.yzx), step(p.x, col.r));

    let d = q.x - min(q.w, q.y);
    let e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

fn hsv2rgb(col: vec3<f32>) -> vec3<f32> {
    let rgb = clamp(
        abs(
            ((col.r * 6.0 + vec3<f32>(0.0, 4.0, 2.0)) % 6.0) - 3.0
        ) - 1.0,
        vec3<f32>(0.0),
        vec3<f32>(1.0)
    );

    return col.b * mix(vec3<f32>(1.0), rgb, vec3<f32>(col.g));
}