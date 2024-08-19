use nalgebra_glm::{Vec2};

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
    pub move_speed: f32,
    pub rot_speed: f32,
}

impl Player {
    pub fn move_forward(&mut self, distance: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_x = self.pos.x + self.a.cos() * distance;
        let new_y = self.pos.y + self.a.sin() * distance;

        let i = (new_x / block_size as f32) as isize;
        let j = (new_y / block_size as f32) as isize;

        if i >= 0 && j >= 0 && i < maze[0].len() as isize && j < maze.len() as isize {
            if maze[j as usize][i as usize] == ' ' {
                self.pos.x = new_x;
                self.pos.y = new_y;
            }
        }
    }

    pub fn move_backward(&mut self, distance: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_x = self.pos.x - self.a.cos() * distance;
        let new_y = self.pos.y - self.a.sin() * distance;

        let i = (new_x / block_size as f32) as isize;
        let j = (new_y / block_size as f32) as isize;

        if i >= 0 && j >= 0 && i < maze[0].len() as isize && j < maze.len() as isize {
            if maze[j as usize][i as usize] == ' ' {
                self.pos.x = new_x;
                self.pos.y = new_y;
            }
        }
    }

    pub fn rotate_left(&mut self, angle: f32) {
        self.a = (self.a - angle) % (2.0 * std::f32::consts::PI);
    }

    pub fn rotate_right(&mut self, angle: f32) {
        self.a = (self.a + angle) % (2.0 * std::f32::consts::PI);
    }
}
