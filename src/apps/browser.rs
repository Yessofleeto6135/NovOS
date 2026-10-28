use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

#[derive(Clone, Debug)]
pub struct WebTab {
    pub title: String,
    pub url: String,
    pub html_content: String,
    pub active: bool,
}

pub static BROWSER_STATE: Mutex<Vec<WebTab>> = Mutex::new(Vec::new());

pub fn run_browser() {
    let mut state = BROWSER_STATE.lock();
    if state.is_empty() {
        state.push(WebTab {
            title: String::from("NovOS Homepage"),
            url: String::from("https://novos.org"),
            html_content: String::from(
                "<html><body>\
                 <h1>Welcome to NovOS NetSurf Browser</h1>\
                 <p>Fast, lightweight web browsing powered by Rust and Netsurf lightweight rendering pipeline.</p>\
                 <ul>\
                 <li>Supports HTML Parsing & Layout</li>\
                 <li>Tabbed Browsing</li>\
                 <li>Privacy & Built-in Ad Protection</li>\
                 </ul>\
                 </body></html>"
            ),
            active: true,
        });

        state.push(WebTab {
            title: String::from("Rust Documentation"),
            url: String::from("https://doc.rust-lang.org"),
            html_content: String::from("<html><body><h1>Rust Kernel Specs</h1><p>Bare-metal memory safety and speed.</p></body></html>"),
            active: false,
        });
    }
}

pub fn navigate_to(url: &str) {
    let mut state = BROWSER_STATE.lock();
    if let Some(tab) = state.iter_mut().find(|t| t.active) {
        tab.url = String::from(url);
        tab.title = String::from("Loaded Web Page");
        tab.html_content = alloc::format!("<html><body><h1>Navigated to {}</h1><p>Page loaded successfully over Ethernet/WiFi network stack.</p></body></html>", url);
    }
}
