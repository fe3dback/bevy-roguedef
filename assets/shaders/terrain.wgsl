#import bevy_pbr::forward_io::VertexOutput

#import "shaders/noise.wgsl"::simplexNoise2
#import "shaders/colors.wgsl"::{rgb2hsv, hsv2rgb}

@group(2) @binding(0) var<uniform> map_size: vec2<f32>;

@group(2) @binding(1) var texture_world_color: texture_2d<f32>;
@group(2) @binding(2) var texture_world_color_sampler: sampler;

@group(2) @binding(3) var texture_grass: texture_2d<f32>;
@group(2) @binding(4) var texture_grass_sampler: sampler;

@group(2) @binding(5) var texture_rock: texture_2d<f32>;
@group(2) @binding(6) var texture_rock_sampler: sampler;

// consts
const COLOR_VARIANT_SCALE_SM = 25.0;
const COLOR_VARIANT_SCALE_BIG = 100.0;

const TEX_GRASS_UV_SCALE = 5.0;
const TEX_ROCK_UV_SCALE = 6.0;


fn copy_color_hue(src: vec3<f32>, dst: vec3<f32>) -> vec3<f32> {
    let src_hsv = rgb2hsv(src);
    let dst_hsv = rgb2hsv(dst);

    return hsv2rgb(vec3(dst_hsv.x, dst_hsv.y, src_hsv.z));
}

fn color_variation_mask(pos: vec2<f32>) -> f32 {
    let noise_sm = simplexNoise2(pos * (1/COLOR_VARIANT_SCALE_SM));
    let noise_big = simplexNoise2(pos * (1/COLOR_VARIANT_SCALE_BIG));
    let noise = noise_sm * 0.35 * (noise_big * 1.4);

    return noise;
}

fn cliff_mask(norm: vec3<f32>) -> f32 {
    return smoothstep(0.4, 0.9, 1.0-norm.y);
}

fn albedo_world_color(pos: vec2<f32>) -> vec3<f32> {
    let world_pos = pos + vec2(map_size.x*0.5, map_size.y*0.5);
    let world_uv = world_pos * vec2((1 / map_size.x), (1 / map_size.y));

    let world_albedo = textureSample(texture_world_color, texture_world_color_sampler, world_uv);

    return world_albedo.rgb;
}

fn albedo_grass(uv: vec2<f32>, world_color: vec3<f32>, color_variation_mask: f32) -> vec3<f32> {
    let tex_base = textureSample(texture_grass, texture_grass_sampler, uv * TEX_GRASS_UV_SCALE);
    let colored = copy_color_hue(tex_base.rgb, world_color);
    let variation = mix(colored, world_color, color_variation_mask);

    return variation.rgb;
}

fn albedo_cliffs(uv: vec2<f32>, world_color: vec3<f32>, color_variation_mask: f32) -> vec3<f32> {
    let tex_base = textureSample(texture_rock, texture_rock_sampler, uv * TEX_ROCK_UV_SCALE);
    let colored = copy_color_hue(tex_base.rgb, world_color);
    let variation = mix(colored, world_color, color_variation_mask);

    return variation.rgb;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let world_color = albedo_world_color(mesh.world_position.xz);
    let color_variation_mask = color_variation_mask(mesh.world_position.xz);

    // base albedos
    let albedo_grass = albedo_grass(mesh.uv, world_color, color_variation_mask);
    let albedo_cliffs = albedo_cliffs(mesh.uv, world_color, color_variation_mask);

    // overlays
    let cliff_mask = cliff_mask(mesh.world_normal);
    let albedo_grass_and_cliffs = mix(albedo_grass, albedo_cliffs, cliff_mask);

    // result albedo
    let albedo = albedo_grass_and_cliffs;

    return vec4(albedo.rgb, 1.0);

}