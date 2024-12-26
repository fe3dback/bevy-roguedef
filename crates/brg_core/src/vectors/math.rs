/// Return value between a and b on time t
/// example: a=5, b=10, t=0.5 will return 7.5
#[inline(always)]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    ((1.0 - t) * a) + (b * t)
}

/// Return time between a and b and value
/// condition: [ a <= v <= b ]
/// example: a=5, b=10, v=7.5 will return 0.5
#[inline(always)]
pub fn lerp_inv(a: f32, b: f32, v: f32) -> f32 {
    let v = v.clamp(a, b);
    (v - a) / (b - a)
}

/// Remap value on different range
/// is just combination of two lerps
#[inline(always)]
pub fn remap(in_min: f32, in_max: f32, out_min: f32, out_max: f32, v: f32) -> f32 {
    let t = lerp_inv(in_min, in_max, v);
    lerp(out_min, out_max, t)
}

/// Normalize f32 value and clamp in range 0..1
/// for example:
/// - (dist=125, max=100) -> 1.0 (clamped)
/// - (dist=100, max=200) -> 0.5
/// - (dist=-50, max=100) -> 0.0 (clamped)
#[inline(always)]
pub fn norm_value(v: f32, max: f32) -> f32 {
    if v >= max {
        return 1.0;
    }

    if v <= f32::EPSILON {
        return 0.0;
    }

    v / max
}
