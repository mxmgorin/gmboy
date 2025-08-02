use crate::config::{GlConfig, VideoConfig};
use crate::video::game_window::VideoTexture;
use crate::video::shader::ShaderFrameBlendMode;
use crate::video::{calc_win_height, calc_win_width, new_scaled_rect, shader};
use sdl2::rect::Rect;
use sdl2::video::{GLContext, Window};
use sdl2::VideoSubsystem;

pub struct GlBackend {
    window: Window,
    _gl_context: GLContext,
    shader_program: u32,
    frame_texture_id: u32,
    prev_frame_texture_id: u32,
    vao: u32,
    vbo: u32,
    uniform_locations: UniformLocations,
    game_rect: Rect,
    fps_texture_id: u32,
    notif_texture_id: u32,
    shader_frame_blend_mode: ShaderFrameBlendMode,
    prev_buffer: Box<[u8]>
}

impl GlBackend {
    pub fn new(
        video_subsystem: &VideoSubsystem,
        game_rect: Rect,
        fps_rect: Rect,
        notif_rect: Rect,
        config: &GlConfig,
    ) -> Result<Self, String> {
        let window = video_subsystem
            .window("GMBoy GL", game_rect.width(), game_rect.height())
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context()?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let mut obj = Self {
            window,
            _gl_context: gl_context,
            shader_program: 0,
            frame_texture_id: 0,
            vao: 0,
            vbo: 0,
            uniform_locations: Default::default(),
            game_rect,
            fps_texture_id: create_texture(fps_rect.w, fps_rect.h),
            prev_frame_texture_id: 0,
            notif_texture_id: create_texture(notif_rect.w, notif_rect.h),
            shader_frame_blend_mode: config.shader_frame_blend_mode,
            prev_buffer: Box::new([]),
        };
        obj.update_config(config);

        Ok(obj)
    }

    pub fn update_config(&mut self, config: &GlConfig) {
        self.load_shader(&config.shader_name, self.shader_frame_blend_mode)
            .unwrap();
    }

    fn draw_quad(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    pub fn draw_menu(&mut self, texture: &VideoTexture) {
        self.draw_buffer(&texture.buffer);
    }

    pub fn draw_fps(&mut self, texture: &VideoTexture) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.fps_texture_id);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                texture.rect.w,
                texture.rect.h,
                gl::BGRA,
                gl::UNSIGNED_BYTE,
                texture.buffer.as_ptr() as *const _,
            );
            self.draw_quad();
        }
    }

    pub fn draw_notif(&mut self, texture: &VideoTexture) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.notif_texture_id);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                texture.rect.w,
                texture.rect.h,
                gl::BGRA,
                gl::UNSIGNED_BYTE,
                texture.buffer.as_ptr() as *const _,
            );
            self.draw_quad();
        }
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        self.window
            .set_size(calc_win_width(scale), calc_win_height(scale))
            .map_err(|e| e.to_string())?;
        self.window.set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered,
        );
        self.update_game_rect();

        Ok(())
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        if fullscreen {
            self.window
                .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                .unwrap();
        } else {
            self.window
                .set_fullscreen(sdl2::video::FullscreenType::Off)
                .unwrap();
        };
        self.update_game_rect();
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.window.position()
    }

    pub fn draw_buffer(&mut self, buffer: &[u8]) {
        let width = VideoConfig::WIDTH;
        let height = VideoConfig::HEIGHT;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.shader_program);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.frame_texture_id);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                width as i32,
                height as i32,
                gl::BGRA,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const _,
            );

            if self.shader_frame_blend_mode != ShaderFrameBlendMode::None {
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, self.prev_frame_texture_id);
                gl::TexSubImage2D(
                    gl::TEXTURE_2D,
                    0,
                    0,
                    0,
                    width as i32,
                    height as i32,
                    gl::BGRA,
                    gl::UNSIGNED_BYTE,
                    self.prev_buffer.as_ptr() as *const _,
                );
                self.uniform_locations.send_prev_image();
                self.copy_buffer(buffer);
            }

            gl::Viewport(
                self.game_rect.x,
                self.game_rect.y,
                self.game_rect.width() as i32,
                self.game_rect.height() as i32,
            );

            self.uniform_locations.send_image();
            self.uniform_locations
                .send_frame_blend_mode(self.shader_frame_blend_mode);
            self.uniform_locations
                .send_in_resolution(VideoConfig::WIDTH as f32, VideoConfig::HEIGHT as f32);
            self.uniform_locations.send_out_resolution(
                self.game_rect.width() as f32,
                self.game_rect.height() as f32,
            );
            self.uniform_locations
                .send_origin(self.game_rect.x as f32, self.game_rect.y as f32);

            self.draw_quad();
        }
    }

    pub fn copy_buffer(&mut self, buffer: &[u8]) {
        if buffer.len() != self.prev_buffer.len(){
            return;
        }

        for (i, v) in buffer.iter().enumerate() {
            self.prev_buffer[i] = *v;
        }
    }

    pub fn show(&self) {
        self.window.gl_swap_window();
    }

    /// Loads and initializes shaders + GPU resources
    pub fn load_shader(
        &mut self,
        name: &str,
        frame_blend_mode: ShaderFrameBlendMode,
    ) -> Result<(), String> {
        let program = shader::load_shader_program(name)?;

        unsafe {
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
                size_of_val(&vertices) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = 4 * size_of::<f32>() as i32;
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (2 * size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            // Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            self.frame_texture_id =
                create_texture(VideoConfig::WIDTH as i32, VideoConfig::HEIGHT as i32);
            self.vao = vao;
            self.vbo = vbo;
        }

        self.shader_program = program;
        self.uniform_locations = get_uniform_locations(self.shader_program);
        self.uniform_locations.send_image();
        self.uniform_locations.send_prev_image();
        self.uniform_locations
            .send_frame_blend_mode(frame_blend_mode);

        self.prev_frame_texture_id = create_texture(self.game_rect.w, self.game_rect.h);
        self.prev_buffer = vec![0; VideoConfig::WIDTH * VideoConfig::HEIGHT].into_boxed_slice();

        Ok(())
    }

    fn update_game_rect(&mut self) {
        let (win_width, win_height) = self.window.size();
        self.game_rect = new_scaled_rect(win_width, win_height);
    }
}

