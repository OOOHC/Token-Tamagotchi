use serde::Serialize;
use tauri::{PhysicalPosition, PhysicalSize, Position, Size};

const SCREEN_MARGIN: i32 = 8;
const BOTTOM_RESERVED_SPACE: i32 = 56;
const BIT_VISIBLE_LEFT: i32 = 112;
const BIT_VISIBLE_RIGHT: i32 = 190;
const BIT_VISIBLE_TOP: i32 = 112;
const BIT_VISIBLE_BOTTOM: i32 = 96;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayoutState {
    pub panel_above: bool,
    pub dock_edge: Option<String>,
}

#[tauri::command]
pub fn start_window_drag(window: tauri::Window) -> Result<(), String> {
    window.start_dragging().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_window_layout(
    window: tauri::Window,
    layout: String,
    previous_layout: Option<String>,
    previous_panel_above: Option<bool>,
) -> Result<WindowLayoutState, String> {
    let size = layout_size(layout.as_str())?;
    let panel_above = should_place_panel_above(&window, layout.as_str())?;
    let old_position = window.outer_position().map_err(|error| error.to_string())?;
    let old_size = window.outer_size().map_err(|error| error.to_string())?;
    let previous_layout = previous_layout.unwrap_or_else(|| layout.clone());
    let previous_panel_above = previous_panel_above.unwrap_or(panel_above);
    let old_anchor = companion_anchor(
        previous_layout.as_str(),
        previous_panel_above,
        old_size.width,
        old_size.height,
    );
    let new_anchor = companion_anchor(layout.as_str(), panel_above, size.width, size.height);
    let companion_screen_position = PhysicalPosition {
        x: old_position.x + old_anchor.x,
        y: old_position.y + old_anchor.y,
    };

    window
        .set_size(Size::Physical(size))
        .map_err(|error| error.to_string())?;
    window
        .set_position(Position::Physical(PhysicalPosition {
            x: companion_screen_position.x - new_anchor.x,
            y: companion_screen_position.y - new_anchor.y,
        }))
        .map_err(|error| error.to_string())?;
    let dock_edge = constrain_window_to_monitor(&window, layout.as_str(), panel_above)?;

    Ok(WindowLayoutState {
        panel_above,
        dock_edge,
    })
}

#[tauri::command]
pub fn constrain_window_to_screen(
    window: tauri::Window,
    layout: Option<String>,
) -> Result<WindowLayoutState, String> {
    let layout = layout.unwrap_or_else(|| "compact".to_string());
    let panel_above = should_place_panel_above(&window, layout.as_str())?;
    let dock_edge = constrain_window_to_monitor(&window, layout.as_str(), panel_above)?;
    Ok(WindowLayoutState {
        panel_above,
        dock_edge,
    })
}

#[tauri::command]
pub fn minimize_window(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn close_window(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|error| error.to_string())
}

fn constrain_window_to_monitor(
    window: &tauri::Window,
    layout: &str,
    panel_above: bool,
) -> Result<Option<String>, String> {
    let Some(monitor) = window
        .current_monitor()
        .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };

    let monitor_position = monitor.position();
    let monitor_size = monitor.size();
    let window_position = window.outer_position().map_err(|error| error.to_string())?;
    let window_size = window.outer_size().map_err(|error| error.to_string())?;

    let window_min_x = monitor_position.x + SCREEN_MARGIN;
    let window_min_y = monitor_position.y + SCREEN_MARGIN;
    let window_max_x =
        monitor_position.x + monitor_size.width as i32 - window_size.width as i32 - SCREEN_MARGIN;
    let window_max_y = monitor_position.y + monitor_size.height as i32
        - window_size.height as i32
        - BOTTOM_RESERVED_SPACE;
    let anchor = companion_anchor(layout, panel_above, window_size.width, window_size.height);
    let visible_min_x = monitor_position.x + SCREEN_MARGIN - anchor.x + BIT_VISIBLE_LEFT;
    let visible_max_x = monitor_position.x + monitor_size.width as i32
        - SCREEN_MARGIN
        - anchor.x
        - BIT_VISIBLE_RIGHT;
    let visible_min_y = monitor_position.y + SCREEN_MARGIN - anchor.y + BIT_VISIBLE_TOP;
    let visible_max_y = monitor_position.y + monitor_size.height as i32
        - BOTTOM_RESERVED_SPACE
        - anchor.y
        - BIT_VISIBLE_BOTTOM;

    let min_x = window_min_x.max(visible_min_x);
    let max_x = window_max_x.min(visible_max_x);
    let min_y = window_min_y.max(visible_min_y);
    let max_y = window_max_y.min(visible_max_y);

    let mut dock_edge = None;
    let mut clamped_x = window_position.x.clamp(min_x, max_x.max(min_x));
    let clamped_y = window_position.y.clamp(min_y, max_y.max(min_y));

    if layout == "compact" {
        if window_position.x <= min_x + 18 {
            clamped_x = monitor_position.x - 80;
            dock_edge = Some("left".to_string());
        } else if window_position.x >= max_x - 18 {
            clamped_x =
                monitor_position.x + monitor_size.width as i32 - window_size.width as i32 + 80;
            dock_edge = Some("right".to_string());
        }
    }

    if clamped_x == window_position.x && clamped_y == window_position.y {
        return Ok(dock_edge);
    }

    window
        .set_position(Position::Physical(PhysicalPosition {
            x: clamped_x,
            y: clamped_y,
        }))
        .map_err(|error| error.to_string())?;

    Ok(dock_edge)
}

fn should_place_panel_above(window: &tauri::Window, layout: &str) -> Result<bool, String> {
    if layout == "compact" {
        return Ok(false);
    }

    let Some(monitor) = window
        .current_monitor()
        .map_err(|error| error.to_string())?
    else {
        return Ok(false);
    };

    let monitor_position = monitor.position();
    let monitor_size = monitor.size();
    let window_position = window.outer_position().map_err(|error| error.to_string())?;
    let window_size = window.outer_size().map_err(|error| error.to_string())?;
    let window_center_y = window_position.y + window_size.height as i32 / 2;
    let monitor_center_y = monitor_position.y + monitor_size.height as i32 / 2;

    Ok(window_center_y >= monitor_center_y)
}

fn layout_size(layout: &str) -> Result<PhysicalSize<u32>, String> {
    match layout {
        "compact" => Ok(PhysicalSize {
            width: 320,
            height: 300,
        }),
        "body" => Ok(PhysicalSize {
            width: 320,
            height: 390,
        }),
        "food" => Ok(PhysicalSize {
            width: 390,
            height: 640,
        }),
        "details" => Ok(PhysicalSize {
            width: 460,
            height: 820,
        }),
        _ => Err(format!("unknown window layout: {layout}")),
    }
}

fn companion_anchor(
    layout: &str,
    panel_above: bool,
    window_width: u32,
    window_height: u32,
) -> PhysicalPosition<i32> {
    let x = window_width as i32 / 2;

    if layout == "compact" {
        return PhysicalPosition { x, y: 150 };
    }

    if layout == "body" {
        return PhysicalPosition { x, y: 196 };
    }

    if !panel_above {
        return PhysicalPosition { x, y: 108 };
    }

    PhysicalPosition {
        x,
        y: window_height as i32 - 104,
    }
}
