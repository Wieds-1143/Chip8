use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::*;
use sdl2::render::Canvas;

pub fn make_pixels(screen_height: i32, screen_width: i32) -> Vec<Rect> {
    let mut pixel_vec = Vec::new();
    for row in 0..32 {
        for column in 0..64 {
            pixel_vec.push(Rect::new(
                column * screen_width / 32,
                row * screen_height / 64,
                screen_width as u32 / 64,
                screen_height as u32 / 32,
            ))
        }
    }
    pixel_vec
}

pub fn color_pixels(
    gfx: &[u8; 32 * 64],
    pixel_vec: &mut Vec<Rect>,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    for index in 0..(32 * 64) {
        if gfx[index] > 0 {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(pixel_vec[index]);
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(pixel_vec[index]);
        }
    }
}
