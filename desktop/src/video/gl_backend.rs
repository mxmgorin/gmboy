use crate::config::VideoConfig;
use crate::video::shader;
use sdl2::rect::Rect;
use sdl2::video::{GLContext, Window};
use sdl2::VideoSubsystem;

pub struct GlBackend {
    window: Window,
    _gl_context: GLContext,
    shader_program: u32,
    gl_texture: u32,
    gl_vao: u32,
    gl_vbo: u32,
    uniform_locations: (i32, i32, i32, i32),
    rect: Rect,
    buffer: Box<[u8]>,
}

impl GlBackend {
    pub fn new(video_subsystem: &VideoSubsystem, rect: Rect) -> Self {
        let window = video_subsystem
            .window("GMBoy GL", rect.width(), rect.height())
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        Self {
            window,
            _gl_context: gl_context,
            shader_program: 0,
            gl_texture: 0,
            gl_vao: 0,
            gl_vbo: 0,
            uniform_locations: (0, 0, 0, 0),
            rect,
            buffer: vec![0; VideoConfig::WIDTH * VideoConfig::HEIGHT * 3].into_boxed_slice(),
        }
    }

    /// Uploads ARGB pixels and draws a textured quad
    pub fn draw_buffer(&mut self, buffer: &[u32]) {
        fill_argb_to_rgb(buffer, &mut self.buffer);
        let width = VideoConfig::WIDTH;
        let height = VideoConfig::HEIGHT;

        unsafe {
            let (dw, dh) = self.window.drawable_size();
            gl::Viewport(0, 0, dw as i32, dh as i32);

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.shader_program);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.gl_texture);

            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                width as i32,
                height as i32,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                self.buffer.as_ptr() as *const _,
            );

            gl::Viewport(
                self.rect.x,
                self.rect.y,
                self.rect.width() as i32,
                self.rect.height() as i32,
            );

            // set uniforms
            let (loc_image, loc_in, loc_out, origin) = self.uniform_locations;
            gl::Uniform1i(loc_image, 0);
            gl::Uniform2f(loc_in, width as f32, height as f32);
            gl::Uniform2f(loc_out, self.rect.width() as f32, self.rect.height() as f32);
            gl::Uniform2f(origin, self.rect.x as f32, self.rect.y as f32);

            // draw quad
            gl::BindVertexArray(self.gl_vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    /// Swaps the window buffers to display rendered frame
    pub fn present(&self) {
        self.window.gl_swap_window();
    }

    /// Loads and initializes shaders + GPU resources
    pub fn load_shader(&mut self, name: &str) -> Result<(), String> {
        let program = shader::load_shader_program(name)?;

        unsafe {
            // Create VAO + VBO
            let mut vao = 0;
            let mut vbo = 0;
            let vertices: [f32; 16] = [
                -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            ];

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of_val(&vertices)) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = 4 * std::mem::size_of::<f32>() as i32;
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (2 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                VideoConfig::WIDTH as i32,
                VideoConfig::HEIGHT as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            // Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            self.gl_texture = texture;
            self.gl_vao = vao;
            self.gl_vbo = vbo;
        }

        self.shader_program = program;
        self.cache_uniform_locations();

        Ok(())
    }

    fn cache_uniform_locations(&mut self) {
        unsafe {
            let program = self.shader_program;
            self.uniform_locations = (
                gl::GetUniformLocation(program, c"image".as_ptr() as *const _),
                gl::GetUniformLocation(program, c"input_resolution".as_ptr() as *const _),
                gl::GetUniformLocation(program, c"output_resolution".as_ptr() as *const _),
                gl::GetUniformLocation(program, c"origin".as_ptr() as *const _),
            );
        }
    }
}

/// Converts ARGB -> RGB (3 bytes per pixel)
pub fn fill_argb_to_rgb(src: &[u32], dst: &mut [u8]) {
    debug_assert_eq!(dst.len(), src.len() * 3);

    let mut i = 0;
    for &p in src {
        dst[i] = ((p >> 16) & 0xFF) as u8; // R
        dst[i + 1] = ((p >> 8) & 0xFF) as u8; // G
        dst[i + 2] = (p & 0xFF) as u8; // B
        i += 3;
    }
}
