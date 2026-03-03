use gtk_layer_shell_test_lib::modules::{media::MediaInfo, system::SystemInfo, workspaces::WorkspaceInfo};
use ts_rs::{Config, TS};

fn main() -> anyhow::Result<()> {
    let cfg = Config::from_env();
    SystemInfo::export_all(&cfg)?;
    MediaInfo::export_all(&cfg)?;
    WorkspaceInfo::export_all(&cfg)?;

    Ok(())
}