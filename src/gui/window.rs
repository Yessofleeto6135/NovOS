use alloc::string::String;

#[derive(Clone, Debug)]
pub struct Window {
    pub id: usize,
    pub title: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub active: bool,
    pub bg_color: u32,
}

impl Window {
    pub fn new(id: usize, title: &str, x: usize, y: usize, w: usize, h: usize, bg_color: u32) -> Self {
        Window {
            id,
            title: String::from(title),
            x,
            y,
            width: w,
            height: h,
            is_minimized: false,
            is_maximized: false,
            active: true,
            bg_color,
        }
    }
}
