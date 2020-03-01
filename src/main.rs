use std::time::{Instant, Duration};

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

mod sand;
use sand::{World, Species};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const CELL_SIZE: u32 = 8;
const TARGET_FPS: u128 = 60;

fn main() -> Result<(), String> {
    println!("Welcome to falling sand game!\nControls:\
        \n1: Sand\
        \n2: Wall\
        \nMinus: Smaller Brush\
        \nEquals: Larger Brush\
        \nSpace: Toggle Pause\
        \nR: Reset");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("falling sand", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut world = World::new(WINDOW_WIDTH/CELL_SIZE, WINDOW_HEIGHT/CELL_SIZE, CELL_SIZE);
    let mut drawing: (bool, u32, u32, Species, bool, u32) = (false, 0, 0, Species::Sand, false, 2);
    let mut paused: bool = false;

    let mut event = sdl_context.event_pump()?;
    'running: loop {
        let time = Instant::now();
        let next_time = time.elapsed().as_nanos() + (1_000_000_000u128 / TARGET_FPS);
        for event in event.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Num1 => { drawing.3 = Species::Sand; },
                        Keycode::Num2 => { drawing.3 = Species::Wall; },
                        Keycode::Num3 => { drawing.3 = Species::Water; },
                        Keycode::Minus => { drawing.5 = if drawing.5 == 2 || drawing.5 == 1 {1} else {drawing.5 - 2}; },
                        Keycode::Equals => { drawing.5 = if drawing.5 == 1 {2} else {drawing.5 + 2}; },
                        Keycode::Space => { paused = !paused;},
                        Keycode::R => { world.reset(); }
                        _ => {}
                    }
                },
                Event::MouseButtonDown {mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Right {
                        drawing.4 = true;
                    }
                    drawing.0 = true;
                },
                Event::MouseButtonUp {mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Right {
                         drawing.4 = false;
                    }
                    drawing.0 = false;
                },
                Event::MouseMotion {x, y, ..} => {
                    drawing.1 = x as u32;
                    drawing.2 = y as u32;
                },
                _ => {}
            }
        }
        if drawing.0 {
            if drawing.4
                {world.paint(drawing.1/CELL_SIZE,drawing.2/CELL_SIZE, drawing.5, Species::Empty, true);}
            else
                {world.paint(drawing.1/CELL_SIZE,drawing.2/CELL_SIZE, drawing.5, drawing.3, false);}
        }
        // tick the world
        if !paused
            { world.tick(); }
        // render the world and then display it
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255,0,0));
        let mut rect = Rect::new(0,0,CELL_SIZE,CELL_SIZE);

        for x in 0..WINDOW_WIDTH/CELL_SIZE {
            for y in 0..WINDOW_HEIGHT/CELL_SIZE {
                let c = world.get_cell(x as i32, y as i32);
                let colour: u32 = match c.species {
                        Species::Empty => {0xFF_FF_FF},
                        Species::Wall => {0x424242},
                        Species::Sand => {0xEDC9AF},
                        Species::Water => {0x0000FF},};
                canvas.set_draw_color(sdl2::pixels::Color::RGB(
                    ((colour & 0xFF_00_00) >> 16) as u8,
                    ((colour & 0xFF_00) >> 8) as u8,
                    (colour & 0xFF) as u8));
                rect.set_x((x*CELL_SIZE) as i32);
                rect.set_y((y*CELL_SIZE) as i32);
                canvas.fill_rect(Some(rect)).unwrap();
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255,255,255));
        canvas.present();
        let elapsed = time.elapsed().as_nanos();
        if next_time > elapsed {
            std::thread::sleep(Duration::new(0, (next_time - elapsed) as u32));
        }
    }
    Ok(())
}