#![allow(unused)]

use std::default;

use ncurses::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    key: Option<i32>,
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
    let mut todo_curr: usize = 1;
    let mut dones = vec![
        "Wake up".to_string(),
        "Cry".to_string(),
        "Kill Myself".to_string(),
    ];
    let mut done_curr = 0usize;

    let mut ui = Ui::default();
    while !quit {
        erase();
        ui.begin(1, 1);
        {
            ui.begin_list(todo_curr);
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(&format!("- [ ] {todo}"), index);
            }
            ui.end_list();
            ui.label("--------------------------", REGULAR_PAIR);
            ui.begin_list(0);
            for (index, done) in dones.iter().enumerate() {
                ui.list_element(&format!("- [x] {done}"), index + 1);
            }
            ui.end_list();
        }
        ui.end();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' => {
                if todo_curr > 0 {
                    todo_curr -= 1;
                }
            }
            's' => if todo_curr + 1 < todos.len() {
                todo_curr += 1;
            },
            '\n' => {
                if todo_curr < todos.len() {
                    dones.push(todos.remove(todo_curr));
                }
            },
            _ => {}
        }
    }

    endwin();
}
