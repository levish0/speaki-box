use std::path::Path;

use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::prelude::*;

const SPRITE_SHADER_PATH: &str = "bevy_sprite_render/sprite_mesh/sprite_material.wesl";
const NAMED_INSTANCE_INDEX: &str = "let instance_index = vertex.instance_index;";
const LOCAL_INSTANCE_INDEX: &str = "var instance_index = vertex.instance_index;";

pub fn install_dx12_sprite_shader(app: &mut App) {
    // Naga 30's HLSL writer adds first_instance twice for a named builtin expression.
    // A local variable keeps the adjusted index without receiving another offset.
    // Register before RenderStartup loads the SpriteMeshMaterial shader.
    let registry = app.world().resource::<EmbeddedAssetRegistry>();
    let path = Path::new(SPRITE_SHADER_PATH);
    let original = registry
        .remove_asset(path)
        .expect("Bevy sprite shader must be registered");
    let source = std::str::from_utf8(original.value()).expect("Bevy sprite shader must be UTF-8");
    assert_eq!(
        source.matches(NAMED_INSTANCE_INDEX).count(),
        1,
        "Bevy sprite shader changed; revalidate the Naga HLSL instance-index fix"
    );
    let corrected = source.replacen(NAMED_INSTANCE_INDEX, LOCAL_INSTANCE_INDEX, 1);
    registry.insert_asset(path.to_owned(), path, corrected.into_bytes());
}

#[cfg(test)]
mod tests {
    use bevy::asset::AssetPlugin;
    use bevy::shader::{Shader, ShaderLoader};
    use bevy::sprite_render::SpriteMeshPlugin;
    use naga::{
        back::hlsl,
        front::wgsl,
        valid::{Capabilities, ValidationFlags, Validator},
    };

    use super::*;

    #[test]
    fn registered_bevy_sprite_shader_uses_a_local_instance_index() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .init_asset::<Shader>()
            .init_asset_loader::<ShaderLoader>()
            .add_plugins(SpriteMeshPlugin);
        install_dx12_sprite_shader(&mut app);
        let registry = app.world().resource::<EmbeddedAssetRegistry>();
        let shader = registry
            .remove_asset(Path::new(SPRITE_SHADER_PATH))
            .unwrap();
        let source = std::str::from_utf8(shader.value()).unwrap();
        assert!(source.contains(LOCAL_INSTANCE_INDEX));
        assert!(!source.contains(NAMED_INSTANCE_INDEX));
    }

    #[test]
    fn dx12_hlsl_adds_the_draw_offset_only_once() {
        let source = r#"
            struct Vertex { @builtin(instance_index) instance_index: u32, }
            @vertex fn vertex(vertex: Vertex) -> @builtin(position) vec4<f32> {
                var instance_index = vertex.instance_index;
                return vec4<f32>(f32(instance_index), f32(instance_index + 27u), 0.0, 1.0);
            }
        "#;
        let module = wgsl::parse_str(source).unwrap();
        let info = Validator::new(ValidationFlags::all(), Capabilities::all())
            .validate(&module)
            .unwrap();
        let options = hlsl::Options {
            special_constants_binding: Some(hlsl::BindTarget::default()),
            ..default()
        };
        let mut output = String::new();
        hlsl::Writer::new(&mut output, &options, &default())
            .write(&module, &info, None)
            .unwrap();
        assert_eq!(
            output.matches("first_instance +").count(),
            1,
            "draw offset was applied more than once:\n{output}"
        );
    }
}
