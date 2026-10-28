use spin::Mutex;

pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 768;

static FRAMEBUFFER: Mutex<Option<FrameBuffer>> = Mutex::new(None);

pub struct FrameBuffer {
    pub ptr: *mut u32,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub back_buffer: [u32; WIDTH * HEIGHT],
}

unsafe impl Send for FrameBuffer {}

pub fn init_framebuffer() {
    static mut DISPLAY_BUF: [u32; WIDTH * HEIGHT] = [0xFF1C1B22; WIDTH * HEIGHT];
    static mut BACK_BUF: [u32; WIDTH * HEIGHT] = [0xFF1C1B22; WIDTH * HEIGHT];

    let fb = FrameBuffer {
        ptr: unsafe { DISPLAY_BUF.as_mut_ptr() },
        width: WIDTH,
        height: HEIGHT,
        pitch: WIDTH * 4,
        back_buffer: unsafe { BACK_BUF },
    };

    *FRAMEBUFFER.lock() = Some(fb);
}

pub fn draw_pixel(x: usize, y: usize, color: u32) {
    if x >= WIDTH || y >= HEIGHT { return; }
    let mut guard = FRAMEBUFFER.lock();
    if let Some(ref mut fb) = *guard {
        fb.back_buffer[y * WIDTH + x] = color;
    }
}

pub fn draw_rect(x: usize, y: usize, w: usize, h: usize, color: u32) {
    for cy in y..y.saturating_add(h).min(HEIGHT) {
        for cx in x..x.saturating_add(w).min(WIDTH) {
            draw_pixel(cx, cy, color);
        }
    }
}

pub fn swap_buffers() {
    let mut guard = FRAMEBUFFER.lock();
    if let Some(ref mut fb) = *guard {
        unsafe {
            core::ptr::copy_nonoverlapping(fb.back_buffer.as_ptr(), fb.ptr, WIDTH * HEIGHT);
        }
    }
}
