use ncurses::*;
fn main() {
    initscr();
    // addstr("Hello world");
    refresh();

    let mut quit = false;
    let mut todos = vec!["Write app", "Learn rust", "Get 1 egg"];

    while !quit {
        for (row, todo) in todos.iter().enumerate() {
            mv(row as i32, 0);
            addstr(*todo);
        }
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }

    endwin();
}
