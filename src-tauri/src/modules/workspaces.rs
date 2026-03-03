use anyhow::Result;
use hyprland::data::{Clients, FullscreenMode, Workspace, Workspaces};
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::prelude::*;
use serde::Serialize;
use tauri::command;
use ts_rs::TS;
use std::collections::HashMap;

// Інформація про робочий стіл для фронтенду
#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct WorkspaceInfo {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    pub windows: i32,
    pub fullscreen: bool,
    pub last_window: Option<String>,
    pub is_active: bool, // нове поле
}

impl From<Workspace> for WorkspaceInfo {
    fn from(w: Workspace) -> Self {
        Self {
            id: w.id,
            name: w.name,
            monitor: w.monitor,
            windows: w.windows as i32,
            fullscreen: w.fullscreen,
            last_window: Some(w.last_window.to_string()),
            is_active: false, // тимчасово, буде перевизначено в get_workspaces
        }
    }
}

// Інформація про клієнта (вікно)
#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct ClientInfo {
    pub address: String,
    pub mapped: bool,
    pub at: (i32, i32),
    pub size: (i32, i32),
    pub workspace: WorkspaceInfo,
    pub floating: bool,
    pub monitor: i32,
    pub class: String,
    pub title: String,
    pub pid: i32,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: bool,
    pub fullscreen_mode: u8,
    pub grouped: Vec<String>,
    pub swallowing: Option<String>,
    pub focus_history_id: u32,
}

// Отримати список усіх робочих столів
#[command]
pub async fn get_workspaces() -> Result<Vec<WorkspaceInfo>, anyhow_serde::Error> {
    let workspaces = Workspaces::get_async().await?;
    let active = Workspace::get_active_async().await?;

    Ok(workspaces
        .into_iter()
        .map(|w| {
            let mut info = WorkspaceInfo::from(w);
            info.is_active = info.id == active.id;
            info
        })
        .collect())
}

// Отримати активний робочий стіл
#[command]
pub async fn get_active_workspace() -> Result<WorkspaceInfo, anyhow_serde::Error> {
    let active = Workspace::get_active_async().await?;
    let mut info = WorkspaceInfo::from(active);
    info.is_active = true;
    Ok(info)
}

// Переключитися на робочий стіл за ID
#[command]
pub async fn switch_to_workspace(id: i32) -> Result<(), anyhow_serde::Error> {
    Dispatch::call_async(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(id)))
        .await?;
    Ok(())
}

// Перемістити активне вікно на вказаний робочий стіл
#[command]
pub async fn move_window_to_workspace(id: i32) -> Result<(), anyhow_serde::Error> {
    Dispatch::call_async(DispatchType::MoveToWorkspace(
        WorkspaceIdentifierWithSpecial::Id(id),
        None, // None означає активне вікно
    ))
    .await?;
    Ok(())
}

// Отримати список усіх вікон з повною інформацією про робочі столи
#[command]
pub async fn get_clients() -> Result<Vec<ClientInfo>, anyhow_serde::Error> {
    // Отримуємо всіх клієнтів
    let clients = Clients::get_async().await?;

    // Отримуємо всі воркспейси для створення мапи
    let workspaces = Workspaces::get_async().await?;
    let active = Workspace::get_active_async().await?; // також потрібно для is_active

    // Мапа ID воркспейсу -> повна інформація
    let workspace_map: HashMap<i32, WorkspaceInfo> = workspaces
        .into_iter()
        .map(|w| {
            let mut info = WorkspaceInfo::from(w);
            info.is_active = info.id == active.id;
            (info.id, info)
        })
        .collect();

    // Конвертуємо клієнтів
    let clients_info: Vec<ClientInfo> = clients
        .into_iter()
        .map(|c| {
            let workspace_info = workspace_map
                .get(&c.workspace.id)
                .cloned()
                .unwrap_or_else(|| {
                    // Якщо воркспейс не знайдено в мапі (наприклад, новий або видалений)
                    WorkspaceInfo {
                        id: c.workspace.id,
                        name: c.workspace.name,
                        monitor: "unknown".to_string(),
                        windows: 0,
                        fullscreen: false,
                        last_window: None,
                        is_active: false, // не може бути активним, якщо його немає в списку
                    }
                });

            ClientInfo {
                address: c.address.to_string(),
                mapped: c.mapped,
                at: (c.at.0 as i32, c.at.1 as i32),
                size: (c.size.0 as i32, c.size.1 as i32),
                workspace: workspace_info,
                floating: c.floating,
                monitor: c.monitor.expect("ERR") as i32,
                class: c.class,
                title: c.title,
                pid: c.pid as i32,
                xwayland: c.xwayland,
                pinned: c.pinned,
                fullscreen: c.fullscreen == FullscreenMode::Fullscreen,
                fullscreen_mode: c.fullscreen as u8,
                grouped: c.grouped.into_iter().map(|addr| addr.to_string()).collect(),
                swallowing: c.swallowing.map(|addr| addr.to_string()),
                focus_history_id: c.focus_history_id as u32,
            }
        })
        .collect();

    Ok(clients_info)
}