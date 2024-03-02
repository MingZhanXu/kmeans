extern crate piston_window;

// use piston_window::math::triangle_face;
use piston_window::*;

use rand::Rng;
use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::time::Instant;

use std::io;

use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use std::sync::{Arc, Mutex};
fn main() {
    let seed_num = read_number("seed_num: ");
    let dot_num = read_number("dot_num: ");
    let num = read_number("num: ");
    let max = read_number("max: ");
    let draw_state = read_number("draw_state (0 false): ");

    let (k_num, team, point) = kmeans(seed_num, num, max, dot_num);
    if draw_state != 0 {
        draw_window(seed_num, &point, &team, &k_num);
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}
//點顯示
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
    fn dis(&self, k_point: &Point) -> f64 {
        ((k_point.x - self.x).powf(2.0) + (k_point.y - self.y).powf(2.0)).powf(0.5)
    }
}

//訊息總類
#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    TaskNameMessage(String, usize), //task_name, max
    CodeNameMessage(usize),
    NumMessage(TaskUser, usize), //code_name, max-now
    PointMessage(Vec<Point>),
    TeamMessage((usize, usize, Vec<Vec<usize>>)), //step, num.
    KNumMessage((usize, usize, Vec<usize>)),
    ResetKNumMessage(Vec<usize>),
}
#[derive(Serialize, Deserialize, Debug)]
struct TaskUser {
    code_name: usize,
    num: usize,
}
//隨機產生不重複點
fn generate_point(dot_num: usize) -> Vec<Point> {
    let mut point: Vec<Point> = vec![];
    for _ in 0..dot_num {
        let mut p: Point;
        loop {
            let x = rand::thread_rng().gen_range(0.0..1024 as f64);
            let y = rand::thread_rng().gen_range(0.0..1024 as f64);
            p = Point { x, y };
            if !point.contains(&p) {
                break;
            }
        }
        point.push(p);
    }
    point
}
//隨機點中心
fn random_center(seed_num: usize, dot_num: usize) -> Vec<usize> {
    //隨機群中心
    let mut k_num: Vec<usize> = vec![];
    for _ in 0..seed_num {
        let mut num;
        loop {
            num = rand::thread_rng().gen_range(0..dot_num);
            //判斷是否包含
            if !k_num.contains(&num) {
                break;
            }
        }
        k_num.push(num);
    }
    k_num
}
//元素分群
fn cluster(point: &Vec<Point>, k_num: &Vec<usize>, num: usize, max: usize) -> Vec<Vec<usize>> {
    let dot_num = point.len();
    let mut dot_range = dot_num / max;
    let start = dot_range * num;
    if num == max - 1 {
        dot_range = dot_num - start;
    }
    let point_t = &point[start..start + dot_range];
    let seed_num = k_num.len();
    let mut team: Vec<Vec<usize>> = vec![vec![]; seed_num];
    for i in 0..dot_range {
        let mut mid_dis = f64::MAX;
        let mut flag: usize = 0;
        for j in 0..seed_num {
            let distant = point_t[i].dis(&point[k_num[j]]);
            if distant < mid_dis {
                mid_dis = distant;
                flag = j;
            }
        }
        team[flag].push(i);
    }
    team
}
//找新中心
//team[編號]
fn re_seed(point: &Vec<Point>, team: &Vec<Vec<usize>>, num: usize, max: usize) -> Vec<usize> {
    let seed_num = team.len();
    //缺少錯誤處裡seed_num < max
    let mut seed_rang = seed_num / max;
    let start = seed_rang * num;
    if num == max - 1 {
        seed_rang = seed_num - start;
    }
    let mut k_num: Vec<usize> = vec![];
    let mut cluster: &Vec<usize>;
    for _i in start..start + seed_rang {
        cluster = &team[_i];
        if !cluster.is_empty() {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for &index in cluster {
                sum_x += point[index].x;
                sum_y += point[index].y;
            }
            let new_center_x = sum_x / cluster.len() as f64;
            let new_center_y = sum_y / cluster.len() as f64;
            let new_center_point = Point {
                x: new_center_x,
                y: new_center_y,
            };
            let mut min_distance = f64::MAX;
            let mut new_center_index = 0;
            for (_j, &index) in cluster.iter().enumerate() {
                let distance = point[index].dis(&new_center_point);
                if distance < min_distance {
                    min_distance = distance;
                    new_center_index = index;
                }
            }
            k_num.push(new_center_index);
        }
    }
    k_num
}
//執行kmeans
fn kmeans(
    seed_num: usize,
    num: usize,
    max: usize,
    dot_num: usize,
) -> (Vec<usize>, Vec<Vec<usize>>, Vec<Point>) {
    let point: Arc<Mutex<Vec<Point>>> = Arc::new(Mutex::new(Vec::new()));
    let team: Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(Vec::new()));
    let k_num: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
    let _range: usize = seed_num / max;
    let _start: usize = num * _range;
    let _end: usize = if num == max - 1 {
        seed_num - 1
    } else {
        _start + _range
    };

    let reset_point = Arc::clone(&point);
    let reset_k_num = Arc::clone(&k_num);
    if num == 0 {
        let mut tmp_point = reset_point.lock().unwrap();
        *tmp_point = generate_point(dot_num); //隨機產生point

        let mut tmp_k_num = reset_k_num.lock().unwrap();
        *tmp_k_num = random_center(seed_num, dot_num); //取得k_num
    }

    let thread_point = Arc::clone(&point);
    let thread_k_num = Arc::clone(&k_num);
    let thread_team = Arc::clone(&team);
    //啟動接收訊息
    let handle = thread::spawn(move || {
        let mut receive_port = 8888;
        let mut _receive_socket = None;
        loop {
            match UdpSocket::bind(format!("127.0.0.1:{}", receive_port)) {
                Ok(socket) => {
                    println!("Successfully bound to port {}", receive_port);
                    _receive_socket = Some(socket);
                    break;
                }
                Err(_) => {
                    println!("Failed to bind to port {}, trying next port", receive_port);
                    receive_port += 1;
                }
            }
        }
        let socket_get = _receive_socket.expect("Failed to bind socket");
        let socket_send = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        // socket_send.set_broadcast(true).expect("Failed to set broadcast"); //廣播模式
        let mut buf = vec![0u8; 1024 * 1024].into_boxed_slice();
        let mut team_list: Vec<Vec<Vec<usize>>> = Vec::new();
        let mut k_num_list: Vec<Vec<usize>> = Vec::new();
        let mut k_num_flag: usize = 0;
        let mut last_k_num: Vec<usize> = Vec::new();
        for _i in 0..seed_num {
            k_num_list.push(Vec::new());
        }
        let mut team_flag: usize = 0;
        //初始化team
        for _i in 0..max {
            team_list.push(Vec::new());
        }
        let mut point_flag = false;
        let mut step_now = 0;
        let start_time: Instant = Instant::now();
        let mut id: TaskUser = TaskUser {
            code_name: 0,
            num: 0,
        };
        let mut user_list: Vec<usize> = Vec::new();
        let mut user_list_flag: usize = 0;
        let mut max_user: usize = 0;
        loop {
            if let Ok((size, _addr)) = socket_get.recv_from(&mut buf) {
                let msg_type: MessageType =
                    serde_json::from_slice(&buf[..size]).expect("Failed to deserialize message");
                match msg_type {
                    MessageType::TaskNameMessage(get_task, get_max) => {
                        println!("get task:{}, max:{}", get_task, get_max);
                        max_user = get_max;
                        user_list = vec![0; max_user];
                        if num != 0 {
                            id.code_name =
                                rand::thread_rng().gen_range(1..std::usize::MAX as usize);
                            let msg_type = MessageType::CodeNameMessage(id.code_name);
                            send_message(&socket_send, msg_type);
                        }
                        if num == 0 {
                            id = TaskUser {
                                code_name: 0,
                                num: 0,
                            };
                            let msg_type = MessageType::CodeNameMessage(id.code_name);
                            send_message(&socket_send, msg_type);
                        }
                    }
                    MessageType::CodeNameMessage(get_code_name) => {
                        println!("get code_name:{}", get_code_name);
                        if num == 0 {
                            if user_list_flag < max_user {
                                let msg_type = MessageType::NumMessage(
                                    TaskUser {
                                        code_name: get_code_name,
                                        num: user_list_flag,
                                    },
                                    max - user_list_flag - 1,
                                );
                                send_message(&socket_send, msg_type);
                                user_list_flag += 1;
                                if user_list_flag == max_user {
                                    let msg_type =
                                        MessageType::PointMessage(generate_point(dot_num)); //產生隨機點
                                    send_message(&socket_send, msg_type);
                                }
                            } else {
                                println!("Warning: user_list_flag >= max - 1 (get_code_name)");
                            }
                        }
                    }
                    MessageType::NumMessage(get_task_user, get_user_t) => {
                        if get_task_user.code_name == id.code_name && get_user_t < max_user {
                            id.num = get_task_user.num;
                        }
                        user_list[get_task_user.num] = get_task_user.code_name;
                    }
                    //初始化Point
                    MessageType::PointMessage(points) => {
                        println!("reset point step_now:{}", step_now);
                        if point_flag == false {
                            let mut point = thread_point.lock().unwrap();
                            point_flag = true;
                            *point = points.clone();
                            step_now = 1;
                            //隨機中心點
                            if num == 0 {
                                let msg_type =
                                    MessageType::ResetKNumMessage(random_center(seed_num, dot_num)); //發送初始中心點
                                send_message(&socket_send, msg_type);
                            }
                        } else {
                            println!("無法接收point，需等到處理完畢");
                        }
                    }
                    //初始化k_num
                    MessageType::ResetKNumMessage(get_k_num) => {
                        println!("reset k_num step_now:{}", step_now);
                        if point_flag == true {
                            let mut k_num = thread_k_num.lock().unwrap();
                            *k_num = get_k_num.clone();
                            last_k_num = k_num.clone();
                            let point = thread_point.lock().unwrap();
                            team_list[num] = cluster(&point, &k_num, num, max); //計算team
                            step_now += 1;
                            let msg_type =
                                MessageType::TeamMessage((step_now, num, team_list[num].clone())); //發送team
                            send_message(&socket_send, msg_type);
                        } else {
                            println!("無法接收k_num，需等到處理完畢");
                        }
                    }
                    MessageType::TeamMessage((get_step, get_num, get_team)) => {
                        println!("received team step_now:{}", step_now);
                        if point_flag == true && get_step == step_now && team_flag < max {
                            team_list[get_num] = get_team.clone();
                            team_flag += 1;
                            if team_flag == max {
                                team_flag = 0;
                                let mut team = thread_team.lock().unwrap();
                                let point = thread_point.lock().unwrap();
                                team.clear();
                                for _i in 0..seed_num {
                                    team.push(Vec::new());
                                }
                                for i in 0..max {
                                    for j in 0..seed_num {
                                        team[j].extend(&team_list[i][j]);
                                    }
                                }
                                //計算並發送k_num
                                k_num_list[num] = re_seed(&point, &team, num, max);
                                step_now += 1;
                                k_num_flag = 0;
                                let msg_type = MessageType::KNumMessage((
                                    step_now,
                                    num,
                                    k_num_list[num].clone(),
                                )); //發送中心點
                                send_message(&socket_send, msg_type);
                            }
                        } else {
                            println!("Please input point. --get_team");
                        }
                    }
                    MessageType::KNumMessage((get_step, get_num, get_k_num)) => {
                        println!("received k_num step_now:{}", step_now);
                        if point_flag == true && get_step == step_now && k_num_flag < max {
                            k_num_list[get_num] = get_k_num.clone();
                            k_num_flag += 1;
                            if k_num_flag == max {
                                k_num_flag = 0;
                                let mut k_num = thread_k_num.lock().unwrap();
                                k_num.clear();
                                for i in &k_num_list {
                                    k_num.extend(i);
                                }
                                if *k_num == last_k_num {
                                    let end_time: Instant = Instant::now();
                                    let elapsed_time = end_time - start_time;
                                    println!("\n{}ms", elapsed_time.as_millis());
                                    break;
                                }
                                last_k_num = k_num.clone();
                                let point = thread_point.lock().unwrap();
                                team_list[num] = cluster(&point, &k_num, num, max); //計算team
                                step_now += 1;
                                let msg_type = MessageType::TeamMessage((
                                    step_now,
                                    num,
                                    team_list[num].clone(),
                                ));
                                send_message(&socket_send, msg_type);
                            }
                        } else {
                            println!("Please input point. --get_k_num");
                            println!("{} {}", k_num_flag, max);
                        }
                    }
                }
            }
        }
    });

    thread::sleep(Duration::from_secs(2)); // 等待接收线程启动
    if num == 0 {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        let task = rand::thread_rng().gen_range(0..std::usize::MAX as usize);
        let msg_type = MessageType::TaskNameMessage(task.to_string(), max); //產生隨機點
        let serialized_msg = serde_json::to_string(&msg_type).expect("Failed to serialize message");
        // for i in serialized_msg.chars(){
        //     socket.send_to(i.to_string().as_bytes(), "127.0.0.1:8888").expect("Failed to send message");
        // }
        socket
            .send_to(serialized_msg.as_bytes(), "127.0.0.1:8888")
            .expect("Failed to send message");
    }

    handle.join().unwrap();
    let out_point = point.lock().unwrap();
    let teams = team.lock().unwrap();
    let k_nums = k_num.lock().unwrap();
    // println!("Final result: {:?}", *out_point);
    (k_nums.clone(), teams.clone(), out_point.clone())
}
//繪製視窗
fn draw_window(seed_num: usize, point: &Vec<Point>, team: &Vec<Vec<usize>>, k_num: &Vec<usize>) {
    //繪圖
    let mut window: PistonWindow = WindowSettings::new("kmean", [1024, 1024])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            for i in 0..seed_num {
                let line_color = [0.0, 0.0, 0.0, 1.0];
                for j in &team[i] {
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
            }
        });
    }
}

fn read_number(prompt: &str) -> usize {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}

fn send_message(socket_send: &UdpSocket, msg_type: MessageType) {
    let serialized_msg = serde_json::to_string(&msg_type).expect("Failed to serialize message");
    socket_send
        .send_to(serialized_msg.as_bytes(), "127.0.0.1:8888")
        .expect("Failed to send message");
}
