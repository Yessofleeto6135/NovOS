use crate::drivers::display::{draw_pixel, draw_rect, swap_buffers, WIDTH, HEIGHT};
use crate::gui::window::Window;
use alloc::vec::Vec;
use spin::Mutex;

pub static WINDOWS: Mutex<Vec<Window>> = Mutex::new(Vec::new());

// Simple 8x8 font rendering
static FONT_BASIC: [[u8; 8]; 128] = [[0u8; 8]; 128];

pub fn draw_text(x: usize, y: usize, text: &str, color: u32) {
    let mut cx = x;
    for ch in text.bytes() {
        if cx + 8 >= WIDTH { break; }
        draw_char(cx, y, ch, color);
        cx += 8;
    }
}

fn draw_char(x: usize, y: usize, ch: u8, color: u32) {
    let glyph = FONT_BASIC.get(ch as usize).cloned().unwrap_or([0xFF; 8]);
    for row in 0..8 {
        let line = glyph[row];
        for col in 0..8 {
            if (line & (1 << (7 - col))) != 0 || ch.is_ascii_graphic() || ch == b' ' {
                draw_pixel(x + col, y + row, color);
            }
        }
    }
}

pub fn render_frame(launcher_open: bool, active_app_title: &str, mouse_x: usize, mouse_y: usize) {
    // 1. Render Modern ChromeOS Wallpaper Gradient
    for y in 0..HEIGHT {
        let r = ((y * 40) / HEIGHT) as u32;
        let g = ((y * 60) / HEIGHT + 20) as u32;
        let b = ((y * 120) / HEIGHT + 80) as u32;
        let color = 0xFF000000 | (r << 16) | (g << 8) | b;
        draw_rect(0, y, WIDTH, 1, color);
    }

    // 2. Render Windows
    let wins = WINDOWS.lock();
    for win in wins.iter() {
        if win.is_minimized { continue; }

        let (wx, wy, ww, wh) = if win.is_maximized {
            (0, 0, WIDTH, HEIGHT - 48)
        } else {
            (win.x, win.y, win.width, win.height)
        };

        // Window Shadow & Border
        draw_rect(wx.saturating_sub(2), wy.saturating_sub(2), ww + 4, wh + 34, 0xAA000000);
        // Window Titlebar
        let title_color = if win.active { 0xFF282C34 } else { 0xFF1E222B };
        draw_rect(wx, wy, ww, 30, title_color);
        draw_text(wx + 12, wy + 10, &win.title, 0xFFFFFFFF);

        // Window Controls
        draw_rect(wx + ww - 24, wy + 7, 16, 16, 0xFFEF4444); // Close
        draw_rect(wx + ww - 48, wy + 7, 16, 16, 0xFFF59E0B); // Maximize
        draw_rect(wx + ww - 72, wy + 7, 16, 16, 0xFF10B981); // Minimize

        // Window Body
        draw_rect(wx, wy + 30, ww, wh, win.bg_color);
    }

    // 3. Render ChromeOS Bottom Shelf / Dock
    let shelf_y = HEIGHT - 48;
    draw_rect(0, shelf_y, WIDTH, 48, 0xEE1E1E24);
    draw_rect(0, shelf_y, WIDTH, 1, 0x44FFFFFF);

    let launcher_bg = if launcher_open { 0xFF3B82F6 } else { 0xFF2D3748 };
    draw_rect(12, shelf_y + 8, 32, 32, launcher_bg);
    draw_text(22, shelf_y + 18, "O", 0xFFFFFFFF);

    let icons = [
        ("Settings", 0xFF3B82F6),
        ("Commander", 0xFF10B981),
        ("Browser", 0xFFEF4444),
        ("Files", 0xFFF59E0B),
        ("CraftBlocks", 0xFF8B5CF6),
        ("TaskMgr", 0xFF6366F1),
    ];

    for (i, (icon_name, icon_color)) in icons.iter().enumerate() {
        let ix = 60 + (i * 44);
        draw_rect(ix, shelf_y + 8, 36, 32, *icon_color);
        draw_text(ix + 6, shelf_y + 18, &icon_name[0..1], 0xFFFFFFFF);
    }

    draw_text(350, shelf_y + 18, active_app_title, 0xFF9CA3AF);

    let tray_x = WIDTH - 260;
    draw_rect(tray_x, shelf_y + 8, 248, 32, 0xFF2D3748);
    draw_text(tray_x + 10, shelf_y + 18, "WiFi 88% | BT | 92% [12:00]", 0xFFE2E8F0);

    // 4. Render ChromeOS App Launcher Drawer
    if launcher_open {
        let drawer_w = 400;
        let drawer_h = 450;
        let drawer_x = 12;
        let drawer_y = shelf_y - drawer_h - 8;

        draw_rect(drawer_x, drawer_y, drawer_w, drawer_h, 0xF8181825);
        draw_rect(drawer_x + 16, drawer_y + 16, drawer_w - 32, 32, 0xFF2A2B3D);
        draw_text(drawer_x + 28, drawer_y + 26, "Search apps, files, web...", 0xFF9CA3AF);

        for (idx, (app_name, app_color)) in icons.iter().enumerate() {
            let row = idx / 3;
            let col = idx % 3;
            let ax = drawer_x + 28 + (col * 120);
            let ay = drawer_y + 70 + (row * 100);

            draw_rect(ax, ay, 80, 60, *app_color);
            draw_text(ax + 10, ay + 24, app_name, 0xFFFFFFFF);
        }
    }

    // 5. Render Mouse Cursor
    draw_rect(mouse_x, mouse_y, 10, 10, 0xFF00FFFF);
    draw_rect(mouse_x + 2, mouse_y + 2, 6, 6, 0xFFFFFFFF);

    swap_buffers();
}
