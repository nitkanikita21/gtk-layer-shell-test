

use anyhow::Result;
use mpris::{DBusError, Metadata, PlaybackStatus, PlayerFinder, Player, LoopStatus};
use serde::Serialize;
use tauri::command;
use ts_rs::TS;
use std::time::Duration;

#[derive(Serialize, TS)]
#[ts(export)]
pub struct MediaInfo {
  pub title: Option<String>,
  pub artist: Option<String>,
  pub album: Option<String>,
  pub status: String,
  pub position: i64,
  pub length: Option<i64>,
  pub can_play: bool,
  pub can_pause: bool,
  pub can_seek: bool,
}

#[command]
pub async fn get_media_info() -> Result<MediaInfo, String> {
  let player_finder = PlayerFinder::new().map_err(|e| format!("Failed to create player finder: {}", e))?;
  
  let players = player_finder.find_all().map_err(|e| format!("Failed to find players: {}", e))?;
  
  if players.is_empty() {
    return Err("No media players found".to_string());
  }

  // Get the first available player
  let player = players.first().unwrap();
  
  let metadata = player.get_metadata().map_err(|e| format!("Failed to get metadata: {}", e))?;
  let status = player.get_playback_status().map_err(|e| format!("Failed to get playback status: {}", e))?;
  let position = player.get_position().map_err(|e| format!("Failed to get position: {}", e))?.as_millis() as i64;
  let can_play = player.can_play().map_err(|e| format!("Failed to get can_play: {}", e))?;
  let can_pause = player.can_pause().map_err(|e| format!("Failed to get can_pause: {}", e))?;
  let can_seek = player.can_seek().map_err(|e| format!("Failed to get can_seek: {}", e))?;

  Ok(MediaInfo {
    title: get_title(&metadata),
    artist: get_artist(&metadata),
    album: get_album(&metadata),
    status: format!("{:?}", status),
    position,
    length: get_length(&metadata),
    can_play,
    can_pause,
    can_seek,
  })
}

#[command]
pub async fn media_play_pause() -> Result<(), String> {
  let player_finder = PlayerFinder::new().map_err(|e| format!("Failed to create player finder: {}", e))?;
  let players = player_finder.find_all().map_err(|e| format!("Failed to find players: {}", e))?;
  
  if let Some(player) = players.first() {
    player.play_pause().map_err(|e| format!("Failed to play/pause: {}", e))?;
  }
  
  Ok(())
}

#[command]
pub async fn media_toggle_play_pause() -> Result<(), String> {
  let player_finder = PlayerFinder::new().map_err(|e| format!("Failed to create player finder: {}", e))?;
  let players = player_finder.find_all().map_err(|e| format!("Failed to find players: {}", e))?;
  
  if let Some(player) = players.first() {
    player.play_pause().map_err(|e| format!("Failed to toggle play/pause: {}", e))?;
  }
  
  Ok(())
}

#[command]
pub async fn media_next() -> Result<(), String> {
  let player_finder = PlayerFinder::new().map_err(|e| format!("Failed to create player finder: {}", e))?;
  let players = player_finder.find_all().map_err(|e| format!("Failed to find players: {}", e))?;
  
  if let Some(player) = players.first() {
    player.next().map_err(|e| format!("Failed to go to next track: {}", e))?;
  }
  
  Ok(())
}

#[command]
pub async fn media_previous() -> Result<(), String> {
  let player_finder = PlayerFinder::new().map_err(|e| format!("Failed to create player finder: {}", e))?;
  let players = player_finder.find_all().map_err(|e| format!("Failed to find players: {}", e))?;
  
  if let Some(player) = players.first() {
    player.previous().map_err(|e| format!("Failed to go to previous track: {}", e))?;
  }
  
  Ok(())
}

#[command]
pub async fn media_stop() -> Result<(), String> {
  let player_finder = PlayerFinder::new().map_err(|e| format!("Failed to create player finder: {}", e))?;
  let players = player_finder.find_all().map_err(|e| format!("Failed to find players: {}", e))?;
  
  if let Some(player) = players.first() {
    player.stop().map_err(|e| format!("Failed to stop: {}", e))?;
  }
  
  Ok(())
}


#[command]
pub async fn media_set_volume(volume: f64) -> Result<(), String> {
  let player_finder = PlayerFinder::new().map_err(|e| format!("Failed to create player finder: {}", e))?;
  let players = player_finder.find_all().map_err(|e| format!("Failed to find players: {}", e))?;
  
  if let Some(player) = players.first() {
    player.set_volume(volume).map_err(|e| format!("Failed to set volume: {}", e))?;
  }
  
  Ok(())
}

// Helper functions to extract metadata
fn get_title(metadata: &Metadata) -> Option<String> {
  metadata.title().map(|s| s.to_string())
}

fn get_artist(metadata: &Metadata) -> Option<String> {
  metadata.artists().and_then(|artists| artists.first().map(|s| s.to_string()))
}

fn get_album(metadata: &Metadata) -> Option<String> {
  metadata.album_name().map(|s| s.to_string())
}

fn get_length(metadata: &Metadata) -> Option<i64> {
  metadata.length().map(|d| d.as_millis() as i64)
}
