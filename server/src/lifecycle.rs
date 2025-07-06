use spacetime_physics::{
    math::Vec3, schedule_physics_tick, Collider, PhysicsWorld, RigidBody, RigidBodyProperties,
};
use spacetimedb::{reducer, ReducerContext, Table};

use crate::{
    tables::{physics_world_ticks, PhysicsWorldTick, Player},
    PLAYER_RB_COLLIDER, PLAYER_RB_PROPERTIES,
};
#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    let world = PhysicsWorld::builder().build().insert(ctx);

    ctx.db.physics_world_ticks().insert(PhysicsWorldTick {
        id: 0,
        world_id: world.id,
        scheduled_at: schedule_physics_tick(&world),
    });

    // Shared rigid body properties for all players
    RigidBodyProperties::builder()
        .id(PLAYER_RB_PROPERTIES)
        .build()
        .insert(ctx);

    // Shared collider information for all players
    Collider::capsule(1, 0.5, 1.8)
        .id(PLAYER_RB_COLLIDER)
        .insert(ctx);

    // Floor
    let floor_collider_id = Collider::cuboid(world.id, Vec3::new(200.0, 0.1, 200.0))
        .insert(ctx)
        .id;
    RigidBody::builder()
        .collider_id(floor_collider_id)
        .properties_id(PLAYER_RB_PROPERTIES)
        .build()
        .insert(ctx);
}

#[reducer(client_disconnected)]
pub fn on_disconnect(ctx: &ReducerContext) {
    if let Some(player) = Player::find(ctx, ctx.sender) {
        Player::delete(ctx, player.id);
        RigidBody::delte_by_id(ctx, player.rigid_body_id);
    }
}
