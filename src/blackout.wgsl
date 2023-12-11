#import bevy_pbr::{
    clustered_forward::{fragment_cluster_index, get_light_id, unpack_offset_and_counts},
    forward_io::{VertexOutput, FragmentOutput},
    mesh_view_bindings::{point_lights, view},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    shadows::fetch_point_shadow,
}

fn quantize(color: vec4<f32>) -> vec4<f32> {
    let levels = 10.0;
    let gray = max(color.r, max(color.g, color.b));
    let lower = floor(gray * levels) / levels;
    let lower_diff = abs(gray - lower);
    let upper = ceil(gray * levels) / levels;
    let upper_diff = abs(upper - gray);
    let level = select(upper, lower, lower_diff <= upper_diff);
    let adjustment = level / gray;
    return vec4<f32>(color.rgb * adjustment, color.a);
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) front: bool) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, front);
    let view_z = dot(vec4<f32>(
        view.inverse_view[0].z,
        view.inverse_view[1].z,
        view.inverse_view[2].z,
        view.inverse_view[3].z,
    ), in.world_position);
    let cluster_index = fragment_cluster_index(pbr_input.frag_coord.xy, view_z, pbr_input.is_orthographic);
    let offset_and_counts = unpack_offset_and_counts(cluster_index);
    let offset = offset_and_counts[0];
    let count = offset_and_counts[1];
    for (var i = 0u; i < count; i += 1u) {
        let light_id = get_light_id(i + offset);
        let light = point_lights.data[light_id];
        let light_color = light.color_inverse_square_range.xyz;

        // If the light is black, it means we're using it for visibility testing
        let is_black = light_color.x == 0.0 && light_color.y == 0.0 && light_color.z == 0.0;
        if (is_black) {
            let shadow = fetch_point_shadow(light_id, pbr_input.world_position, pbr_input.world_normal);
            let angle = dot(normalize(light.position_radius.xyz - in.world_position.xyz), in.world_normal);

            // If a single viz "light" can see this fragment, return it as normal and don't discard
            if (shadow > 0.0 && angle > 0.06) {
                pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
                var out: FragmentOutput;
                out.color = main_pass_post_lighting_processing(pbr_input, apply_pbr_lighting(pbr_input));
                return out;
            }
        }
    }
    discard;
}
