use rand::Rng;
use std::fmt;
use std::cmp::{Eq, PartialEq};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskUser {
    pub code_name: usize,
    pub num: usize,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
//點顯示
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}
//點比較
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}
//計算點差距
impl Point {
    pub fn dis(&self, k_point: &Point) -> f64 {
        ((k_point.x - self.x).powf(2.0) + (k_point.y - self.y).powf(2.0)).powf(0.5)
    }
}
pub fn generate_point(dot_num: usize) -> Vec<Point> {
    let mut point: Vec<Point> = vec![];
    for _ in 0..dot_num {
        let mut p: Point;
        loop {
            let x = rand::thread_rng().gen_range(0.0..1024.0 as f64);
            let y = rand::thread_rng().gen_range(0.0..1024.0 as f64);
            p = Point { x, y };
            if !point.contains(&p) {
                break;
            }
        }
        point.push(p);
    }
    println!("隨機點產生完畢");
    point
}

//訊息總類
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    TaskNameMessage(String, usize), //task_name, max
    CodeNameMessage(usize),
    NumMessage(Vec<usize>), //code_name
    PointMessage(Vec<Point>),
    TeamMessage((usize, usize, Vec<Vec<usize>>)), //step, num.
    KNumMessage((usize, usize, Vec<usize>)),
    ResetKNumMessage(Vec<usize>),
    HandshakingMessage((String, usize)), //Port, MessageType(now_step)
}