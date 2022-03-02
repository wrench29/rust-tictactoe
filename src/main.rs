use crossterm::event::KeyCode;


mod field {
    pub struct Field {
        field: [Player; 9],
        won: bool,
        tie: bool,
        who_won: Player,
        won_combination: [usize; 3],
    }

    #[derive(Copy, Clone)]
    pub enum Player {
        Empty,
        Cross,
        Nought,
    }

    static COMBINATIONS:[[usize; 3]; 8] = [
        [0,1,2],
        [3,4,5],
        [6,7,8],
        [0,3,6],
        [1,4,7],
        [2,5,8],
        [0,4,8],
        [2,4,6],
    ];

    impl std::fmt::Display for Player {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                Player::Cross => write!(f, "Cross"),
                Player::Nought => write!(f, "Nought"),
                Player::Empty => write!(f, "_"),
            }
         }
    }

    impl Default for Field {
        fn default() -> Self {
            Self {
                field: [Player::Empty, Player::Empty, Player::Empty, 
                        Player::Empty, Player::Empty, Player::Empty, 
                        Player::Empty, Player::Empty, Player::Empty],
                won: false,
                tie: false,
                who_won: Player::Empty,
                won_combination: [0,0,0]
            }
        }
    }

    impl Field {
        pub fn get_player(&self, position: (isize, isize)) -> &Player {
            assert!(position.0 > 0 && position.0 < 4, "x out of bounds");
            assert!(position.1 > 0 && position.1 < 4, "y out of bounds");

            &self.field[usize::try_from(((position.0-1) * 3) + (position.1-1)).unwrap()]
        }
        pub fn set_player(&mut self, position: (isize, isize), player: Player) -> bool {
            assert!(position.0 > 0 && position.0 < 4, "x out of bounds");
            assert!(position.1 > 0 && position.1 < 4, "y out of bounds");
            
            self.check_won();

            if !matches!(self.get_player(position), Player::Empty) || self.won {
                return false;
            }
            self.field[usize::try_from(((position.0-1) * 3) + (position.1-1)).unwrap()] = player;
            return true;
        }
        pub fn check_won(&mut self) -> bool {
            if self.won {
                return true;
            }
            if self.tie {
                return false;
            }
            let won: bool;
            let won_player: Player;
            for variant in COMBINATIONS {
                if 
                    matches!(self.field[variant[0]], Player::Cross) &&
                    matches!(self.field[variant[1]], Player::Cross) &&
                    matches!(self.field[variant[2]], Player::Cross) {
                    won_player = Player::Cross;
                } else if 
                    matches!(self.field[variant[0]], Player::Nought) &&
                    matches!(self.field[variant[1]], Player::Nought) &&
                    matches!(self.field[variant[2]], Player::Nought) {
                    won_player = Player::Nought;
                } else {
                    continue;
                }
                won = true;
                self.won = won;
                self.who_won = won_player;
                self.won_combination = variant;
                break;
            }
            return self.won;
        }
        pub fn check_tie(&mut self) -> bool {
            for player in &self.field {
                if matches!(player, Player::Empty) {
                    return false;
                }
            }
            return true;
        }

        pub fn who_won(&self) -> &Player {
            return &self.who_won;
        }

        fn make_cells(&self, select_cell: Option<isize>) -> [String; 27] {
            let mut field_array: [String; 27] = Default::default();
            let mut i = 0;
            let mut real_index = 0;

            let selected_cell: isize;
            match select_cell {
                Some(v) => selected_cell = v,
                None => selected_cell = -1
            }

            for pl in &self.field {
                match pl {
                    Player::Cross => {
                        field_array[i]   = "00  00".to_string();
                        field_array[i+3] = "  00  ".to_string();
                        field_array[i+6] = "00  00".to_string();
                    },
                    Player::Nought => {
                        field_array[i]   = "  00  ".to_string();
                        field_array[i+3] = "00  00".to_string();
                        field_array[i+6] = "  00  ".to_string();
                    },
                    Player::Empty => {
                        field_array[i]   = "      ".to_string();
                        field_array[i+3] = "      ".to_string();
                        field_array[i+6] = "      ".to_string();
                    }
                }

                if real_index == selected_cell {
                    field_array[i] = field_array[i].replace(" ", "#");
                    field_array[i+3] = field_array[i+3].replace(" ", "#");
                    field_array[i+6] = field_array[i+6].replace(" ", "#");
                }

                i += 1;
                real_index += 1;
                if i == 3 {
                    i = 9;
                }
                if i == 12 {
                    i = 18;
                }
            }
            field_array
        }
    
        pub fn make_field(&self, select_cell: Option<(isize, isize)>) -> String {
            let cells: [String; 27];
            
            match select_cell {
                Some(v) => cells = self.make_cells(Some(((v.0-1) * 3) + (v.1-1))),
                None => cells = self.make_cells(None)
            }

            return format!(
                " {} | {} | {} \n {} | {} | {} \n {} | {} | {} \n \
                 ------------------------\n \
                  {} | {} | {} \n {} | {} | {} \n {} | {} | {} \n \
                 ------------------------\n \
                  {} | {} | {} \n {} | {} | {} \n {} | {} | {}", 
                cells[0], cells[1], cells[2], cells[3], cells[4], cells[5],
                cells[6], cells[7], cells[8], cells[9], cells[10], cells[11],
                cells[12], cells[13], cells[14], cells[15], cells[16], cells[17],
                cells[18], cells[19], cells[20], cells[21], cells[22], cells[23],
                cells[24], cells[25], cells[26]);
        }
    }
}

