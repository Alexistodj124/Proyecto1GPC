use std::time::{Instant, Duration};
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::caster::cast_ray;
use image::GenericImageView;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

mod framebuffer;
mod maze;
mod player;
mod caster;

fn load_texture(path: &str) -> image::DynamicImage {
    image::open(path).expect("Failed to load texture")
}

fn draw_wall_slice(framebuffer: &mut Framebuffer, x: usize, height: usize, texture: &image::DynamicImage, texture_x: usize) {
    let screen_height = framebuffer.height as isize;
    let height = height as isize;

    let wall_top = screen_height / 2 - height / 2;
    let wall_bottom = screen_height / 2 + height / 2;

    if wall_top >= framebuffer.height as isize || wall_bottom <= 0 {
        return;
    }

    let texture_height = texture.height() as usize;
    let step = texture_height as f32 / height as f32;

    framebuffer.set_current_color(0xFFFFFF);

    for y in 0..height {
        let texture_y = (y as f32 * step) as usize;

        let color = texture.get_pixel(texture_x as u32, texture_y as u32);
        let color_value = ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | (color[2] as u32);

        if wall_top + y >= 0 && (wall_top + y) < framebuffer.height as isize {
            framebuffer.point(x, (wall_top + y) as usize, color_value);
        }
    }
}

fn draw_minimap(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize, minimap_scale: f32) {
    let offset_x = 10;
    let offset_y = 10;

    for j in 0..maze.len() {
        for i in 0..maze[j].len() {
            let x = offset_x + (i as f32 * block_size as f32 * minimap_scale) as usize;
            let y = offset_y + (j as f32 * block_size as f32 * minimap_scale) as usize;
            let size = (block_size as f32 * minimap_scale) as usize;

            if maze[j][i] != ' ' {
                for dy in 0..size {
                    for dx in 0..size {
                        framebuffer.point(x + dx, y + dy, 0x808080);
                    }
                }
            } else {
                for dy in 0..size {
                    for dx in 0..size {
                        framebuffer.point(x + dx, y + dy, 0x000000);
                    }
                }
            }
        }
    }

    let player_x = offset_x + (player.pos.x * minimap_scale) as usize;
    let player_y = offset_y + (player.pos.y * minimap_scale) as usize;

    let player_size = (block_size as f32 * minimap_scale * 0.5) as usize;
    for dy in 0..player_size {
        for dx in 0..player_size {
            framebuffer.point(player_x + dx, player_y + dy, 0xFF0000);
        }
    }
}

fn render(framebuffer: &mut Framebuffer, player: &Player, texture: &image::DynamicImage, maze: &Vec<Vec<char>>, block_size: usize) {
    let screen_width = framebuffer.width;
    let screen_height = framebuffer.height as f32;

    let player_angle_offset = player.a - player.fov / 2.0;
    let fov_step = player.fov / screen_width as f32;

    for i in 0..screen_width {
        let ray_angle = player_angle_offset + fov_step * (i as f32);
        let (distance, hit_x, hit_y) = cast_ray(framebuffer, maze, player, ray_angle, block_size);

        let corrected_distance = distance * (player.a - ray_angle).cos();
        let wall_height = ((screen_height * block_size as f32) / corrected_distance) as usize;

        let texture_x = (hit_x % block_size) * (texture.width() as usize) / block_size;

        draw_wall_slice(framebuffer, i, wall_height, texture, texture_x);
    }

    let minimap_scale = 0.1;
    draw_minimap(framebuffer, player, maze, block_size, minimap_scale);
}

fn display_welcome_message(framebuffer: &mut Framebuffer, window: &mut Window) {
    let welcome_message = "Bienvenido al juego de Alexis";
    let text_color = 0xFFFFFF;
    let background_color = 0x000000;
    let scale = 2;
    let screen_width = framebuffer.width as i32;
    let text_length = welcome_message.len() as i32;
    let block_size = 5 * scale;
    let total_text_width = text_length * block_size * 6;

    let y_position = framebuffer.height as i32 / 2 - block_size as i32 / 2;
    let start_x = screen_width;
    let end_x = -total_text_width as i32;

    for x_offset in (end_x..start_x).rev() {
        framebuffer.clear();
        framebuffer.set_background_color(background_color);
        framebuffer.draw_character_moving(&welcome_message, x_offset, y_position, scale.try_into().unwrap(), text_color);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();

        std::thread::sleep(Duration::from_millis(1));
    }

    std::thread::sleep(Duration::from_secs(2));
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open("./taylor_swift_track.wav").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.set_volume(0.5);
    sink.play();

    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 500;
    let framebuffer_height = 300;
    let frame_delay = Duration::from_millis(66);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner 3D",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0xADD8E6);

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
        move_speed: 5.0,
        rot_speed: 0.1,
    };

    let wall_texture = load_texture("./texturagris.png"); 
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    let mut last_time = Instant::now();
    let mut fps_counter = 0;
    let mut fps_display = 0;

    display_welcome_message(&mut framebuffer, &mut window);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_time).as_secs_f32();

        if elapsed >= 1.0 {
            fps_display = fps_counter;
            fps_counter = 0;
            last_time = current_time;
        }

        fps_counter += 1;

        framebuffer.clear();

        if window.is_key_down(Key::W) {
            player.move_forward(player.move_speed, &maze, block_size);
        }
        if window.is_key_down(Key::S) {
            player.move_backward(player.move_speed, &maze, block_size);
        }
        if window.is_key_down(Key::A) {
            player.rotate_left(player.rot_speed);
        }
        if window.is_key_down(Key::D) {
            player.rotate_right(player.rot_speed);
        }

        render(&mut framebuffer, &player, &wall_texture, &maze, block_size);

        window.set_title(&format!("Maze Runner 3D - FPS: {}", fps_display));

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    sink.sleep_until_end();
}