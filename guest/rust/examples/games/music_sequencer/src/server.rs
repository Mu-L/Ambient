use ambient_api::prelude::*;
use common::BEAT_COUNT;
use components::{next_player_hue, player_hue, track, track_audio_url, track_note_selection};

use ambient_api::{
    components::core::player::{player, user_id},
    entity::resources,
};

mod common;

const TRACKS: [(&str, &str); 8] = [
    ("Kick Drum", "assets/BD2500.wav"),
    ("Snare Drum", "assets/SD7550.wav"),
    ("Closed Hihat", "assets/CH.wav"),
    ("Open Hihat", "assets/OH75.wav"),
    ("Low Conga", "assets/LC00.wav"),
    ("Mid Conga", "assets/MC00.wav"),
    ("High Tom", "assets/HT75.wav"),
    ("Mid Tom", "assets/MT75.wav"),
];

#[main]
pub async fn main() {
    entity::add_component(resources(), next_player_hue(), 0);

    // Create the tracks.
    for (idx, (track_name, track_url)) in TRACKS.iter().enumerate() {
        Entity::new()
            .with(name(), track_name.to_string())
            .with(track(), idx as u32)
            .with(track_audio_url(), track_url.to_string())
            .with(track_note_selection(), vec![0; BEAT_COUNT])
            .spawn();
    }

    // When a player spawns, create their player state.
    spawn_query(user_id())
        .requires(player())
        .bind(move |players| {
            for (player, _player_user_id) in players {
                let mut h = entity::get_component(resources(), next_player_hue()).unwrap();
                h = (h + 103) % 360;
                entity::add_component(player, player_hue(), h);
                entity::set_component(resources(), next_player_hue(), h);
            }
        });

    // When a player clicks on a note, toggle it.
    messages::Click::subscribe(move |source, data| {
        let id = source.client_entity_id().unwrap();
        let color_to_set = entity::get_component(id, player_hue()).unwrap();

        entity::mutate_component(data.track_id, track_note_selection(), |selection| {
            if selection[data.index as usize] == color_to_set {
                selection[data.index as usize] = 0;
            } else {
                selection[data.index as usize] = color_to_set;
            }
        });
    });
}
