mod error;
mod window;

use crate::window::run_window;

use interoptopus::{ffi_function, ffi_type, ffi_service, ffi_service_method, ffi_service_ctor, Inventory, InventoryBuilder, function, Error, pattern, extra_type};
use std::ops::Mul;
use crate::error::FFIError;

#[ffi_type(opaque)]
#[derive(Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi_service(error = "FFIError", prefix = "vec2_")]
impl Vec2 {
    #[ffi_service_ctor]
    pub fn new(x: f32, y: f32) -> Result<Self, FFIError> {
        Ok(Self {
            x,
            y
        })
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn multiply(&self, other: Vec2) -> Vec2 {
        return Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn add(&self, other: Vec2) -> Vec2 {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, otherVec: Self) -> Self {
        return Self {x: self.x * otherVec.x, y: self.y * otherVec.y};
    }
}

#[ffi_type]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, otherVec: Self) -> Self {
        return Self::new(self.x * otherVec.x, self.y * otherVec.y, self.z * otherVec.z);
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn vec2_mul(vec1: Vec2, vec2: Vec2) -> Vec2 {
    vec1 * vec2
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn vec3_mul(vec1: Vec3, vec2: Vec3) -> Vec3 {
    vec1 * vec2
}

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(vec2_mul))
        .register(function!(vec3_mul))
        .register(pattern!(Vec2))
        .register(extra_type!(Vec3))
        .register(function!(run_window))
        .inventory()
}