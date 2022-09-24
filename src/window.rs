use std::sync::mpsc::Receiver;
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};
use glfw::ffi::GLFWwindow;
use interoptopus::{ffi_function, ffi_type, ffi_service, ffi_service_method, ffi_service_ctor, Inventory, InventoryBuilder, function, Error, pattern, extra_type};

use crate::error::FFIError;

#[ffi_type(opaque)]
pub struct GameWindow {
    window: Window,
    glfw: Glfw,
    events: Receiver<(f64, WindowEvent)>
}

#[ffi_service(error = "FFIError", prefix = "game_window_")]
impl GameWindow {

    #[ffi_service_ctor]
    pub fn new() -> Result<Self, FFIError> {
        env_logger::init();

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut theWindow, theEvents): (Window, Receiver<(f64, WindowEvent)>) = glfw.create_window(1280, 720, "C# Rust Interop", glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        let gameWindow: Self = GameWindow {
            window: theWindow,
            glfw,
            events: theEvents
        };

        return Ok(gameWindow)
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn run(&mut self) {
        self.window.set_key_polling(true);
        self.window.make_current();

        while !self.window.should_close() {
            self.glfw.poll_events();
        }
    }
}