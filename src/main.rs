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

use cocoa::base::{id, nil, YES};
use cocoa::appkit::{NSApp, NSApplication, NSStatusBar, NSStatusItem, NSMenu, NSMenuItem, NSButton, NSImage};
use cocoa::foundation::{NSAutoreleasePool, NSString, NSSize};
use objc::{msg_send, sel, sel_impl};
use objc::runtime::{Object, Sel, Class};
use std::process::Command;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

// Store the left-click command in a static for the callback
static LEFT_CLICK_CMD: Lazy<Mutex<Option<Arc<String>>>> = Lazy::new(|| Mutex::new(None));

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
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--on-left-click" && i + 1 < args.len() {
            left_click_cmd = Some(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    if let Some(cmd) = left_click_cmd {
        *LEFT_CLICK_CMD.lock().unwrap() = Some(Arc::new(cmd));
    }
    let app = unsafe { NSApp() };
    unsafe { app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory); }
    let status_item = unsafe { NSStatusBar::systemStatusBar(nil).statusItemWithLength_(cocoa::appkit::NSVariableStatusItemLength) };
    let button: id = unsafe { status_item.button() };
    // Load PNG icon
    let project_root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let icon_path = format!("{}/assets/rocket.png", project_root);
    println!("Icon path: {}", icon_path);
    let ns_icon_path = unsafe { NSString::alloc(nil).init_str(&icon_path) };
    let image = unsafe { NSImage::alloc(nil).initByReferencingFile_(ns_icon_path) };
    if image.is_null() {
        println!("Failed to load tray icon image");
    } else {
        let size = NSSize { width: 16.0, height: 16.0 };
        unsafe { let _: () = msg_send![image, setSize: size]; }
        unsafe { let _: () = msg_send![image, setTemplate: YES]; }
        unsafe { button.setImage_(image); }
        let empty_title = unsafe { NSString::alloc(nil).init_str("") };
        unsafe { button.setTitle_(empty_title); }
        }
        // Set up left-click handler if command is provided
        if LEFT_CLICK_CMD.lock().unwrap().is_some() {
            // Register MugTrayTarget class and create instance
            let superclass = objc::runtime::Class::get("NSObject").unwrap();
            let mut decl = objc::declare::ClassDecl::new("MugTrayTarget", superclass).unwrap();
            unsafe {
                decl.add_method(sel!(tray_left_click), tray_left_click as extern "C" fn(&Object, Sel));
            }
            let cls = decl.register();
            let target: id = unsafe { msg_send![cls, alloc] };
            let target: id = unsafe { msg_send![target, init] };
            unsafe { let _: () = msg_send![button, setTarget: target]; }
            unsafe { let _: () = msg_send![button, setAction: sel!(tray_left_click)]; }
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