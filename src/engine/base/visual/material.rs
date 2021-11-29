pub enum AttrSize {
    Scaler = 1,
    Vec2 = 2,
    Vec3 = 3,
    Vec4 = 4,
    Mat4 = 16,
}

pub enum AttrType {
    Int,
    Float,
}

pub enum AttrData {
    Integer(Vec<i32>),
    Float(Vec<f32>),
}

pub struct UniformInfo {
    index: usize,
    size:  AttrSize,
    data:  Vec<AttrEle>,
}

pub enum AttrEle {
    Int(i32),
    Float(f32),
}

#[derive(Copy, Clone)]
pub enum UniformData {
    Int(i32),
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat4([f32; 16]),
}

// pub struct Material {
//     shader:
// }