fn get_uniform_locations(program: u32) -> UniformLocations {
    unsafe {
        UniformLocations {
            image: gl::GetUniformLocation(program, c"image".as_ptr() as *const _),
            input_resolution: gl::GetUniformLocation(
                program,
                c"input_resolution".as_ptr() as *const _,
            ),
            out_resolution: gl::GetUniformLocation(
                program,
                c"output_resolution".as_ptr() as *const _,
            ),
            origin: gl::GetUniformLocation(program, c"origin".as_ptr() as *const _),
            frame_blending_mode: gl::GetUniformLocation(
                program,
                c"frame_blending_mode".as_ptr() as *const _,
            ),
            prev_image: gl::GetUniformLocation(program, c"previous_image".as_ptr() as *const _),
        }
    }
}

#[derive(Default)]
struct UniformLocations {
    pub image: i32,
    pub input_resolution: i32,
    pub out_resolution: i32,
    pub origin: i32,
    pub frame_blending_mode: i32,
    pub prev_image: i32,
}

impl UniformLocations {
    pub fn send_image(&self) {
        unsafe {
            gl::Uniform1i(self.image, 0);
        }
    }

    pub fn send_prev_image(&self) {
        unsafe {
            gl::Uniform1i(self.prev_image, 1);
        }
    }

    pub fn send_in_resolution(&self, w: f32, h: f32) {
        unsafe {
            gl::Uniform2f(self.input_resolution, w, h);
        }
    }

    pub fn send_out_resolution(&self, w: f32, h: f32) {
        unsafe {
            gl::Uniform2f(self.out_resolution, w, h);
        }
    }

    pub fn send_origin(&self, x: f32, y: f32) {
        unsafe {
            gl::Uniform2f(self.origin, x, y);
        }
    }

    pub fn send_frame_blend_mode(&self, mode: ShaderFrameBlendMode) {
        let mode = mode as i32;

        unsafe {
            gl::Uniform1i(self.frame_blending_mode, mode);
        }
    }
}

pub fn create_texture(w: i32, h: i32) -> u32 {
    unsafe {
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
            gl::RGBA as i32,
            w,
            h,
            0,
            gl::BGRA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );

        texture
    }
}
