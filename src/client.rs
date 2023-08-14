use ambient_api::{
    core::{app::components::window_logical_size, messages::WindowMouseInput},
    prelude::*,
};

use embers::fish_demo::messages::SpawnPellet;

#[main]
pub fn main() {
    let windows = query(window_logical_size()).build();
    WindowMouseInput::subscribe(move |ev| {
        if ev.pressed {
            let (_e, size) = windows.evaluate().into_iter().next().unwrap();
            let mut at = (input::get().mouse_position.xy() / size.as_vec2()) * 2.0 - 1.0;
            at.y = -at.y;
            SpawnPellet { at }.send_server_unreliable();
        }
    });
}
