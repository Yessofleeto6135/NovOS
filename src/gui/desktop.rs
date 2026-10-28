use crate::drivers::input::MOUSE;
use crate::gui::compositor::{render_frame, WINDOWS};
use crate::gui::window::Window;
use spin::Mutex;

pub static LAUNCHER_OPEN: Mutex<bool> = Mutex::new(false);
pub static ACTIVE_APP_TITLE: Mutex<&'static str> = Mutex::new("NovOS Desktop");

pub fn launch_novos_desktop() {
    // Open default desktop windows
    let mut wins = WINDOWS.lock();
    wins.push(Window::new(1, "NovOS Files Manager", 50, 60, 600, 420, 0xFF181825));
    wins.push(Window::new(2, "Settings", 200, 100, 550, 400, 0xFF1E1E2E));
}

pub fn update_desktop_loop() {
    let mouse = MOUSE.lock();
    let launcher_open = *LAUNCHER_OPEN.lock();
    let active_title = *ACTIVE_APP_TITLE.lock();

    render_frame(launcher_open, active_title, mouse.x, mouse.y);
}
