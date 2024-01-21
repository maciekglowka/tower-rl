use rogalik::math::vectors::Vector2f;

pub fn move_towards(origin: Vector2f, target: Vector2f, max_delta: f32) -> Vector2f {
    let a = target - origin;
    let l = a.len();
    if l <= max_delta || l == 0. {
        return target
    }
    origin + a / l * max_delta
}
