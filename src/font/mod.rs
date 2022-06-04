#[warn(dead_code)]
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Renderer, TextureQuery};
use sdl2::ttf::Font;

pub fn print_text<'a>(renderer: &'a mut Renderer, font: &'a Font, text: &'a str, pos: Point) {
    let surface = font.render(text).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
    let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(pos.x, pos.y, width, height);
    renderer.copy(&mut texture, None, Some(target)).unwrap();
}
