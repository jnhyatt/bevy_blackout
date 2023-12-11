use bevy_asset::Asset;
use bevy_pbr::{ExtendedMaterial, MaterialExtension, StandardMaterial};
use bevy_reflect::Reflect;
use bevy_render::render_resource::{AsBindGroup, ShaderRef};

use crate::SHADER;

/// Extension to [`StandardMaterial`] that discards fragments that aren't
/// visible to any visibility casters (point lights with color set to black).
///
/// Example usage:
/// ```rs
/// commands.spawn(MaterialMeshBundle {
///     mesh,
///     material: materials.add(BlackoutMaterial {
///         base: StandardMaterial { ... },
///         extension: default(),
///     },
///     ..default()
/// });
/// ```
pub type BlackoutMaterial = ExtendedMaterial<StandardMaterial, BlackoutExt>;

#[derive(Asset, Default, Clone, Copy, AsBindGroup, Reflect)]
pub struct BlackoutExt {}

impl MaterialExtension for BlackoutExt {
    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }
}
