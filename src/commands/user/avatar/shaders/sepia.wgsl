@group(0) @binding(0) var input_texture : texture_2d<f32>;
@group(0) @binding(1) var output_texture : texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(16, 16)
fn sepia_main(
  @builtin(global_invocation_id) global_id : vec3<u32>,
) {
    let dimensions = textureDimensions(input_texture);
    let coords = vec2<i32>(global_id.xy);

    if(coords.x >= dimensions.x || coords.y >= dimensions.y) {
        return;
    }

    let color = textureLoad(input_texture, coords.xy, 0);

    let r = dot(color.rgb, vec3<f32>(0.393, 0.769, 0.189));
    let g = dot(color.rgb, vec3<f32>(0.349, 0.686, 0.168));
    let b = dot(color.rgb, vec3<f32>(0.272, 0.534, 0.131));
    
    let sepia = vec3<f32>(r, g, b);

    textureStore(output_texture, coords.xy, vec4<f32>(sepia, color.a));
}
