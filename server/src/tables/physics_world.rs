use spacetimedb::{table, ScheduleAt};

use crate::physics_tick;

#[table(name = physics_world_ticks, scheduled(physics_tick))]
pub struct PhysicsWorldTick {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub world_id: u64,
    pub scheduled_at: ScheduleAt,
}
