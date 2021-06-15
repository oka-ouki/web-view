#![windows_subsystem = "windows"]

extern crate web_view;
extern crate libc;

use web_view::*;
use std::ffi::CStr;
use libc::c_char;

fn main() {
    let mut webview1 = web_view::builder()
        .title("Multi window example - Window 1")
        .content(Content::Url("https://en.m.wikipedia.org/wiki/Main_Page"))
        .size(800, 600)
        .resizable(true)
        .debug(false)
        .user_data(0)
        .invoke_handler(|webview, arg| {
            match arg {
                "2" => *webview.user_data_mut() = 2,
                _ => *webview.user_data_mut() = 1,
            }

            Ok(())
        })
        .build()
        .unwrap();

    let mut webview2 = web_view::builder()
        .title("Multi window example - Window 2")
        .content(Content::Url("https://en.m.wikipedia.org/wiki/Main_Page"))
        .size(800, 600)
        .resizable(true)
        .debug(false)
        .user_data(0)
        .invoke_handler(|webview, arg| {
            match arg {
                "2" => *webview.user_data_mut() = 2,
                _ => *webview.user_data_mut() = 1,
            }

            Ok(())
        })
        .build()
        .unwrap();

    loop {
        if webview1.step().is_none() {
            break;
        }
        if webview2.step().is_none() {
            break;
        }
        if *webview1.user_data() != 0 {
            unsafe {
                let url = CStr::from_ptr(&(*webview1.link_url()) as *const c_char).to_str().unwrap();
                if *webview1.user_data() == 2 {
                    let _ = webview2.eval(&format!("{}{}{}", "document.documentElement.innerHTML='", format!("{}{}{}", HTML1, url, HTML2).replace('\n', ""), "';"));
                } else {
                    let _ = webview2.eval(&format!("{}{}{}", "window.location.href='", url, "';"));
                }
            }
            *webview1.user_data_mut() = 0;
        }
        if *webview2.user_data() != 0 {
            unsafe {
                let url = CStr::from_ptr(&(*webview2.link_url()) as *const c_char).to_str().unwrap();
                if *webview2.user_data() == 2 {
                    let _ = webview1.eval(&format!("{}{}{}", "document.documentElement.innerHTML='", format!("{}{}{}", HTML1, url, HTML2).replace('\n', ""), "';"));
                } else {
                    let _ = webview1.eval(&format!("{}{}{}", "window.location.href='", url, "';"));
                }
            }
            *webview2.user_data_mut() = 0;
        }
    }
}

const HTML1: &str = r#"
<body style="background-color: black; margin: 0; padding: 0;">
  <img src="
"#;

const HTML2: &str = r#"
" style="width: 100vw; height: 100vh; object-fit: contain;">
  </body>
</html>
"#;
