/*
一款简单的五子棋终端游戏
Copyright (C) 2023  郝季仁

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.


version 0.0.1:
2023.08.20.18:01
version 0.0.2:
bug [140,144]
*/
use std::fmt;
//use std::fmt::Error;
use std::io;
use std::num::ParseIntError;
//use std::process::Command;

fn win(l1: [u8; 5], l2: [u8; 5]) -> bool {
    if l1 == l2 {
        true
    } else {
        false
    }
}

/*
fn c_m(c: &Point2D) -> [usize; 4] {
    let cx: u8 = c.x;
    let cy: u8 = c.y;
    let max = |mut x: u8| {
        let mut i: u8 = 0;
        loop {
            if x < 9 {
                x += 1;
                i += 1;
                if i == 4 {
                    break x;
                }
            } else {
                break x;
            }
        }
    };
    let min = |mut x: u8| {
        let mut i: u8 = 0;
        loop {
            if x > 0 {
                x -= 1;
                i += 1;
                if i == 4 {
                    break x;
                }
            } else {
                break x;
            }
        }
    };
    [
        max(cx) as usize, //x_max
        min(cx) as usize, //x_min
        max(cy) as usize, //y_max
        min(cy) as usize, //y_min
    ]
}*/

/*
fn range_check(x: u8, y: u8) -> bool {
    // if (c.x > a) | (c.y > b) {true} else {false}
    (x <= 9) & (y <= 9)
}*/

fn clear() {
    // 清除屏幕
    print!("\x1Bc");
}

fn string_to_static_str(s: String) -> &'static str {
    //将`String`转换为`&str`
    Box::leak(s.into_boxed_str())
}
/*
fn stoi(s: &str) -> u8 {
    //字符串转无符号8位整数
    s.trim().parse::<u8>().expect("请输入坐标!")
}*/

fn stoi(s: &str) -> Result<u8, ParseIntError> {
    //字符串转无符号8位整数
    s.trim().parse::<u8>()
}

#[derive(Debug)]
pub struct Point2D {
    // 坐标
    x: u8,
    y: u8,
}
impl Point2D {
    pub fn create() -> Self {
        // 每次循环必须重新初始化`input`, 否则内存中的值不会更改
        loop {
            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");
            /*
            如果想使一个可恢复错误按不可恢复错误处理
            Result 类提供了两个办法:
            `unwrap()` 和 `expect(message: &str)`
            expect 能够向 panic! 宏发送一段指定的错误信息
            */
            let v: Vec<&str> = input.trim().split(',').collect();
            // 移除字符串前后空格后以逗号分隔解析输入坐标
            //println!("{:?},{:?}", stoi(v[0]), stoi(v[1]));
            /*
            return Point2D {
                x: stoi(v[0]),
                y: stoi(v[1]),
            };
            */
            if v.len() != 2 {
                //drop(input);
                println!("请输入坐标!");
                continue;
            }
            let mut p2d: [u8; 2] = [0, 0];
            match stoi(v[0]) {
                Ok(x0) => {
                    //p2d[0] = x0;
                    p2d[0] = x0;
                }
                Err(_) => {
                    println!("请输入坐标!");
                    continue;
                }
            };
            match stoi(v[1]) {
                Ok(y0) => {
                    p2d[1] = y0;
                }
                Err(_) => {
                    println!("请输入坐标!");
                    continue;
                }
            };
            break Point2D {
                x: p2d[0],
                y: p2d[1],
            };
        }
    }
}

/*
struct Game {
    players: [Player; 2],
}
*/

#[derive(Clone)]
struct Player {
    name: String,
    c: u8,
}

//#[derive(Clone)]
pub struct Map<'a> {
    index: [[u8; 10]; 10],
    players: [&'a Player; 2],
}

// 重写`Display` 显示棋盘
impl fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
        println!("  0 1 2 3 4 5 6 7 8 9");
        let mut r: u8 = 0;
        for n in self.index {
            s += string_to_static_str(r.to_string());
            for m in n {
                if m == 1 {
                    s += " @";
                } else if m == 2 {
                    s += " #";
                } else {
                    s += " +";
                }
            }
            s += "\n";
            r += 1;
        }
        write!(f, "{}", s)
    }
}

