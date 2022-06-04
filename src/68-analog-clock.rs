extern crate sdl2;
extern crate time;

mod font;

use font::print_text;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Renderer;
use sdl2::ttf::Font;

const W_WIDTH: u32 = 320;
const W_HEIGHT: u32 = 320;
const H_AMP: f32 = 40.0;
const M_AMP: f32 = 60.0;
const S_AMP: f32 = 100.0;
const FONT_SIZE: u16 = 28;

fn get_point(center: Point, angle: f32, amplitute: f32) -> Point {
    use std::f32::consts::FRAC_PI_2;
    let x = center.x + (amplitute * (angle - FRAC_PI_2).cos()).round() as i32;
    let y = center.y + (amplitute * (angle - FRAC_PI_2).sin()).round() as i32;
    Point::new(x, y)
}

fn render<'a>(renderer: &'a mut Renderer, font: &'a mut Font) {
    let center = Point::new((W_WIDTH / 2) as i32, (W_HEIGHT / 2) as i32);
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    for name in 1..13 {
        let text = format!("{}", name);
        let angle = (name as f32 * 30.0).to_radians();
        let mut pos = get_point(center, angle, 120.0);
        let (w, h) = font.size_of(&text).unwrap();
        pos.y -= (h / 2) as i32;
        pos.x -= (w / 2) as i32;
        print_text(renderer, font, &text, pos);
    }
    let time = time::now();
    let h_angle = (time.tm_hour as f32 * 30.0).to_radians();
    let m_angle = (time.tm_min as f32 * 6.0).to_radians();
    let s_angle = (time.tm_sec as f32 * 6.0).to_radians();
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.draw_line(center, get_point(center, h_angle, H_AMP)).unwrap();
    renderer.set_draw_color(Color::RGB(0, 0, 255));
    renderer.draw_line(center, get_point(center, m_angle, M_AMP)).unwrap();
    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.draw_line(center, get_point(center, s_angle, S_AMP)).unwrap();
    renderer.present();
}

fn main() {
    let sdl_contex = sdl2::init().unwrap();
    let video_subsystem = sdl_contex.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let window = video_subsystem.window("analog clock", W_WIDTH, W_HEIGHT).position_centered().build().unwrap();
    let mut renderer = window.renderer().present_vsync().build().unwrap();
    let mut font = ttf_context.load_font("assets/SourceSansPro-Regular.ttf", FONT_SIZE).unwrap();
    font.set_style(sdl2::ttf::STYLE_NORMAL);
    let mut event_pump = sdl_contex.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => (),
            }
        }
        render(&mut renderer, &mut font);
    }
}
