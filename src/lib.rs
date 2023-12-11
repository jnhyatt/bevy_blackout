use bevy_app::{App, Plugin};
use bevy_asset::{load_internal_asset, Handle};
use bevy_pbr::MaterialPlugin;
use bevy_render::render_resource::Shader;
pub use material::BlackoutMaterial;

mod material;

/// Adds the [`MaterialPlugin`] for [`BlackoutMaterial`] to the app.
pub struct BlackoutPlugin;

const SHADER: Handle<Shader> = Handle::weak_from_u128(5738253885312041122);

impl Plugin for BlackoutPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER, "blackout.wgsl", Shader::from_wgsl);
        app.add_plugins(MaterialPlugin::<BlackoutMaterial>::default());
    }
}
