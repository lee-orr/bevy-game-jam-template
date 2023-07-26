#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings    view
#import bevy_pbr::pbr_types             STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT
#import bevy_core_pipeline::tonemapping tone_mapping
#import bevy_pbr::pbr_functions as fns

@group(1) @binding(0)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(1)
var base_color_sampler: sampler;
@group(1) @binding(2)
var shadow_color_texture: texture_2d<f32>;
@group(1) @binding(3)
var shadow_color_sampler: sampler;

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let layer = i32(mesh.world_position.x) & 0x3;

    let color =  textureSample(base_color_texture, base_color_sampler, mesh.uv);

    var pbr_input: fns::PbrInput = fns::pbr_input_new();
    
    pbr_input.material.base_color =color;
    // pbr_input.material.base_color = vec4<f32>(1., 1., 1., 1.);

    pbr_input.frag_coord = mesh.position;
    pbr_input.world_position = mesh.world_position;
    pbr_input.world_normal = fns::prepare_world_normal(
        mesh.world_normal,
        (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
        is_front,
    );

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    pbr_input.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        mesh.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
        mesh.world_tangent,
#endif
#endif
        mesh.uv,
        view.mip_bias,
    );
    pbr_input.V = fns::calculate_view(mesh.world_position, pbr_input.is_orthographic);


    let result = fns::pbr(pbr_input);

    let min = min(result.x, min(result.y, result.z));
    let max = max(result.x, max(result.y, result.z));

    let max_m_min = max - min;
    let max_p_min = max + min;

    let l = max_p_min / 2.0;

    let shadow = textureSample(shadow_color_texture, shadow_color_sampler, vec2<f32>(l, 0.5));

    let result_shadeless = clamp(result / l, vec4<f32>(0., 0., 0., 0.), vec4<f32>(1., 1., 1., 1.));

    let shaded = color * shadow;
    let shaded_with_hue = result_shadeless * shaded;   

    return 1. - (1. - shaded) * (1. - shaded_with_hue);
    // return result;
}
