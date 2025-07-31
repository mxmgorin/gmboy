use crate::config::VideoConfig;
use crate::video::game_window::VideoTexture;
use crate::video::{calc_win_height, calc_win_width, new_scaled_rect, shader};
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
    uniform_locations: UniformLocations,
    game_rect: Rect,
    menu_texture: u32,
}

impl GlBackend {
    pub fn new(video_subsystem: &VideoSubsystem, game_rect: Rect) -> Self {
        let window = video_subsystem
            .window("GMBoy GL", game_rect.width(), game_rect.height())
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
            uniform_locations: Default::default(),
            game_rect,
            menu_texture: create_texture(game_rect.w, game_rect.h),
        }
    }

    fn before_draw(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let (dw, dh) = self.window.drawable_size();
            gl::Viewport(0, 0, dw as i32, dh as i32);
            gl::UseProgram(self.shader_program);
        }
    }

    fn draw_quad(&self) {
        unsafe {
            self.send_to_shader();
            gl::BindVertexArray(self.gl_vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    fn send_to_shader(&self) {
        unsafe {
            gl::Uniform1i(self.uniform_locations.image, 0);
            gl::Uniform2f(
                self.uniform_locations.input_resolution,
                VideoConfig::WIDTH as f32,
                VideoConfig::HEIGHT as f32,
            );
            gl::Uniform2f(
                self.uniform_locations.out_resolution,
                self.game_rect.width() as f32,
                self.game_rect.height() as f32,
            );
            gl::Uniform2f(
                self.uniform_locations.origin,
                self.game_rect.x as f32,
                self.game_rect.y as f32,
            );
        }
    }

    pub fn draw_menu(&mut self, texture: &VideoTexture) {
        self.before_draw();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.menu_texture);
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
            gl::Viewport(
                texture.rect.x,
                texture.rect.y,
                texture.rect.w,
                texture.rect.h,
            );

            self.draw_quad();
        }
    }

    pub fn draw_fps(&mut self, _texture: &VideoTexture) {}

    pub fn draw_notif(&mut self, _texture: &VideoTexture) {}

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
        self.before_draw();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.gl_texture);
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
            gl::Viewport(
                self.game_rect.x,
                self.game_rect.y,
                self.game_rect.width() as i32,
                self.game_rect.height() as i32,
            );
            self.draw_quad();
        }
    }

    pub fn show(&self) {
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

            // Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            self.gl_texture = create_texture(VideoConfig::WIDTH as i32, VideoConfig::HEIGHT as i32);
            self.gl_vao = vao;
            self.gl_vbo = vbo;
        }

        self.shader_program = program;
        self.uniform_locations = get_uniform_locations(self.shader_program);

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
        }
    }
}

#[derive(Default)]
struct UniformLocations {
    pub image: i32,
    pub input_resolution: i32,
    pub out_resolution: i32,
    pub origin: i32,
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
