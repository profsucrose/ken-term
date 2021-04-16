use std::ops::Add;
use std::process::Command;

use cgmath::vec3;
use glfw::Key;

use crate::opengl::text_renderer::TextRenderer;

const PS1: &str = "KenTerm: (USER:profsucrose)> ";

// TextRenderer wrapper for rendering terminal lines
pub struct Console {
    text_renderer: TextRenderer,
    past_lines: Vec<String>,
    last_line: String,
    shift_pressed: bool
}

impl Console {
    pub fn new(text_renderer: TextRenderer) -> Console {
        Console { text_renderer, past_lines: vec![], last_line: String::new(), shift_pressed: false }
    }

    fn current_line_formatted(&self) -> String {
        let mut line = String::new();
        line.push_str(PS1);
        line.push_str(self.last_line.as_str());
        return line
    }

    pub fn draw_lines(&self) {
        for (i, line) in self.past_lines.iter().enumerate() {
            unsafe { self.text_renderer.render_text(line, 10.0, (i + 1) as f32 * 30.0, 1.0, vec3(1.0, 1.0, 1.0)); }
        }

        unsafe { self.text_renderer.render_text(self.current_line_formatted().as_str(), 10.0, (self.past_lines.len() + 1) as f32 * 30.0, 1.0, vec3(1.0, 1.0, 1.0)); }
    }

    pub fn update_screen_size(&mut self, width: u32, height: u32) {
        self.text_renderer.update_screen_size(width, height);
    }

    pub fn add_line(&mut self, line: String) {
        self.past_lines.push(line);
    }

    pub fn add_to_last_line(&mut self, ch: char) {
        self.last_line.push(ch);
    }

    fn remove_last_char(&mut self) {
        if self.last_line.len() == 0 {
            return;
        }

        self.last_line.remove(self.last_line.len() - 1);
    }

    pub fn shift(&mut self) {
        self.shift_pressed = true;
    }

    pub fn unshift(&mut self) {
        self.shift_pressed = false;
    }

    fn run_command(&mut self) {
        if self.last_line == "clear" {
            self.last_line.clear();
            self.past_lines.clear();
            return;
        }

        if self.last_line.len() == 0 {
            self.past_lines.push(self.current_line_formatted());
            return;
        }

        let mut neofetch = false;
        if self.last_line == "neofetch" {
            neofetch = true;
            self.last_line = String::from("cat /home/profsucrose/Documents/kenfetch");
        }

        let mut words = self.last_line.trim().split_whitespace();

        let mut command = Command::new(words.next().unwrap())
            .args(words)
            .output();
        
        if neofetch {
            self.last_line = String::from("neofetch");
        }

        self.past_lines.push(self.current_line_formatted());
        self.last_line.clear();
        
        match command {
            Ok(result) => {
                let stdout = std::str::from_utf8(&result.stdout).unwrap();
                for line in stdout.split('\n') {
                    self.past_lines.push(line.to_string());
                }
            },
            Err(err) => {
                let stdout = err.to_string();
                for line in stdout.split('\n') {
                    self.past_lines.push(line.to_string());
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        if key == Key::Backspace {
            self.remove_last_char();
            return;
        }

        if key == Key::Enter {
            self.run_command();
            return;
        }

        let ch = match key {
            Key::Space => Some(' '),
            key => {
                let mut ch = key.get_name();
                match ch {
                    Some(ch) => {
                        let mut ch = ch.chars().nth(0).unwrap();
                        if self.shift_pressed {
                            ch = match ch {
                                '1' => '!',
                                '2' => '@',
                                '3' => '#',
                                '4' => '$',
                                '5' => '%',
                                '6' => '^',
                                '7' => '&',
                                '8' => '*',
                                '9' => '(',
                                '0' => ')',
                                '=' => '+',
                                '\'' => '"',
                                '\\' => '|',
                                ',' => '<',
                                '.' => '>',
                                '`' => '~',
                                ch => ch.to_ascii_uppercase()
                            }
                        }
                        Some(ch)
                    },
                    None => None
                }
            }
        };

        if let Some(ch) = ch {
            self.add_to_last_line(ch);
        }
    }
}