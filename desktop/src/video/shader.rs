use gl::types::GLenum;
use std::ffi::CString;
use serde::{Deserialize, Serialize};

const VERTEX_SHADER: &str = include_str!("../../shaders/master.vert");
const PASSTHROUGH_FRAGMENT: &str = include_str!("../../shaders/passthrough.frag");
const BILINEAR_FRAGMENT: &str = include_str!("../../shaders/bilinear.frag");
const SMOOTH_BILINEAR_FRAGMENT: &str = include_str!("../../shaders/smooth_bilinear.frag");
const CRT_FRAGMENT: &str = include_str!("../../shaders/crt.frag");
const MASTER_FRAGMENT: &str = include_str!("../../shaders/master.frag");
const FLAT_CRT_FRAGMENT: &str = include_str!("../../shaders/flat_crt.fsh");
const HQ2X_FRAGMENT: &str = include_str!("../../shaders/HQ2x.fsh");
const AAOMNI_SCALE_LEGACY: &str = include_str!("../../shaders/AAOmniScaleLegacy.fsh");
const AASCALE2X: &str = include_str!("../../shaders/AAScale2x.fsh");
const AASCALE4X: &str = include_str!("../../shaders/AAScale4x.fsh");
const LCD: &str = include_str!("../../shaders/LCD.fsh");
const MONO_LCD: &str = include_str!("../../shaders/MonoLCD.fsh");
const OMNI_SCALE: &str = include_str!("../../shaders/OmniScale.fsh");
const SCALE2X: &str = include_str!("../../shaders/Scale2x.fsh");
const SCALE4X: &str = include_str!("../../shaders/Scale4x.fsh");

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum ShaderFrameBlendMode {
    None = 0,
    Simple = 1,
    AccEven = 2,
    AccOdd = 3,
}

pub const SHADERS: [(&str, &str); 14] = [
    ("passthrough", PASSTHROUGH_FRAGMENT),
    ("bilinear", BILINEAR_FRAGMENT),
    ("smooth_bilinear", SMOOTH_BILINEAR_FRAGMENT),
    ("crt", CRT_FRAGMENT),
    ("flat_crt", FLAT_CRT_FRAGMENT),
    ("HQ2x", HQ2X_FRAGMENT),
    ("AAOmniScaleLegacy", AAOMNI_SCALE_LEGACY),
    ("AAScale2x", AASCALE2X),
    ("AAScale4x", AASCALE4X),
    ("LCD", LCD),
    ("MonoLCD", MONO_LCD),
    ("OmniScale", OMNI_SCALE),
    ("Scale2x", SCALE2X),
    ("Scale4x", SCALE4X),
];

pub fn load_shader_program(name: &str) -> Result<u32, String> {
    for (i_name, shader) in SHADERS {
        if i_name.to_lowercase() == name.to_lowercase() {
            let fragment_source = MASTER_FRAGMENT.replace("{filter}", shader);
            return unsafe { compile_program(VERTEX_SHADER, &fragment_source) };
        }
    }

    Err(format!("Shader {name} not found"))
}

pub fn next_shader_by_name<'a>(current_name: &str) -> (&'a str, &'a str) {
    let idx = SHADERS
        .iter()
        .position(|(name, _)| *name == current_name)
        .unwrap_or(0);
    // Calculate next index with wrap-around
    let next_idx = (idx + 1) % SHADERS.len();

    SHADERS[next_idx]
}

pub fn prev_shader_by_name<'a>(current_name: &str) -> (&'a str, &'a str) {
    let idx = SHADERS
        .iter()
        .position(|(name, _)| *name == current_name)
        .unwrap_or(0);

    // Calculate previous index with wrap-around
    let prev_idx = if idx == 0 { SHADERS.len() - 1 } else { idx - 1 };

    SHADERS[prev_idx]
}

unsafe fn compile_program(vertex_src: &str, fragment_src: &str) -> Result<u32, String> {
    let vs = compile_shader(gl::VERTEX_SHADER, vertex_src)?;
    let fs = compile_shader(gl::FRAGMENT_SHADER, fragment_src)?;
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    gl::DeleteShader(vs);
    gl::DeleteShader(fs);
    let mut status = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    if status == 0 {
        let mut len = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = vec![0u8; (len as usize) - 1];
        gl::GetProgramInfoLog(
            program,
            len,
            std::ptr::null_mut(),
            buf.as_mut_ptr() as *mut _,
        );
        gl::DeleteProgram(program);
        return Err(String::from_utf8_lossy(&buf).into_owned());
    }
    Ok(program)
}

unsafe fn compile_shader(kind: GLenum, source: &str) -> Result<u32, String> {
    let shader = gl::CreateShader(kind);
    let c_str = CString::new(source).unwrap();
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
    gl::CompileShader(shader);
    let mut status = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    if status == 0 {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = vec![0u8; (len as usize) - 1];
        gl::GetShaderInfoLog(
            shader,
            len,
            std::ptr::null_mut(),
            buf.as_mut_ptr() as *mut _,
        );
        gl::DeleteShader(shader);
        return Err(String::from_utf8_lossy(&buf).into_owned());
    }
    Ok(shader)
}
