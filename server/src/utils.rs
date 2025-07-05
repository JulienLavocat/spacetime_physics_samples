pub fn compute_damage_falloff(distance: f32, min_damage: f32, max_damage: f32) -> f32 {
    if distance <= 0.0 {
        return max_damage;
    }

    let falloff = (1.0 - (distance / 100.0)).max(0.0);
    let damage = min_damage + (falloff * (max_damage - min_damage));

    damage.clamp(min_damage, max_damage)
}
