use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::audio::{SoundData, SoundSource};
use ggez::Context;

use crate::helpers::names::Names;

pub fn play_asteroid_strategy_set_system(world: &World, context: &mut Context) -> Result<()> {
    let mut need_to_play = world
        .get_resource(&Names::NeedToPlayAsteroidStrategySet.to_string())?
        .borrow_mut();
    let need_to_play: &mut bool = need_to_play.cast_mut()?;

    if !*need_to_play {
        return Ok(());
    }

    let sound_data = world
        .get_resource(&Names::AsteroidStrategySetSound.to_string())?
        .borrow();
    let sound_data: &SoundData = sound_data.cast()?;
    let mut sound = ggez::audio::Source::from_data(context, sound_data.clone())?;
    sound.set_volume(0.25);
    sound.play_detached()?;
    *need_to_play = false;
    Ok(())
}
