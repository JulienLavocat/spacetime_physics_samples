use spacetime_physics::{
    math::{Quat, Vec3},
    KinematicBody,
};
use spacetimedb::{table, Identity, ReducerContext, Table};

#[table(name = players)]
pub struct Player {
    #[primary_key]
    pub id: Identity,
    pub name: String,
    pub health: f32,
    pub score: i32,
    pub position: Vec3,
    pub looking_at: Quat,
    #[unique]
    pub rigid_body_id: u64,
    pub raycast_id: u64,
}

impl Player {
    pub fn new(id: Identity, name: String, rigid_body_id: u64, raycast_id: u64) -> Self {
        Self {
            id,
            name,
            rigid_body_id,
            raycast_id,
            health: 100.0,
            score: 0,
            position: Vec3::ZERO,
            looking_at: Quat::IDENTITY,
        }
    }

    pub fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.players().insert(self)
    }

    pub fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.players().id().update(self)
    }

    pub fn find(ctx: &ReducerContext, id: Identity) -> Option<Self> {
        ctx.db.players().id().find(id)
    }

    pub fn find_by_rigid_body(ctx: &ReducerContext, rigid_body_id: u64) -> Option<Self> {
        ctx.db.players().rigid_body_id().find(rigid_body_id)
    }

    pub fn collect_kinematic_update<'a>(
        ctx: &'a ReducerContext,
    ) -> impl Iterator<Item = KinematicBody> + 'a {
        ctx.db
            .players()
            .iter()
            .map(|player| (player.rigid_body_id, (player.position, player.looking_at)))
    }

    pub fn delete(ctx: &ReducerContext, id: Identity) {
        ctx.db.players().id().delete(id);
    }
}
