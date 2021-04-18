use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::audio::{SoundData, SoundSource, Source};
use ggez::Context;

use crate::helpers::names::Names;

pub fn play_explosion_sound_system(world: &World, context: &mut Context) -> Result<()> {
    let mut need_to_play = world
        .get_resource(&Names::NeedToPlayExplosionSound.to_string())?
        .borrow_mut();
    let need_to_play: &mut bool = need_to_play.cast_mut()?;

    if !*need_to_play {
        return Ok(());
    }

    let explosion_sound_data = world
        .get_resource(&Names::ExplosionSound.to_string())?
        .borrow();
    let explosion_sound_data: &SoundData = explosion_sound_data.cast()?;
    let mut sound = Source::from_data(context, explosion_sound_data.clone())?;
    sound.set_volume(0.15);
    sound.set_pitch(2.0);
    sound.play_detached()?;
    *need_to_play = false;
    Ok(())
}
