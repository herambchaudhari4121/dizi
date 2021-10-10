use std::path::Path;

use dizi_lib::error::DiziResult;

use crate::audio::PlayerStatus;
use crate::context::AppContext;

pub fn player_play(context: &mut AppContext, path: &Path) -> DiziResult<()> {
    context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .play(path)
}

pub fn player_pause(context: &mut AppContext) -> DiziResult<()> {
    context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .pause()
}

pub fn player_resume(context: &mut AppContext) -> DiziResult<()> {
    context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .resume()
}

pub fn player_toggle_play(context: &mut AppContext) -> DiziResult<PlayerStatus> {
    let status = context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .toggle_play()?;
    Ok(status)
}

pub fn player_get_len(context: &mut AppContext) -> DiziResult<usize> {
    let len = context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .len()?;
    Ok(len)
}

pub fn player_get_volume(context: &mut AppContext, amount: usize) -> DiziResult<f32> {
    let volume = context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .get_volume()?;
    Ok(volume)
}

pub fn player_volume_increase(context: &mut AppContext, amount: usize) -> DiziResult<usize> {
    let volume = context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .get_volume()?;

    let amount: f32 = amount as f32 / 100.0;
    let volume = if volume + amount > 1.0 {
        1.0
    } else {
        volume + amount
    };
    context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .set_volume(volume)?;
    eprintln!("volume is now: {}", volume);

    let volume: usize = (volume * 100.0) as usize;
    Ok(volume)
}

pub fn player_volume_decrease(context: &mut AppContext, amount: usize) -> DiziResult<usize> {
    let volume = context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .get_volume()?;

    let amount: f32 = amount as f32 / 100.0;
    let volume = if volume - amount < 0.0 {
        0.0
    } else {
        volume - amount
    };
    context
        .player_context_mut()
        .player_mut()
        .lock()
        .unwrap()
        .set_volume(volume)?;
    eprintln!("volume is now: {}", volume);

    let volume: usize = (volume * 100.0) as usize;
    Ok(volume)
}
