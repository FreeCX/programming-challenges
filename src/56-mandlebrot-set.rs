extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use std::f32;

const W_WIDTH: u32 = 640;
const W_HEIGHT: u32 = 480;
const X_MIN: f32 = -2.2;
const X_MAX: f32 = 1.0;
const Y_MIN: f32 = -1.2;
const Y_MAX: f32 = 1.2;

fn delta(min: f32, max: f32, width: f32) -> f32 {
    (min.abs() + max.abs()) / width
}

fn stepper<F>(array: &mut Vec<(u8, u8, u8)>, step: u8, f: F)
where
    F: Fn(u8) -> (u8, u8, u8),
{
    for color in (0..255).filter(|x| x % step == 0) {
        array.push(f(color));
    }
}

fn render<'a>(renderer: &'a mut Renderer, texture: &Texture) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.copy(texture, None, Some(Rect::new(0, 0, W_WIDTH, W_HEIGHT))).unwrap();
    renderer.present();
}

fn gen_texture(renderer: &Renderer) -> Texture {
    let mut hist = Vec::new();
    stepper(&mut hist, 10, |c| (0, c, 255 - c));
    stepper(&mut hist, 10, |c| (c, 255 - c, 0));
    stepper(&mut hist, 10, |c| (255 - c, 0, c));
    stepper(&mut hist, 90, |c| (255 - c, 0, 250 - c));
    let max_iteration = hist.len() - 1;
    let dx = delta(X_MIN, X_MAX, W_WIDTH as f32);
    let dy = delta(Y_MIN, Y_MAX, W_HEIGHT as f32);
    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, W_WIDTH, W_HEIGHT).unwrap();
    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for px in 0..W_WIDTH {
                for py in 0..W_HEIGHT {
                    let x0 = (px as f32) * dx + X_MIN;
                    let y0 = (py as f32) * dy + Y_MIN;
                    let mut x: f32 = 0.0;
                    let mut y: f32 = 0.0;
                    let mut iteration = 0;
                    while x.powi(2) + y.powi(2) < 4.0 && iteration < max_iteration {
                        let xtemp = x.powi(2) - y.powi(2) + x0;
                        y = 2.0 * x * y + y0;
                        x = xtemp;
                        iteration += 1;
                    }
                    let offset = (py as usize) * pitch + (px as usize) * 3;
                    let (r, g, b) = hist[iteration];
                    buffer[offset + 0] = r;
                    buffer[offset + 1] = g;
                    buffer[offset + 2] = b;
                }
            }
        })
        .unwrap();
    texture
}

fn main() {
    let sdl_contex = sdl2::init().unwrap();
    let video_subsystem = sdl_contex.video().unwrap();
    let window = video_subsystem.window("mandlebrot set", W_WIDTH, W_HEIGHT).position_centered().build().unwrap();
    let mut renderer = window.renderer().present_vsync().build().unwrap();
    let texture = gen_texture(&renderer);
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
        render(&mut renderer, &texture);
    }
}
