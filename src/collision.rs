use crate::AppState;
use bevy::prelude::*;
use impacted::CollisionShape;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(update_collision_transforms),
        );
    }
}

fn update_collision_transforms(
    mut shapes: Query<(&mut CollisionShape, &GlobalTransform), Changed<GlobalTransform>>,
) {
    // Iterate through all collision shapes and set transform accordingly
    for (mut shape, transform) in shapes.iter_mut() {
        shape.set_transform(*transform);
    }
}
