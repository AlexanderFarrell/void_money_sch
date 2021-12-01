use crate::engine::base::app::{CoreComponent, Source};
use crate::engine::base::visual::ClearInfo;

pub struct ShaderNew {

}

impl ShaderNew {
    pub fn create(context: &impl Context, source: Vec<ShaderType, Source>) {
        
    }

    pub fn delete(context: &impl Context) {

    }
    
    pub fn uniform_location(context: &impl Context) -> u32 {

    }

    pub fn attribute_location(context: &impl Context) -> u32 {

    }
}

pub trait Shader<T: Context, UniLoc, AttrLoc>: Drop + Bindable<T> {
    fn create(context: &T, source: Vec<ShaderTypes, Source>) -> Result<Self, String>;
    fn delete();
    fn uniform_location(context: &T, name: &str) -> UniLoc;
    fn attribute_location(context: &T, name: &str) -> AttrLoc;
}

pub trait Bindable<T: Context> {
    fn bind(context: &T);
}

pub trait Material<T: Context> {

}

pub enum ShaderTypes {
    Vertex,
    Fragment,
    Geometry,
    Compute,
}

pub trait Context: CoreComponent {
    type ShaderSource = (Self::ShaderType, Source);
    type Program;
    type Shader: Shader<Self>;
    type UniformLocation;
    type AttributeLocation;
    type ShaderType;
    type ShaderParameter;

    fn shader_type(shader_type: ShaderTypes) -> Self::ShaderType;

    //Internal
    //Programs
    // fn create_program(sources: Vec<Self::ShaderSource>) -> Result<Self::Program, String>;
    //
    // fn link_program(program: &Self::Program) -> Result<(), String>;
    //
    // fn program_parameter_exists(program: &Self::Program) -> Result<bool, String>;
    //
    // fn program_info_log(program: &Self::Program) -> Result<String, String>;
    //
    // fn bind_program(program: &Self::Program);
    //
    // //Shaders
    // fn create_shader(shader_type: Self::ShaderType) -> Result<Self::Shader, String>;
    //
    // fn attach_shader_source(shader: &Self::Shader, source: Source);
    //
    // fn compile_shader(shader: &Self::Shader);
    //
    // fn shader_parameter(shader: &Self::Shader, param: Self::ShaderParameter) -> Result<bool, String>;
    //
    // fn shader_info_log(shader: &Self::Shader) -> Result<String, String>;

    //Drawing
    fn clear(info: &ClearInfo);
}

// pub struct ContextWeb {
//
// }
//
// impl CoreComponent for ContextWeb {
//     fn setup(&mut self) {
//         todo!()
//     }
//
//     fn breakdown(&mut self) {
//         todo!()
//     }
// }
//
// impl Context for ContextWeb {
//     type Program = i32;
//     type UniformLocation = f32;
//     type AttributeLocation = i32;
//
//     fn create_program() -> Self::Program {
//         return 8.52;
//     }
// }
//
// fn something() {
//
// }