use std::fs;

/// Flatpak 沙箱内 `current_exe()` 返回 `/app/bin/bongo-cat`，宿主机无法执行。
/// 此时需要把 `Exec=` 写成 `flatpak run com.ayangweb.BongoCat`。
#[cfg(target_os = "linux")]
fn get_exec_path() -> Result<String, String> {
    if std::env::var("FLATPAK_ID").is_ok() {
        Ok("/usr/bin/flatpak run com.ayangweb.BongoCat".into())
    } else {
        std::env::current_exe()
            .map(|p| p.display().to_string())
            .map_err(|e| e.to_string())
    }
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn enable() -> Result<(), String> {
    let exec = get_exec_path()?;

    let data = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Version=1.0\n\
         Name=BongoCat\n\
         Comment=BongoCat startup script\n\
         Exec={}\n\
         StartupNotify=false\n\
         Terminal=false",
        exec
    );

    let dir = dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("autostart");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    fs::write(dir.join("com.ayangweb.BongoCat.desktop"), data).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn disable() -> Result<(), String> {
    let path = dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("autostart")
        .join("com.ayangweb.BongoCat.desktop");
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn is_enabled() -> Result<bool, String> {
    let path = dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("autostart")
        .join("com.ayangweb.BongoCat.desktop");
    Ok(path.exists())
}

// ── macOS ──────────────────────────────────────────────

#[cfg(target_os = "macos")]
static AUTO: std::sync::LazyLock<auto_launch::AutoLaunch> =
    std::sync::LazyLock::new(|| {
        let current_exe = std::env::current_exe().unwrap();
        let exe_path = current_exe.canonicalize().unwrap();
        let exe_str = exe_path.display().to_string();

        // macOS 上需要传 .app bundle 路径而不是内部可执行文件
        let app_path = exe_str
            .split(".app/")
            .next()
            .map(|p| format!("{}.app", p))
            .unwrap_or(exe_str);

        auto_launch::AutoLaunchBuilder::new()
            .set_app_name("BongoCat")
            .set_app_path(&app_path)
            .set_use_launch_agent(true)
            .build()
            .unwrap()
    });

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn enable() -> Result<(), String> {
    AUTO.enable().map_err(|e| e.to_string())
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn disable() -> Result<(), String> {
    AUTO.disable().map_err(|e| e.to_string())
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn is_enabled() -> Result<bool, String> {
    AUTO.is_enabled().map_err(|e| e.to_string())
}
