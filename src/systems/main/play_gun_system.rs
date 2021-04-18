use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::audio::SoundSource;
use ggez::Context;

use crate::helpers::names::Names;

pub fn play_gun_system(world: &World, context: &mut Context) -> Result<()> {
    let mut need_to_fire = world
        .get_resource(&Names::NeedToPlayGunSound.to_string())?
        .borrow_mut();
    let need_to_fire: &mut bool = need_to_fire.cast_mut()?;

    if !*need_to_fire {
        return Ok(());
    }

    let gun_sound = world.get_resource(&Names::GunSound.to_string())?.borrow();
    let gun_sound: &ggez::audio::SoundData = gun_sound.cast()?;
    let mut sound = ggez::audio::Source::from_data(context, gun_sound.clone())?;
    sound.set_volume(0.15);
    sound.play_detached()?;

    *need_to_fire = false;
    Ok(())
}
