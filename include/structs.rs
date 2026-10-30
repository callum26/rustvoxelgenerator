// copied logic from cuda ray tracer project, with some modifications in order to suit this project
// github.com/callum26/cudaraytracer/include/structs.h
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    // def constructor
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // dot product
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // vector mag
    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }

    // norm vec
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            Self::default()
        } else {
            *self / magnitude
        }
    }

    // cross prod
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// arithmetic opps for vec math
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
// multiplying by scalars
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// div by scalars
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Material {
    pub colour: Vec3,
    pub ambient_reflectivity: f32,
    pub diffuse_reflectivity: f32,
    pub specular_reflectivity: f32,
    pub shininess: f32,
    pub transparency: f32,
    pub refraction: f32,
    pub absorption: Vec3,
    pub emission: Vec3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Light {
    pub position: Vec3,
    pub ambient_intensity: Vec3,
    pub diffuse_intensity: Vec3,
    pub specular_intensity: Vec3,
    pub light_intensity: f32,
    pub light_radius: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub normal: Vec3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub hit_point: Vec3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Aabb {
    pub box_min: Vec3,
    pub box_max: Vec3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BvhNode {
    pub bounds: Aabb,
    pub left_index: i32,
    pub right_index: i32,
    pub first_object: i32,
    pub object_count: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BuildObject {
    pub bounds: Aabb,
    pub centroid: Vec3,
    pub object_index: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SahBucket {
    pub count: i32,
    pub bounds: Aabb,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SurfaceInteraction {
    pub normal: Vec3,
    pub inside: bool,
}
