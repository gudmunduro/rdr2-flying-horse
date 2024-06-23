use std::{ops::{Add, Mul}, fmt::Display};


pub type DWORD = u32;
pub type WORD = u16;
pub type BOOL = i32;
pub type BYTE = u8;
pub type Any = u32;
pub type Hash = u32;
pub type Entity = i32;
pub type Player = i32;
pub type FireId = i32;
pub type Ped = i32;
pub type Vehicle = i32;
pub type Cam = i32;
pub type CarGenerator = i32;
pub type Group = i32;
pub type Train = i32;
pub type Pickup = i32;
pub type Object = i32;
pub type Weapon = i32;
pub type Interior = i32;
pub type Blip = i32;
pub type Texture = i32;
pub type TextureDict = i32;
pub type CoverPoint = i32;
pub type Camera = i32;
pub type TaskSequence = i32;
pub type ColourIndex = i32;
pub type Sphere = i32;
pub type ScrHandle = i32;
pub type AnimScene = i32;
pub type Volume = i32;
pub type ItemSet = i32;
pub type Prompt = i32;
pub type PropSet = i32;
pub type PersChar = i32;
pub type PopZone = i32; 

#[repr(C, align(1))]
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    paddingx: DWORD,
    pub y: f32,
    paddingy: DWORD,
    pub z: f32,
    paddingz: DWORD
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            paddingx: 0,
            y,
            paddingy: 0,
            z,
            paddingz: 0
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Vector3 {x, y, z, ..} = *self;
        f.write_str(&format!("Vector3({x}, {y}, {z})"))?;
        Ok(())
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &'b Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a> Mul<f32> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}