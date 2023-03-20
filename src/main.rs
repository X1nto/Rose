use std::time::Duration;

const FRAMES: &[&str] = &[
    "
         *
         *
    ",
    "
         * *
         **
         *
         *
    ",
    "
     * *
       **
         * *
         **
         *
         *
    ",
    "
        *
         *
        *
     * *
       **
         * *
         **
         *
         *
    ",
    "
        *   ******
         * *    *
        * ******
     * *
       **
         * *
         **
         *
         *
    ",
    "
  #    #     #
   #    #   #
     ######
        *   ******
         * *    *
        * ******
     * *
       **
         * *
         **
         *
         *
    ",
    "
    #  #   #
   #  #     #
  #    #     #
   #    #   #
     ######
        *   ******
         * *    *
        * ******
     * *
       **
         * *
         **
         *
         *
    ",
];

fn main() {
    let window = pancurses::initscr();
    pancurses::start_color();
    pancurses::init_pair(0, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    pancurses::init_pair(2, pancurses::COLOR_RED, pancurses::COLOR_BLACK);

    for frame in FRAMES {
        window.clear();
        let lines = frame.split_terminator('\n');
        for line in lines {
            let attrnum = if line.contains('#') {
                2
            } else if line.contains('*') {
                1
            } else {
                0
            };
            let attr = pancurses::COLOR_PAIR(attrnum);
            window.attron(attr);
            window.addstr(line);
            window.addch('\n');
            window.refresh();
            window.attroff(attr);
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    window.getch();
    window.delwin();
}
