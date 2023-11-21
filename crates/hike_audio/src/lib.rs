use fyrox_sound::{
    buffer::{DataSource, SoundBufferResource, SoundBufferResourceExtension},
    context::SoundContext,
    engine::SoundEngine,
    pool::Handle,
    source::{SoundSource, SoundSourceBuilder, Status}
};
use rogalik::{
    events::{EventBus, SubscriberHandle},
    storage::World
};
use std::collections::HashMap;

use hike_game::{GameEvent, get_player_entity};

pub fn handle_game_audio(context: &mut AudioContext, world: &World) {
    for ev in context.ev_game.read().iter().flatten() {
        match ev {
            GameEvent::PickInstant => {
                context.play("coin");
            },
            GameEvent::Spawn => {
                context.play("spawn");
            }
            GameEvent::Travel(entity, is_animated) => {
                if !is_animated {
                    context.play("teleport");
                }
            //     if Some(*entity) == get_player_entity(world) {
            //         context.play("walk");
            //     }
            },
            GameEvent::Attack(_, _) | GameEvent::HitProjectile(_) => {
                context.play("hit");
            },
            _ => continue
        }
    }
}

pub struct AudioContext {
    inner: Option<SoundContext>,
    sounds: HashMap<&'static str, Handle<SoundSource>>,
    ev_game: SubscriberHandle<GameEvent>,
}
impl AudioContext {
    fn play(&mut self, sound: &str) {
        let Some(handle) = self.sounds.get(sound) else { return };
        if let Some(context) = self.inner.as_mut() {
            let mut state = context.state();
            let source = state.source_mut(*handle);
            let _ = source.stop();
            source.play();
        }
    }
}

pub fn get_audio_context(events: &mut EventBus<GameEvent>) -> AudioContext {
    let ev_game = events.subscribe();
    let Ok(engine) = SoundEngine::new() else {
        return AudioContext {
            inner: None,
            sounds: HashMap::new(),
            ev_game
        }
    };
    let context = SoundContext::new();
    engine.state().add_context(context.clone());

    let mut data = HashMap::new();
    data.insert("coin", include_bytes!("../../../assets/sfx/coin.wav").to_vec());
    data.insert("hit", include_bytes!("../../../assets/sfx/hit.wav").to_vec());
    data.insert("spawn", include_bytes!("../../../assets/sfx/spawn.wav").to_vec());
    data.insert("teleport", include_bytes!("../../../assets/sfx/teleport.wav").to_vec());

    let mut sounds = HashMap::new();

    for (k, v) in data.iter() {
        let buffer = SoundBufferResource::new_generic(
                DataSource::from_memory(v.to_vec())
            )
            .expect(&format!("Can't build audio buffer for {}!", k));
        let source = SoundSourceBuilder::new()
            .with_buffer(buffer)
            .build()
            .expect(&format!("Can't build audio source for {}!", k));
        let handle = context.state().add_source(source);
        sounds.insert(*k, handle);
    }

    AudioContext { inner: Some(context), sounds, ev_game }
}