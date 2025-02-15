use crate::bus::Bus;
use crate::ppu::tile::{
    TileData, TILES_COUNT, TILE_BITS_COUNT, TILE_HEIGHT, TILE_LINE_BYTES_COUNT, TILE_WIDTH,
};
use crate::ui::{SCALE, SPACER, TILE_COLS, TILE_ROWS, X_DRAW_START, Y_SPACER};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowPos};
use sdl2::VideoSubsystem;

pub struct DebugWindow {
    pub canvas: Canvas<Window>,
    tiles_map_rects: [Vec<Rect>; 4],
    tiles: [TileData; TILES_COUNT],
}

impl DebugWindow {
    pub fn new(video_subsystem: &VideoSubsystem) -> DebugWindow {
        let tile_grid_width =
            TILE_COLS as u32 * TILE_WIDTH as u32 * SCALE + (TILE_COLS as u32 * SCALE);
        let tile_grid_height = TILE_ROWS as u32 * TILE_HEIGHT as u32 * SCALE + 122;

        let debug_window = video_subsystem
            .window("Debug Window", tile_grid_width, tile_grid_height)
            .position_centered()
            .build()
            .unwrap();

        Self {
            canvas: debug_window.into_canvas().build().unwrap(),
            tiles_map_rects: allocate_rects_group(
                TILES_COUNT * TILE_LINE_BYTES_COUNT * TILE_BITS_COUNT as usize * 4,
            ),
            tiles: [TileData::default(); TILES_COUNT],
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.canvas
            .window_mut()
            .set_position(WindowPos::Positioned(x), WindowPos::Positioned(y));
    }

    pub fn draw(&mut self, bus: &Bus) {
        bus.video_ram.fill_tiles(&mut self.tiles);
        self.draw_tiles();
    }

    fn draw_tiles(&mut self) {
        let mut col_x_draw = X_DRAW_START;
        let mut row_y_draw: i32 = 0;
        let mut tile_num = 0;
        let mut rects_count: [usize; 4] = [0; 4];
        self.canvas.set_draw_color(Color::RGB(18, 18, 18));
        self.canvas.fill_rect(None).unwrap();

        for row in 0..TILE_ROWS {
            for col in 0..TILE_COLS {
                let tile = self.tiles[tile_num as usize];

                for (line_y, line) in tile.lines.iter().enumerate() {
                    for (color_x, color_id) in line.iter_color_ids().enumerate() {
                        let color_index = color_id as usize;
                        let rect = &mut self.tiles_map_rects[color_index][rects_count[color_index]];
                        rect.x =
                            col_x_draw + (col * SCALE as i32) + (color_x as i32 * SCALE as i32);
                        rect.y = row_y_draw + (row + SCALE as i32) + (line_y as i32 * SCALE as i32);
                        rects_count[color_index] += 1;
                    }
                }

                col_x_draw += SPACER;
                tile_num += 1;
            }

            row_y_draw += SPACER + Y_SPACER;
            col_x_draw = X_DRAW_START;
        }

        fill_rects(&mut self.canvas, &self.tiles_map_rects, rects_count);
        self.canvas.present();
    }
}

pub fn allocate_rects_group(len: usize) -> [Vec<Rect>; 4] {
    let mut recs = Vec::with_capacity(len);
    for _ in 0..recs.capacity() {
        recs.push(Rect::new(0, 0, SCALE, SCALE));
    }

    [recs.clone(), recs.clone(), recs.clone(), recs]
}

pub fn set_tile_recs(recs: &mut [Vec<Rect>; 4], tile: TileData, x: i32, y: i32) -> [usize; 4] {
    let mut rects_count: [usize; 4] = [0; 4];

    for (line_y, lines) in tile.lines.iter().enumerate() {
        for (bit, color_id) in lines.iter_color_ids().enumerate() {
            let rect = &mut recs[color_id as usize][rects_count[color_id as usize]];
            rect.x = x + (bit as i32 * SCALE as i32);
            rect.y = y + (line_y as i32 * SCALE as i32);
            rects_count[color_id as usize] += 1;
        }
    }

    rects_count
}

pub fn fill_rects(canvas: &mut Canvas<Window>, recs: &[Vec<Rect>; 4], rects_count: [usize; 4]) {
    for (color_id, rects) in recs.iter().enumerate() {
        canvas.set_draw_color(SDL_COLORS[color_id]);
        canvas.fill_rects(&rects[..rects_count[color_id]]).unwrap();
    }
}

const SDL_COLORS: [Color; 4] = [
    Color::WHITE,
    Color::RGB(170, 170, 170), // Light Gray
    Color::RGB(85, 85, 85),    // Dark Gray
    Color::BLACK,
];
