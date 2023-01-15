// #![allow(dead_code)]

// use std::default;

use ncurses::*;
use std::fs::File;
use std::io::Write;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.col = col;
        self.row = row;
    }
    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none());
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) -> bool {
        let id_curr = self
            .list_curr
            .expect("Can't create elements outside of lists");
        self.label(
            label,
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            },
        );
        false
    }

    fn end_list(&mut self) {
        self.list_curr = None;
    }
    fn end(&mut self) {
        // todo!()
    }

    fn label(&mut self, label: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(label);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
        // todo!()
    }
}

enum Status {
    Todo,
    Done,
}

impl Status {
    fn toggle(&self) -> Self {
        match self {
            Status::Todo => Status::Done,
            Status::Done => Status::Todo,
        }
    }
}

fn list_up(_list: &[String], list_curr: &mut usize) {
    *list_curr = list_curr.saturating_sub(1);
}
fn list_down(list: &Vec<String>, list_curr: &mut usize) {
    if *list_curr + 1 < list.len() {
        *list_curr += 1;
    }
}

fn parse_item(line: &str) -> Option<(Status, &str)> {
    None
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    refresh();

    let mut quit = false;
    let mut todos = vec![
        "Write app".to_string(),
        "Learn rust".to_string(),
        "Get 1 egg".to_string(),
    ];
    let mut todo_curr: usize = 0;
    let mut dones = vec![
        "Wake up".to_string(),
        "Cry".to_string(),
        "Kill Myself".to_string(),
    ];
    let mut done_curr = 1usize;
    let mut focus = Status::Todo;
    let mut ui = Ui::default();
    while !quit {
        erase();
        ui.begin(1, 1);
        {
            match focus {
                Status::Todo => {
                    ui.label("[TODO] DONE  <tab>", REGULAR_PAIR);
                    ui.label("-------------", REGULAR_PAIR);
                    ui.begin_list(todo_curr);
                    for (index, todo) in todos.iter().enumerate() {
                        ui.list_element(&format!("- [ ] {todo}"), index);
                    }
                    ui.end_list();
                }
                Status::Done => {
                    ui.label(" TODO [DONE] <tab>", REGULAR_PAIR);
                    ui.label("-------------", REGULAR_PAIR);
                    ui.begin_list(done_curr);
                    for (index, done) in dones.iter().enumerate() {
                        ui.list_element(&format!("- [x] {done}"), index);
                    }
                    ui.end_list();
                }
            }

        }
        ui.end();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'e' => {
                let mut file = File::create("TODO").unwrap();
                todos.iter().for_each(|todo| {
                    writeln!(file, "TODO: {todo}").unwrap();
                });
                dones.iter().for_each(|done| {
                    writeln!(file, "DONE: {done}").unwrap();
                });
            }
            'w' => match focus {
                Status::Todo => list_up(&todos, &mut todo_curr),
                Status::Done => list_up(&dones, &mut done_curr),
            },
            's' => match focus {
                Status::Todo => list_down(&todos, &mut todo_curr),
                Status::Done => list_down(&dones, &mut done_curr),
            },
            '\n' => match focus {
                Status::Todo => {
                    if todo_curr < todos.len() {
                        dones.push(todos.remove(todo_curr));
                        if todo_curr >= todos.len() && todo_curr > 0 {
                            todo_curr = todos.len() - 1;
                        }
                    }
                }
                Status::Done => {
                    if done_curr < dones.len() {
                        todos.push(dones.remove(done_curr));
                    }
                    if done_curr >= dones.len() && done_curr > 0 {
                        done_curr = dones.len() - 1;
                    }
                }
            },
            '\t' => focus = focus.toggle(),
            _ => {}
        }
    }

    endwin();
}
