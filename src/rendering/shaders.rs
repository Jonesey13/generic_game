use glium::{Program, Display};
use glium::program::{SourceCode, ProgramCreationInput};

#[allow(dead_code)]
pub enum Shaders {
    None,
    VertexFragment(&'static str, &'static str),
    VertexGeometryFragment(&'static str, &'static str, &'static str),
    VertexTesselationFragment(&'static str, &'static str, &'static str, &'static str),
    VertexTesselationGeometryFragment(&'static str, &'static str, &'static str, &'static str, &'static str),
}

pub fn make_program_from_shaders(shaders: Shaders, display: &Display) -> Program {
    match shaders {
        Shaders::None => panic!("Cannot build a Program when there are no Shaders!"),
        Shaders::VertexFragment(vert, frag) => {
            Program::new(
                display, 
                ProgramCreationInput::SourceCode {
                    vertex_shader: vert,
                    tessellation_control_shader: None,
                    tessellation_evaluation_shader: None,
                    geometry_shader: None,
                    fragment_shader: frag,
                    outputs_srgb: true,
                    transform_feedback_varyings: None,
                    uses_point_size: false
                }).unwrap()
        }
        Shaders::VertexGeometryFragment(vert, geom, frag) => { 
            Program::new(
                display, 
                ProgramCreationInput::SourceCode {
                    vertex_shader: vert,
                    tessellation_control_shader: None,
                    tessellation_evaluation_shader: None,
                    geometry_shader: Some(geom),
                    fragment_shader: frag,
                    outputs_srgb: true,
                    transform_feedback_varyings: None,
                    uses_point_size: false
                }).unwrap()
        }
        Shaders::VertexTesselationFragment(vert, tess_ctrl, tess_eval, frag) => {
            Program::new(
                display,
                ProgramCreationInput::SourceCode {
                    vertex_shader: vert,
                    tessellation_control_shader: Some(tess_ctrl),
                    tessellation_evaluation_shader: Some(tess_eval),
                    geometry_shader: None,
                    fragment_shader: frag,
                    outputs_srgb: true,
                    transform_feedback_varyings: None,
                    uses_point_size: false
                }).unwrap()
        },
        Shaders::VertexTesselationGeometryFragment(vert, tess_ctrl, tess_eval, geom, frag) => {
            Program::new(
                display,
                ProgramCreationInput::SourceCode {
                    vertex_shader: vert,
                    tessellation_control_shader: Some(tess_ctrl),
                    tessellation_evaluation_shader: Some(tess_eval),
                    geometry_shader: Some(geom),
                    fragment_shader: frag,
                    outputs_srgb: true,
                    transform_feedback_varyings: None,
                    uses_point_size: false
                }).unwrap()
        }
    }
}
