use spin::Mutex;

#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    pub x: usize,
    pub y: usize,
    pub left_button: bool,
    pub right_button: bool,
}

pub static MOUSE: Mutex<MouseState> = Mutex::new(MouseState {
    x: 512,
    y: 384,
    left_button: false,
    right_button: false,
});

pub fn init_input_devices() {
    let mut mouse = MOUSE.lock();
    mouse.x = 512;
    mouse.y = 384;
}

pub fn update_mouse(dx: i32, dy: i32, left: bool, right: bool) {
    let mut mouse = MOUSE.lock();
    let new_x = (mouse.x as i32 + dx).clamp(0, 1023) as usize;
    let new_y = (mouse.y as i32 + dy).clamp(0, 767) as usize;
    mouse.x = new_x;
    mouse.y = new_y;
    mouse.left_button = left;
    mouse.right_button = right;
}
