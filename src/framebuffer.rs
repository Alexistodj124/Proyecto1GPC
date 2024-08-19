pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }
    pub fn draw_character_moving(&mut self, text: &str, x_start: i32, y_start: i32, scale: usize, color: u32) {
        let block_size = 5 * scale as i32; 
        for (i, c) in text.chars().enumerate() {
            let x_offset = x_start + (i as i32 * block_size * 6);
            self.draw_character(c, x_offset, y_start, block_size, color);
        }
    }

    pub fn draw_character(&mut self, c: char, x_start: i32, y_start: i32, block_size: i32, color: u32) {
        let pattern = match c {
            'B' => vec![
                " ### ",
                "#   #",
                " ### ",
                "#   #",
                " ### "
            ],
            'i' => vec![
                "     ",
                "  #  ",
                "  #  ",
                "  #  ",
                "     "
            ],
            'e' => vec![
                " ### ",
                "#    ",
                "#####",
                "#    ",
                " ### "
            ],
            'n' => vec![
                "     ",
                "###  ",
                "#  # ",
                "#   #",
                "#   #"
            ],
            'v' => vec![
                "     ",
                "#   #",
                "#   #",
                " # # ",
                "  #  "
            ],
            'l' => vec![
                " #   ",
                " #   ",
                " #   ",
                " #   ",
                " ### "
            ],
            'o' => vec![
                " ### ",
                "#   #",
                "#   #",
                "#   #",
                " ### "
            ],
            'a' => vec![
                " ### ",
                "#   #",
                "#####",
                "#   #",
                "#   #"
            ],
            'j' => vec![
                "    #",
                "    #",
                "    #",
                "#   #",
                " ### "
            ],
            'u' => vec![
                "     ",
                "#   #",
                "#   #",
                "#   #",
                " ### "
            ],
            'd' => vec![
                " ### ",
                "#   #",
                "#   #",
                "#   #",
                " ### "
            ],
            'm' => vec![
                "     ",
                "## ##",
                "# # #",
                "#   #",
                "#   #"
            ],
            'x' => vec![
                "     ",
                "#   #",
                " # # ",
                " # # ",
                "#   #"
            ],
            's' => vec![
                " ### ",
                "#    ",
                " ### ",
                "    #",
                " ### "
            ],
            ' ' => vec![
                "     ",
                "     ",
                "     ",
                "     ",
                "     "
            ],
            _ => vec![
                "     ",
                "     ",
                "     ",
                "     ",
                "     "
            ],
        };

        for (y, row) in pattern.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' {
                    for dy in 0..block_size {
                        for dx in 0..block_size {
                            self.point(
                                (x_start + (x as i32 * block_size) + dx) as usize,
                                (y_start + (y as i32 * block_size) + dy) as usize,
                                color,
                            );
                        }
                    }
                }
            }
        }
    }
}
