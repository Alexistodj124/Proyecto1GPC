use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, player: &Player, a: f32, block_size: usize) -> (f32, usize, usize) {
    let mut d = 0.0;

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();

        let x = player.pos.x + cos;
        let y = player.pos.y + sin;

        let i = (x / block_size as f32) as isize;
        let j = (y / block_size as f32) as isize;
        if i >= 0 && j >= 0 && i < maze[0].len() as isize && j < maze.len() as isize {
            if maze[j as usize][i as usize] != ' ' {
                return (d, x as usize, y as usize);
            }
        } else {
            break;
        }

        d += 10.0;
    }
    (d, player.pos.x as usize, player.pos.y as usize)
}
