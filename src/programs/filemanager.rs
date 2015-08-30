use programs::common::*;

pub struct FileManager {
    window: Box<Window>,
    files: Vec<String>,
    selected: isize
}

impl FileManager {
    pub fn new() -> FileManager {
        let mut size = Size::new(0, 0);

        let mut files: Vec<String> = Vec::new();

        let mut resource = URL::from_string("file:///".to_string()).open();

        let mut vec: Vec<u8> = Vec::new();
        resource.read_to_end(&mut vec);

        for file in String::from_utf8(&vec).split("\n".to_string()){
            if size.width < (file.len() + 1) * 8 {
                size.width = (file.len() + 1) * 8 ;
            }
            files.push(file.clone());
        }

        if size.height < files.len() * 16 {
            size.height = files.len() * 16;
        }

        let mut ret = FileManager {
            window: Window::new(Point::new((rand() % 400 + 50) as isize, (rand() % 300 + 50) as isize), size, String::from_str("File Manager")),
            files: files,
            selected: -1
        };

        ret.draw_content();

        return ret;
    }

    fn draw_content(&mut self){
        let content = &self.window.content;

        content.set(Color::new(0, 0, 0));

        let mut i = 0;
        let mut row = 0;
        for string in self.files.iter() {
            let mut col = 0;
            for c in string.chars() {
                if c == '\n' {
                    col = 0;
                    row += 1;
                }else if c == '\t' {
                    col += 8 - col % 8;
                }else{
                    let color;
                    if i == self.selected {
                        color = Color::new(128, 128, 128);
                    }else{
                        color = Color::new(255, 255, 255);
                    }

                    if col < content.width / 8 && row < content.height / 16 {
                        content.char(Point::new(8*col as isize, 16*row as isize), c, color);
                        col += 1;
                    }
                }
                if col >= content.width / 8 {
                    col = 0;
                    row += 1;
                }
            }
            row += 1;
            i += 1;
        }

        content.flip();

        RedrawEvent {
            redraw: REDRAW_ALL
        }.to_event().trigger();
    }
}

impl SessionItem for FileManager {
    fn main(&mut self, url: URL){
        loop {
            match self.window.poll() {
                EventOption::Key(key_event) => {
                    if key_event.pressed {
                        match key_event.scancode {
                            0x01 => break,
                            0x47 => self.selected = 0,
                            0x48 => if self.selected > 0 {
                                        self.selected -= 1;
                                    },
                            0x4F => self.selected = self.files.len() as isize - 1,
                            0x50 => if self.selected < self.files.len() as isize - 1 {
                                        self.selected += 1;
                                    },
                            _ => ()
                        }
                        match key_event.character {
                            '\0' => (),
                            '\n' => {
                                if self.selected >= 0 && self.selected < self.files.len() as isize {
                                    match self.files.get(self.selected as usize) {
                                        Option::Some(file) => OpenEvent{ url_string: "file:///".to_string() + file.clone() }.trigger(),
                                        Option::None => ()
                                    }
                                }
                            },
                            _ => {
                                let mut i = 0;
                                for file in self.files.iter() {
                                    if file[0] == key_event.character {
                                        self.selected = i;
                                        break;
                                    }
                                    i += 1;
                                }
                            }
                        }

                        self.draw_content();
                    }
                },
                EventOption::Mouse(mouse_event) => {
                    if ! self.window.shaded {
                        let mut i = 0;
                        let mut row = 0;
                        for file in self.files.iter() {
                            let mut col = 0;
                            for c in file.chars() {
                                if c == '\n' {
                                    col = 0;
                                    row += 1;
                                }else if c == '\t' {
                                    col += 8 - col % 8;
                                }else{
                                    if col < self.window.size.width / 8 && row < self.window.size.height / 16 {
                                        let point = Point::new(self.window.point.x + 8*col as isize, self.window.point.y + 16*row as isize);
                                        if mouse_event.x >= point.x && mouse_event.x < point.x + 8 && mouse_event.y >= point.y && mouse_event.y < point.y + 16 {
                                            self.selected = i;
                                        }
                                        col += 1;
                                    }
                                }
                                if col >= self.window.size.width / 8 {
                                    col = 0;
                                    row += 1;
                                }
                            }
                            row += 1;
                            i += 1;
                        }
                    }

                    self.draw_content();
                },
                _ => sys_yield()
            }
        }
    }
}
