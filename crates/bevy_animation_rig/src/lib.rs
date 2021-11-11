use bevy_app::{App, CoreStage, Plugin, StartupStage};
use bevy_asset::AddAsset;
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemLabel};
use bevy_transform::TransformSystem;

mod skinned_mesh;
pub use skinned_mesh::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum AnimationRigSystem {
    SkinnedMeshSetup,
    SkinnedMeshUpdate,
}

#[derive(Default)]
pub struct AnimationRigPlugin;

impl Plugin for AnimationRigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SkinnedMesh>()
            .add_asset::<SkinnedMeshInverseBindposes>()
            .add_startup_system_to_stage(
                StartupStage::PreStartup,
                skinned_mesh_setup.label(AnimationRigSystem::SkinnedMeshSetup),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                skinned_mesh_update
                    .label(AnimationRigSystem::SkinnedMeshUpdate)
                    .after(TransformSystem::TransformPropagate),
            );
    }
}
