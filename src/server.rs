use ambient_api::{
    core::{
        app::components::main_scene,
        camera::concepts::make_orthographic_camera,
        primitives::{
            components::{cube, quad, sphere_radius},
            concepts::make_sphere,
        },
        rendering::components::color,
        transform::{
            components::{scale, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use embers::fish_demo::{components::*, messages::*};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_orthographic_camera())
        .with_default(main_scene())
        .spawn();

    for _ in 0..10 {
        Entity::new()
            .with_merge(make_transformable())
            .with_default(fish())
            .with(position(), random::<Vec2>() * 2.0 - 1.0)
            .with_default(cube())
            .with(size(), vec2(0.1, 0.05))
            .with(speed(), 0.2)
            .with(color(), vec4(0.0, 0.0, 1.0, 1.0))
            .spawn();
    }

    query((speed(), position()))
        .requires(fish())
        .each_frame(move |entities| {
            for (e, (speed, old_position)) in entities {
                let mut new_position = old_position + Vec2::X * speed * delta_time();

                if new_position.x > 1.1 {
                    new_position.x -= 2.2;
                    new_position.y = random::<f32>() * 2.0 - 1.0;
                }

                if new_position.x < -1.1 {
                    new_position.x += 2.2;
                    new_position.y = random::<f32>() * 2.0 - 1.0;
                }

                entity::set_component(e, position(), new_position);
            }
        });

    let pellets = query(position()).requires(pellet()).build();
    query((position(), size()))
        .requires(fish())
        .each_frame(move |entities| {
            let pellets = pellets.evaluate();
            for (fish, (position, fish_size)) in entities {
                for (pellet, pellet_position) in pellets.iter() {
                    let distance = (*pellet_position - position).abs() * 2.0;
                    if distance.x < fish_size.x && distance.y < fish_size.y {
                        if entity::exists(*pellet) {
                            entity::despawn_recursive(*pellet);

                            entity::mutate_component(fish, size(), |size| {
                                *size += 0.02;
                            });
                        }
                    }
                }
            }
        });

    query(size()).each_frame(move |entities| {
        for (e, size) in entities {
            let new_scale = size.extend(1.0);
            entity::add_component(e, scale(), new_scale);
        }
    });

    query(position()).each_frame(move |entities| {
        for (e, position) in entities {
            let new_translation = position.extend(0.0);
            entity::add_component(e, translation(), new_translation);
        }
    });

    SpawnPellet::subscribe(move |_source, data| {
        Entity::new()
            .with_merge(make_transformable())
            .with_merge(make_sphere())
            .with(sphere_radius(), 0.02)
            .with_default(pellet())
            .with(position(), data.at)
            .with(translation(), data.at.extend(0.0))
            .spawn();
    });
}
