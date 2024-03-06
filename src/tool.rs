use piston_window::*;
use std::io;
use super::*;

//繪製視窗
pub fn draw_window(point: &Vec<Point>, team: &Vec<Vec<usize>>, k_num: &Vec<usize>) {
    println!("draw:\nteam:{:?}\nk_num:{:?}", team, k_num);
    //繪圖
    let mut window: PistonWindow = WindowSettings::new("kmean", [1024, 1024])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            for i in 0..k_num.len() {
                let line_color = [0.0, 0.0, 0.0, 1.0];
                for j in &team[i] {
                    print!("{} ", *j);
                    line(
                        line_color,
                        1.0,
                        [
                            point[*j].x,
                            point[*j].y,
                            point[k_num[i]].x,
                            point[k_num[i]].y,
                        ],
                        c.transform,
                        g,
                    );
                }
                println!();
            }
        });
    }
}
//顯示、讀取鍵盤輸入資料
pub fn read_number(prompt: &str) -> usize {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}
