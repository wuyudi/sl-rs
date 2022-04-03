mod body;
use body::*;
use libc::*;
use ncurses::*;
use std::env;
#[derive(Default)]
struct Options {
    accident: bool,
    logo: bool,
    fly: bool,
    c51: bool,
}

#[derive(Default, Copy, Clone)]
struct Smokes {
    y: i32,
    x: i32,
    ptrn: usize,
    kind: usize,
}
fn my_mvaddstr(y: i32, x: i32, s: &str) {
    let diff_x = x.abs() as usize;
    if diff_x < s.len() {
        let ss = s[diff_x..].chars();
        ss.enumerate().for_each(|(x, c)| {
            mvaddch(y, x.try_into().unwrap(), c as u32);
        })
    }
}
fn add_smoke(y: i32, x: i32, _sum: &mut usize, s: &mut [Smokes]) {
    const SMOKEPTNS: usize = 16;
    let sum = *_sum;

    let smoke = [
        [
            "(   )", "(    )", "(    )", "(   )", "(  )", "(  )", "( )", "( )", "()", "()", "O",
            "O", "O", "O", "O", " ",
        ],
        [
            "(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)", "(@@)", "(@)", "(@)", "@@", "@@", "@",
            "@", "@", "@", "@", " ",
        ],
    ];
    let eraser = [
        "     ", "      ", "      ", "     ", "    ", "    ", "   ", "   ", "  ", "  ", " ", " ",
        " ", " ", " ", " ",
    ];
    let dy = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let dx = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

    if x % 4 != 0 {
        return;
    }
    for i in 0..sum {
        my_mvaddstr(s[i].y, s[i].x, eraser[s[i].ptrn]);
        s[i].y -= dy[s[i].ptrn];
        s[i].x += dx[s[i].ptrn];
        s[i].ptrn += if s[i].ptrn < SMOKEPTNS - 1 { 1 } else { 0 };
        my_mvaddstr(s[i].y, s[i].x, smoke[s[i].kind][s[i].ptrn]);
    }
    my_mvaddstr(y, x, smoke[sum % 2][0]);
    s[sum].y = y;
    s[sum].x = x;
    s[sum].ptrn = 0;
    s[sum].kind = sum % 2;
    *_sum += 1;
}
fn add_man(y: i32, x: i32) {
    let man = [["", "(O)"], ["Help!", "\\O/"]];
    for i in 0..2 {
        my_mvaddstr(
            y + i,
            x,
            man[((LOGOLENGTH + x) / 12 % 2) as usize][i as usize],
        );
    }
}
fn add_c51<'a>(
    x: i32,
    opt: &'a Options,
    _sum: &'a mut usize,
    s: &mut [Smokes],
) -> Result<(), &'a str> {
    let c51 = [
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH11, C51WH12,
            C51WH13, C51WH14, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH21, C51WH22,
            C51WH23, C51WH24, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH31, C51WH32,
            C51WH33, C51WH34, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH41, C51WH42,
            C51WH43, C51WH44, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH51, C51WH52,
            C51WH53, C51WH54, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH61, C51WH62,
            C51WH63, C51WH64, C51DEL,
        ],
    ];
    let coal = [
        COALDEL, COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10,
        COALDEL,
    ];

    let mut dy = 0;

    if x < -C51LENGTH {
        return Err("err");
    } else {
    }
    let mut y = LINES() / 2 - 5;

    if opt.fly {
        y = (x / 7) + LINES() - (COLS() / 7) - C51HEIGHT;
        dy = 1;
    }
    for i in 0..=C51HEIGHT {
        my_mvaddstr(
            y + i,
            x,
            c51[((C51LENGTH + x) % C51PATTERNS) as usize][i as usize],
        );
        my_mvaddstr(y + i + dy, x + 55, coal[i as usize]);
    }
    if opt.accident {
        add_man(y + 3, x + 45);
        add_man(y + 3, x + 49);
    }
    add_smoke(y - 1, x + C51FUNNEL, _sum, s);
    Ok(())
}
fn add_d51<'a>(
    x: i32,
    opt: &'a Options,
    _sum: &'a mut usize,
    s: &mut [Smokes],
) -> Result<(), &'a str> {
    let d51 = [
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL11, D51WHL12,
            D51WHL13, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL21, D51WHL22,
            D51WHL23, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL31, D51WHL32,
            D51WHL33, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL41, D51WHL42,
            D51WHL43, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL51, D51WHL52,
            D51WHL53, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL61, D51WHL62,
            D51WHL63, D51DEL,
        ],
    ];
    let coal = [
        COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
    ];

    let mut dy = 0;

    if x < -D51LENGTH {
        return Err("err");
    } else {
    }
    let mut y = LINES() / 2 - 5;

    if opt.fly {
        y = (x / 7) + LINES() - (COLS() / 7) - D51HIGHT;
        dy = 1;
    }
    for i in 0..=D51HIGHT {
        my_mvaddstr(
            y + i,
            x,
            d51[((D51LENGTH + x) % D51PATTERNS) as usize][i as usize],
        );
        my_mvaddstr(y + i + dy, x + 53, coal[i as usize]);
    }
    if opt.accident {
        add_man(y + 2, x + 43);
        add_man(y + 2, x + 47);
    }
    add_smoke(y - 1, x + D51FUNNEL, _sum, s);
    Ok(())
}
fn add_sl<'a>(
    x: i32,
    opt: &'a Options,
    _sum: &'a mut usize,
    s: &mut [Smokes],
) -> Result<(), &'a str> {
    let sl = [
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL11, LWHL12, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL21, LWHL22, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL31, LWHL32, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL41, LWHL42, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL51, LWHL52, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL61, LWHL62, DELLN],
    ];

    let coal = [LCOAL1, LCOAL2, LCOAL3, LCOAL4, LCOAL5, LCOAL6, DELLN];

    let car = [LCAR1, LCAR2, LCAR3, LCAR4, LCAR5, LCAR6, DELLN];

    let mut py1 = 0;
    let mut py2 = 0;
    let mut py3 = 0;

    if x < -LOGOLENGTH {
        return Err("err");
    } else {
    }
    let mut y = LINES() / 2 - 3;

    if opt.fly {
        y = (x / 6) + LINES() - (COLS() / 6) - LOGOHEIGHT;
        py1 = 2;
        py2 = 4;
        py3 = 6;
    }
    for i in 0..=LOGOHEIGHT {
        my_mvaddstr(
            y + i,
            x,
            sl[((LOGOLENGTH + x) / 3 % LOGOPATTERNS) as usize][i as usize],
        );
        my_mvaddstr(y + i + py1, x + 21, coal[i as usize]);
        my_mvaddstr(y + i + py2, x + 42, car[i as usize]);
        my_mvaddstr(y + i + py3, x + 63, car[i as usize]);
    }
    if opt.accident {
        add_man(y + 1, x + 14);
        add_man(y + 1 + py2, x + 45);
        add_man(y + 1 + py2, x + 53);
        add_man(y + 1 + py3, x + 66);
        add_man(y + 1 + py3, x + 74);
    }
    add_smoke(y - 1, x + LOGOFUNNEL, _sum, s);
    Ok(())
}

fn option(s: &str, opt: &mut Options) {
    match s.chars().next().unwrap() {
        'a' => opt.accident = true,
        'F' => opt.fly = true,
        'l' => opt.logo = true,
        'c' => opt.c51 = true,
        _ => (),
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut opt = Options::default();
    args.iter().for_each(|s| option(s, &mut opt));
    let mut sum: usize = 0;
    let mut s = [Smokes::default(); 1000];
    initscr();
    unsafe {
        signal(SIGINT, SIG_IGN);
    }
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    nodelay(stdscr(), TRUE == 1);
    leaveok(stdscr(), TRUE == 1);
    scrollok(stdscr(), FALSE == 1);
    for x in 0..(COLS() - 1) {
        if opt.logo {
            add_sl(x, &opt, &mut sum, &mut s).unwrap();
        } else if opt.c51 {
            add_c51(x, &opt, &mut sum, &mut s).unwrap();
        } else {
            add_d51(x, &opt, &mut sum, &mut s).unwrap();
        }
        getch();
        refresh();
        unsafe {
            usleep(40000);
        }
    }
    mvcur(0, COLS() - 1, LINES() - 1, 0);
    endwin();
}
