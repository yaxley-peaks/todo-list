use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    initscr();
    noecho();
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    // init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);

    // addstr("Hello world");
    refresh();

    let mut quit = false;
    let todos = vec!["Write app", "Learn rust", "Get 1 egg"];
    let mut todo_curr: usize = 1;
    while !quit {
        for (index, todo) in todos.iter().enumerate() {
            let pair = {
                if todo_curr == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };
            attron(COLOR_PAIR(pair));
            mv(index as i32, 0);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' => {
                if todo_curr > 0 {
                    todo_curr -= 1;
                }
            }
            's' => todo_curr = std::cmp::min(todo_curr + 1, todos.len() - 1),
            _ => {}
        }
    }

    endwin();
}
