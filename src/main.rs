// SPDX-License-Identifier: BSD-3-Clause
// Author: Björn Busse <bj.rn@baerlin.eu>
//
// Description: Native macOS tray icon app in Rust.
// Loads a PNG icon, displays it in the menu bar.
// Runs a command on left click if --on-left-click is supplied.
//
#[allow(dead_code)]
extern "C" {
    fn NSLog(format: *const std::os::raw::c_void);
    fn object_getClass(obj: *mut Object) -> *const Class;
}

use cocoa::base::{id, nil, YES, NO};
use cocoa::appkit::{NSApp, NSApplication, NSStatusBar, NSStatusItem, NSMenu, NSMenuItem, NSButton, NSImage};
use cocoa::foundation::{NSAutoreleasePool, NSString, NSSize};
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::{Object, Sel, Class};
use std::process::Command;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

// Store the left-click command in a static for the callback

static LEFT_CLICK_CMD: Lazy<Mutex<Option<Arc<String>>>> = Lazy::new(|| Mutex::new(None));
static RIGHT_CLICK_CMD: Lazy<Mutex<Option<Arc<String>>>> = Lazy::new(|| Mutex::new(None));

#[allow(dead_code)]
extern "C" fn tray_right_click(this: &Object, _cmd: Sel) {
    let _ = this;
    let result = std::panic::catch_unwind(|| {
        if let Some(cmd) = RIGHT_CLICK_CMD.lock().unwrap().as_ref() {
            let result = Command::new("/bin/sh").arg("-c").arg(cmd.as_str()).spawn();
            let log_msg = if let Ok(child) = &result {
                format!("Launched right-click command: {} (pid {})", cmd, child.id())
            } else {
                format!("Failed to launch right-click command: {}", cmd)
            };
            println!("{}", log_msg);
        }
    });
    if let Err(e) = result {
        println!("Right-click handler panicked: {:?}", e);
    }
}

#[allow(dead_code)]
extern "C" fn tray_left_click(this: &Object, _cmd: Sel) {
    let _ = this;
    let result = std::panic::catch_unwind(|| {
        if let Some(cmd) = LEFT_CLICK_CMD.lock().unwrap().as_ref() {
            let result = Command::new("/bin/sh").arg("-c").arg(cmd.as_str()).spawn();
            let log_msg = if let Ok(child) = &result {
                format!("Launched left-click command: {} (pid {})", cmd, child.id())
            } else {
                format!("Failed to launch left-click command: {}", cmd)
            };
            println!("{}", log_msg);
        }
    });
    if let Err(e) = result {
        println!("Left-click handler panicked: {:?}", e);
    }
}

#[cfg(target_os = "macos")]
fn main() {
    let _pool = unsafe { NSAutoreleasePool::new(nil) };
    let args: Vec<String> = std::env::args().collect();
    let mut left_click_cmd: Option<String> = None;
    let mut right_click_cmd: Option<String> = None;
    let mut use_colour = false;
    let mut tray_text: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--on-left-click" && i + 1 < args.len() {
            left_click_cmd = Some(args[i + 1].clone());
            i += 2;
        } else if args[i] == "--on-right-click" && i + 1 < args.len() {
            right_click_cmd = Some(args[i + 1].clone());
            i += 2;
        } else if args[i] == "--use-colour" {
            use_colour = true;
            i += 1;
        } else if args[i] == "--text" && i + 1 < args.len() {
            tray_text = Some(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    if let Some(cmd) = left_click_cmd {
        *LEFT_CLICK_CMD.lock().unwrap() = Some(Arc::new(cmd));
    }
    if let Some(cmd) = right_click_cmd {
        *RIGHT_CLICK_CMD.lock().unwrap() = Some(Arc::new(cmd));
    }
    let app = unsafe { NSApp() };
    unsafe { app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory); }
    let status_item = unsafe { NSStatusBar::systemStatusBar(nil).statusItemWithLength_(cocoa::appkit::NSVariableStatusItemLength) };
    let button: id = unsafe { status_item.button() };
    if let Some(text) = tray_text {
        let ns_text = unsafe { NSString::alloc(nil).init_str(&text) };
        unsafe { button.setImage_(nil); }
        unsafe { button.setTitle_(ns_text); }
    } else {
        // Load PNG icon from bundle Resources using NSBundle, fallback to relative path
        let ns_bundle: id = unsafe { msg_send![class!(NSBundle), mainBundle] };
        let ns_icon_name = unsafe { NSString::alloc(nil).init_str("rocket.png") };
        let ns_icon_path: id = unsafe { msg_send![ns_bundle, pathForResource:ns_icon_name ofType:nil] };
        let image = if !ns_icon_path.is_null() {
            println!("Icon path (NSBundle): {:?}", ns_icon_path);
            unsafe { NSImage::alloc(nil).initByReferencingFile_(ns_icon_path) }
        } else {
            // Fallback: try relative path for dev mode
            let fallback_path = "assets/rocket.png";
            println!("Icon path fallback: {}", fallback_path);
            let ns_fallback_path = unsafe { NSString::alloc(nil).init_str(fallback_path) };
            unsafe { NSImage::alloc(nil).initByReferencingFile_(ns_fallback_path) }
        };
        if image.is_null() {
            println!("Failed to load tray icon image");
        } else {
            let size = NSSize { width: 16.0, height: 16.0 };
            unsafe { let _: () = msg_send![image, setSize: size]; }
            if use_colour {
                unsafe { let _: () = msg_send![image, setTemplate: NO]; }
            } else {
                unsafe { let _: () = msg_send![image, setTemplate: YES]; }
            }
            unsafe { button.setImage_(image); }
            let empty_title = unsafe { NSString::alloc(nil).init_str("") };
            unsafe { button.setTitle_(empty_title); }
        }
    }
        // Set up click handlers if commands are provided
        if LEFT_CLICK_CMD.lock().unwrap().is_some() || RIGHT_CLICK_CMD.lock().unwrap().is_some() {
            let superclass = objc::runtime::Class::get("NSObject").unwrap();
            let mut decl = objc::declare::ClassDecl::new("MugTrayTarget", superclass).unwrap();
            unsafe {
                decl.add_method(sel!(tray_left_click), tray_left_click as extern "C" fn(&Object, Sel));
                decl.add_method(sel!(tray_right_click), tray_right_click as extern "C" fn(&Object, Sel));
            }
            let cls = decl.register();
            let target: id = unsafe { msg_send![cls, alloc] };
            let target: id = unsafe { msg_send![target, init] };
            unsafe { let _: () = msg_send![button, setTarget: target]; }
            // Set left click
            if LEFT_CLICK_CMD.lock().unwrap().is_some() {
                unsafe { let _: () = msg_send![button, setAction: sel!(tray_left_click)]; }
            }
            // Set right click
            if RIGHT_CLICK_CMD.lock().unwrap().is_some() {
                unsafe { let _: () = msg_send![button, setRightMouseDownAction: sel!(tray_right_click)]; }
            }
        } else {
            let menu: id = unsafe { msg_send![NSMenu::alloc(nil), init] };
            let quit_item = unsafe {
                NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
                    NSString::alloc(nil).init_str("Quit"),
                    sel!(terminate:),
                    NSString::alloc(nil).init_str("q"),
                )
            };
            unsafe { menu.addItem_(quit_item); }
            unsafe { status_item.setMenu_(menu); }
        }

        unsafe { app.run(); }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("This tray app only supports macOS natively.");
}