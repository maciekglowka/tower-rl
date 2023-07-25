use rogalik::math::vectors::Vector2F;

pub fn move_towards(origin: Vector2F, target: Vector2F, max_delta: f32) -> Vector2F {
    let a = target - origin;
    let l = a.len();
    if l <= max_delta || l == 0. {
        return target
    }
    origin + a / l * max_delta
} 