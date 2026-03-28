pub use makepad_widgets;
pub use makepad_widgets::*;

#[path = "obj/cube.rs"]
pub mod cube;
#[path = "obj/gltf.rs"]
pub mod gltf;
#[path = "util/gltf_bridge.rs"]
pub mod gltf_bridge;
#[path = "obj/icosphere.rs"]
pub mod icosphere;
#[path = "util/mesh_generators.rs"]
pub mod mesh_generators;
#[path = "util/passthrough_env.rs"]
pub mod passthrough_env;
#[path = "obj/physics_view.rs"]
pub mod physics_view;
#[path = "obj/refractive_cube.rs"]
pub mod refractive_cube;
#[path = "util/scene_draw.rs"]
mod scene_draw;
#[path = "obj/shooter.rs"]
pub mod shooter;
#[path = "obj/tree.rs"]
pub mod tree;
#[path = "obj/view_splat.rs"]
pub mod view_splat;
#[path = "scene/xr_body_spawn.rs"]
pub mod xr_body_spawn;
#[path = "scene/xr_env.rs"]
pub mod xr_env;
#[path = "scene/xr_gesture.rs"]
mod xr_gesture;
#[path = "xr_net.rs"]
pub mod xr_net;
#[path = "scene/xr_node.rs"]
pub mod xr_node;
#[path = "scene/xr_people_debug.rs"]
pub mod xr_people_debug;
#[path = "scene/xr_permissions_flow.rs"]
pub mod xr_permissions_flow;
#[path = "scene/xr_root.rs"]
pub mod xr_root;
#[path = "scene/xr_select.rs"]
pub mod xr_select;
#[path = "scene/xr_view.rs"]
pub mod xr_view;

pub mod render {
    pub use crate::util::gltf_bridge::{
        GltfDecodedMeshes, GltfDecodedPrimitiveObject, GltfDefaultView, GltfDrawObject,
        GltfMaterialState, GltfMeshObjects, GltfPrimitiveObject, GltfRenderer,
    };
    pub use crate::util::passthrough_env::DrawPassthroughEnvFace;
}

pub(crate) mod prelude {
    pub use crate::algorithms::depth_align::*;
    pub use crate::{net::*, render::*, scene::*};
    pub use makepad_widgets::*;
}

pub fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
    scene::xr_node::script_mod(vm);
    obj::car::script_mod(vm);
    obj::gltf::script_mod(vm);
    obj::icosphere::script_mod(vm);
    obj::cube::script_mod(vm);
    scene::xr_permissions_flow::script_mod(vm);
    obj::physics_view::script_mod(vm);
    obj::refractive_cube::script_mod(vm);
    obj::shooter::script_mod(vm);
    obj::tank::script_mod(vm);

    util::passthrough_env::script_mod(vm);
    obj::tree::script_mod(vm);
    obj::view_splat::script_mod(vm);
    scene::xr_env::script_mod(vm);
    sync::xr_peer_sync::script_mod(vm);
    sync::xr_scene_sync_controller::script_mod(vm);
    scene::xr_select::script_mod(vm);
    scene::xr_view::script_mod(vm);
    scene::xr_root::script_mod(vm)
}
