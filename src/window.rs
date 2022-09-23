use glfw::{Action, Context, Key};
use interoptopus::{ffi_function, ffi_type, ffi_service, ffi_service_method, ffi_service_ctor, Inventory, InventoryBuilder, function, Error, pattern, extra_type};

#[ffi_function]
#[no_mangle]
pub extern "C" fn run_window()
{
    env_logger::init();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(1280, 720, "C# Rust Interop", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
    }
}