mod ui {
    use tui::{
        backend::CrosstermBackend,
        widgets::{Block, Borders, Paragraph},
        Terminal
    };

    pub struct Ui {
        terminal: tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
        message: String,
        sub_message: String,
        field: String,
    }

    impl Default for Ui {
        fn default() -> Self {
            crossterm::terminal::enable_raw_mode().unwrap();
            let mut stdout = std::io::stdout();
            crossterm::execute!(stdout, 
                crossterm::terminal::EnterAlternateScreen, 
                crossterm::event::EnableMouseCapture).unwrap();
            let backend = CrosstermBackend::new(stdout);

            Self {
                terminal: Terminal::new(backend).unwrap(),
                message: "".to_string(),
                sub_message: "".to_string(),
                field: "".to_string(),
            }
        }
    }

    impl Ui {
        pub fn key_event(&self) -> 
                Option<crossterm::event::KeyEvent> {
            if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                return Some(key);
            }
            return None;
        }
        pub fn set_message(&mut self, message: String) {
            self.message = message;
        }
        pub fn set_sub_message(&mut self, sub_message: String) {
            self.sub_message = sub_message;
        }
        pub fn set_field(&mut self, field: String) {
            self.field = field;
        }
        pub fn render(&mut self) {
            self.terminal.draw(|f| {
                let mut size = f.size();
                size.x = 2;
                size.y = 1;
                size.width = 28;
                size.height = 15;
                let block = Block::default()
                .title("Tic Tac Toe")
                .borders(Borders::ALL);
                
                let paragraph_str: String = format!(" {}\n{}\n {}",
                    self.message,
                    self.field,
                    self.sub_message
                );
                
                let paragraph = Paragraph::new(paragraph_str).block(block);
                
                f.render_widget(paragraph, size);
            }).unwrap();
        }
    }
    impl Drop for Ui {
        fn drop(&mut self) {
            crossterm::terminal::disable_raw_mode().unwrap();
            crossterm::execute!(
                self.terminal.backend_mut(),
                crossterm::terminal::LeaveAlternateScreen,
                crossterm::event::DisableMouseCapture
            ).unwrap();
            self.terminal.show_cursor().unwrap();
        }
    }
}

fn move_selection(current_position: (isize, isize), side: KeyCode) -> (isize, isize) {
    assert!(current_position.0 > 0 && current_position.0 < 4, "x out of bounds");
    assert!(current_position.1 > 0 && current_position.1 < 4, "y out of bounds");

    let mut new_position = current_position;

    match side {
        KeyCode::Up => {
            new_position.0 -= 1;
            if new_position.0 == 0 {
                new_position.0 = 3;
            }
        },
        KeyCode::Down => {
            new_position.0 += 1;
            if new_position.0 == 4 {
                new_position.0 = 1;
            }
        },
        KeyCode::Left => {
            new_position.1 -= 1;
            if new_position.1 == 0 {
                new_position.1 = 3;
            }
        },
        KeyCode::Right => {
            new_position.1 += 1;
            if new_position.1 == 4 {
                new_position.1 = 1;
            }
        },
        _ => {
            panic!("Wrong KeyCode");
        }
    }
    
    return new_position;
}

fn main() {
    let mut ui = ui::Ui::default();
    let mut field = field::Field::default();
    let mut current_position = (1, 1);
    let mut game_phase = 1;
    let mut current_player = field::Player::Cross;

    loop {
        if field.check_won() {
            ui.set_message(format!("{} won!", field.who_won()).to_string());
            game_phase = 2;
        } else if field.check_tie() {
            ui.set_message("Tie!".to_string());
            game_phase = 3;
        } else {
            match current_player {
                field::Player::Cross => ui.set_message("Cross' move".to_string()),
                field::Player::Nought => ui.set_message("Nought's move".to_string()),
                field::Player::Empty => panic!("Player empty")
            }
        }

        if game_phase != 1 {
            ui.set_field(field.make_field(None));
        } else {
            ui.set_field(field.make_field(Some(current_position)));
        }
        ui.render();        

        if game_phase == 1 {
            let key = ui.key_event().unwrap();
            match key.code {
                KeyCode::Up | KeyCode::Down | 
                KeyCode::Left | KeyCode::Right => {
                    current_position = move_selection(current_position, key.code);
                    ui.set_field(field.make_field(Some(current_position)));
                },
                KeyCode::Char(' ') => {
                    if !field.set_player(current_position, current_player) {
                        ui.set_sub_message("Field already selected".to_string());
                    } else {
                        ui.set_sub_message("".to_string());
                        match current_player {
                            field::Player::Cross => current_player = field::Player::Nought,
                            field::Player::Nought => current_player = field::Player::Cross,
                            field::Player::Empty => panic!("Player empty")
                        }
                    }
                },
                _ => {
                    
                }
            }
        } else {
            ui.set_sub_message("Press any key to exit.".to_string());
            ui.render();
            ui.key_event().unwrap();
            break;
        }
    }
}