impl Map<'_> {
    fn goto(&mut self, p: Player) -> bool {
        loop {
            println!("玩家[{}]请输入坐标:", p.name);
            let c: Point2D = Point2D::create();
            if (|c: &Point2D, a: u8, b: u8| {
                if (c.x > a) | (c.y > b) {
                    true
                } else {
                    false
                }
            })(&c, 9, 9)
            {
                println!("超出范围,请重新输入.");
                continue;
            } else if self.index[c.y as usize][c.x as usize] != 0 {
                println!("这里已经有人了.");
                continue;
            } else {
                self.index[c.y as usize][c.x as usize] = p.c;
                clear();
                println!("玩家[{}]进[{},{}].", p.name, c.x, c.y);
                // 判断胜负!

                let l0: [u8; 5] = [p.c, p.c, p.c, p.c, p.c];
                // x_max,x_min,y_max,y_min
                //let lm: [usize; 4] = c_m(&c);
                //println!("{:?}", lm);
                //displacement

                let cx: usize = c.x as usize;
                let cy: usize = c.y as usize;
                //let x_dpm: usize= lm[0] - lm[1];
                //let y_dpm: usize= lm[2] - lm[3];
                for x_dpm in 0..(c.x + 1) {
                    if x_dpm + 4 >= 10 {
                        break;
                    }
                    let l_x: [u8; 5] = [
                        self.index[cy][(x_dpm + 0) as usize],
                        self.index[cy][(x_dpm + 1) as usize],
                        self.index[cy][(x_dpm + 2) as usize],
                        self.index[cy][(x_dpm + 3) as usize],
                        self.index[cy][(x_dpm + 4) as usize],
                    ];
                    //println!("l_x{:?}", l_x);
                    if win(l_x, l0) {
                        return true;
                    }

                    for y_dpm in 0..(c.y + 1) {
                        if y_dpm + 4 >= 10 {
                            break;
                        }
                        let l_y: [u8; 5] = [
                            self.index[(y_dpm + 0) as usize][cx],
                            self.index[(y_dpm + 1) as usize][cx],
                            self.index[(y_dpm + 2) as usize][cx],
                            self.index[(y_dpm + 3) as usize][cx],
                            self.index[(y_dpm + 4) as usize][cx],
                        ];
                        //println!("l_y{:?}", l_y);
                        if win(l_y, l0) {
                            return true;
                        }
                        let l_24: [u8; 5] = [
                            self.index[(y_dpm + 0) as usize][(x_dpm + 0) as usize],
                            self.index[(y_dpm + 1) as usize][(x_dpm + 1) as usize],
                            self.index[(y_dpm + 2) as usize][(x_dpm + 2) as usize],
                            self.index[(y_dpm + 3) as usize][(x_dpm + 3) as usize],
                            self.index[(y_dpm + 4) as usize][(x_dpm + 4) as usize],
                        ];
                        //println!("l_24{:?}", l_24);
                        if win(l_24, l0) {
                            return true;
                        }
                    }

                    if (x_dpm as i8) + 4 > 9 {
                        break;
                    }
                    // y=[4,9],x=[0,5]
                    // array.reverse();
                    let ydpm: [i8; 6] = [9, 8, 7, 6, 5, 4];
                    for y_dpm in ydpm {
                        //loop {
                        let l_31: [u8; 5] = [
                            self.index[(y_dpm - 0) as usize][(x_dpm + 0) as usize],
                            self.index[(y_dpm - 1) as usize][(x_dpm + 1) as usize],
                            self.index[(y_dpm - 2) as usize][(x_dpm + 2) as usize],
                            self.index[(y_dpm - 3) as usize][(x_dpm + 3) as usize],
                            self.index[(y_dpm - 4) as usize][(x_dpm + 4) as usize],
                        ];
                        //println!("l_31{:?}", l_31);
                        if win(l_31, l0) {
                            return true;
                        }
                    }
                }

                // 18446744073709551616 = 2^64
                // 王克松 3-2502
                //println!("{:?}", l_24);
                //println!("{:?}", l_31);
                println!("");
                return false;
            }
        }
    }
}

fn main() {
    let mut n: u8 = 0;

    let p1: Player = Player {
        name: String::from("nmsl@"),
        c: 1,
    };
    let p2: Player = Player {
        name: String::from("cnmd#"),
        c: 2,
    };
    let mut b1: Map = Map {
        index: [[0; 10]; 10],
        players: [&p1, &p2],
    };

    clear();
    println!("开始游戏...");

    loop {
        println!("{:#}", b1);
        if b1.goto(b1.players[0].clone()) {
            println!("{}获胜", p1.name);
            break;
        }
        n += 1;
        println!("{:#}", b1);
        if b1.goto(b1.players[1].clone()) {
            println!("{}获胜", p2.name);
            break;
        }
        n += 1;
        if n >= 100 {
            println!("平局...");
            break;
        }
    }

    println!("{:#}", b1);
}
