use ambient_api::{
    core::{primitives::components::cube, transform::concepts::make_transformable, rendering::components::color},
    prelude::*,
};

use embers::fish_demo::components::*;

#[main]
fn main() {
    for _ in 0..10 {
        Entity::new()
            .with_merge(make_transformable())
            .with_default(fish())
            .with(position(), random::<Vec2>() * 2.0 - 1.0)
            .with_default(cube())
            .with(size(), vec2(0.1, 0.05))
            .with(speed(), -0.3)
            .with(color(), vec4(1.0, 0.0, 0.0, 1.0))
            .spawn();
    }
}
