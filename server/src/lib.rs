use spacetime_physics::{
    math::{Quat, Vec3},
    step_world, PhysicsWorld, RayCast, RigidBody,
};
use spacetimedb::{reducer, ReducerContext};
use tables::{PhysicsWorldTick, Player};
use utils::compute_damage_falloff;

mod lifecycle;
mod tables;
mod utils;

const PLAYER_RB_PROPERTIES: u64 = 1;
const PLAYER_RB_COLLIDER: u64 = 1;
const MAIN_WORLD_ID: u64 = 1;

#[reducer]
pub fn physics_tick(ctx: &ReducerContext, tick: PhysicsWorldTick) {
    let world = PhysicsWorld::find(ctx, tick.world_id).unwrap();
    let kinematic_bodies = Player::collect_kinematic_update(ctx);
    step_world(ctx, &world, kinematic_bodies);
}

#[reducer]
pub fn player_spawn(ctx: &ReducerContext) {
    let rigid_body = RigidBody::builder()
        .collider_id(PLAYER_RB_COLLIDER)
        .properties_id(PLAYER_RB_PROPERTIES)
        .build()
        .insert(ctx);

    let raycast = RayCast::new(
        MAIN_WORLD_ID,
        Vec3::new(0.0, 0.8, 0.0),
        Vec3::Z,
        100.0,
        false,
    )
    .insert(ctx);

    Player::new(
        ctx.sender,
        ctx.sender.to_abbreviated_hex().to_string(),
        rigid_body.id,
        raycast.id,
    )
    .insert(ctx);
}

#[reducer]
pub fn player_move(ctx: &ReducerContext, position: Vec3, looking_at: Quat) {
    let mut player = Player::find(ctx, ctx.sender).unwrap();
    let mut ray = RayCast::find(ctx, player.raycast_id).unwrap();

    player.position = position;
    player.looking_at = looking_at;

    ray.origin = position + Vec3::new(0.0, 0.8, 0.0); // Adjust ray origin to be at eye level
    ray.direction = looking_at.forward_by(Vec3::Z); // Forward direction based on the player's looking direction

    player.update(ctx);
}

#[reducer]
pub fn player_shoot(ctx: &ReducerContext) {
    let player = Player::find(ctx, ctx.sender).unwrap();
    let ray = RayCast::find(ctx, player.raycast_id).unwrap();

    for hit in ray.hits {
        let target = Player::find_by_rigid_body(ctx, hit.rigid_body_id).unwrap();
        let damage = compute_damage_falloff(hit.distance, 10.0, 100.0);
        if target.id != player.id {
            let mut target = target;
            target.health -= damage;

            if target.health <= 0.0 {
                Player::delete(ctx, target.id);
            } else {
                target.update(ctx);
            }
        }
    }
}
