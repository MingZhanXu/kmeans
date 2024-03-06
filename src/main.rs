extern crate piston_window;

// use piston_window::math::triangle_face;

use std::time::Instant;


use std::thread;
use std::time::Duration;

use std::sync::{Arc, Mutex};

mod kmeans_struct;
use crate::kmeans_struct::*;
mod udp_transmission;
use crate::udp_transmission::*;
mod kmeans;
use crate::kmeans::*;
mod tool;
use crate::tool::*;
const TASK_PUBLLISHER: usize = 0;
fn main() {
    let num = read_number("num(0為發布者 >0為運算者): ");
    match num {
        TASK_PUBLLISHER => {
            let max = read_number("max(總運算者人數): ");
            let seed_num = read_number("seed_num(中心點數): ");
            let dot_num = read_number("dot_num(總點座標數): ");
            let draw_state = read_number("draw_state (是否繪畫>0/0): ");
            let (k_num, team, point) = kmeans(seed_num, num, max, dot_num);
            // println!("output:\nteam:{:?}\nk_num:{:?}", team, k_num);
            if draw_state != 0 {
                draw_window(&point, &team, &k_num);
            }
        }
        _ => {
            kmeans(1, 1, 1, 1);
        }
    }
}


//執行kmeans
fn kmeans(
    seed_num_temp: usize,
    num: usize,
    max: usize,
    dot_num: usize,
) -> (Vec<usize>, Vec<Vec<usize>>, Vec<Point>) {
    let point: Arc<Mutex<Vec<Point>>> = Arc::new(Mutex::new(Vec::new()));
    let k_num: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
    let team: Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(Vec::new()));

    let thread_point = Arc::clone(&point);
    let thread_k_num = Arc::clone(&k_num);
    let thread_team = Arc::clone(&team);
    //啟動接收訊息
    let handle = thread::spawn(move || {
        let socket_get = get_port(8888).expect("Failed to bind socket");
        let socket_send = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        // socket_send.set_broadcast(true).expect("Failed to set broadcast"); //廣播模式
        let mut _buf = vec![0u8; 1024 * 1024].into_boxed_slice();
        let mut size: usize;
        let mut team_list: Vec<Vec<Vec<usize>>> = Vec::new();
        let mut team_flag: usize = 0;
        let mut k_num_list: Vec<Vec<usize>> = Vec::new();
        let mut k_num_flag: usize = 0;
        let mut seed_num: usize = 0;
        let mut last_k_num: Vec<usize> = Vec::new();
        let mut point_flag = false;
        let mut user_list: Vec<usize> = Vec::new();
        let mut user_list_flag: usize = 0;
        let mut max_user: usize = 0;
        let ports = vec!["127.0.0.1:8888", "127.0.0.1:8889", "127.0.0.1:8890"];
        let mut step_now = 0;
        let mut start_time: Instant = Instant::now();
        let mut user_id: TaskUser = TaskUser {
            code_name: 0,
            num: 0,
        };
        // let mut my_addr: &str;
        loop {
            // println!("準備接收訊息");
            (_buf, size) = receive_message(&socket_get);
            let msg_type: MessageType =
            serde_json::from_slice(&_buf[..size]).expect("Failed to deserialize message");
            match msg_type {
                //接收到任務
                MessageType::TaskNameMessage(get_task, get_max) => {
                    println!("get task:{}, max:{}", get_task, get_max);
                    println!("num: {}", num);
                    max_user = get_max;
                    user_list = vec![1; max_user];
                    team_list.resize(max_user, Vec::new());
                    match num{
                        TASK_PUBLLISHER => {
                            println!("發布者");
                        }
                        _ => {
                            println!("接取者");
                            user_id.code_name = rand::thread_rng().gen_range(2..std::usize::MAX as usize);
                        }
                    }
                    let msg_type = MessageType::CodeNameMessage(user_id.code_name);
                    send_message(&socket_send, msg_type, ports.clone());
                }
                //接收到使用者代號  
                MessageType::CodeNameMessage(get_code_name) => {
                    println!("get code_name:{}", get_code_name);
                    match num{
                        //任務發布者
                        TASK_PUBLLISHER => {
                            if user_list_flag < max_user {
                                user_list[user_list_flag] = get_code_name;
                                user_list_flag += 1;
                                println!("user_list_flag:{} max_user:{}",user_list_flag ,max_user);
                                if user_list_flag == max_user{
                                    let msg_type = MessageType::NumMessage(user_list.clone());
                                    send_message(&socket_send, msg_type, ports.clone());
                                    thread::sleep(Duration::from_secs(1));
                                    let msg_type =
                                        MessageType::PointMessage(generate_point(dot_num)); //產生隨機點
                                    send_message(&socket_send, msg_type, ports.clone());
                                }
                            } else {
                                println!("Warning: User capacity reached. --CodeNameMessage\n\tuser_list_flag:{}\t\tmax_user:{}", user_list_flag, max_user);
                            }
                        }
                        //其餘
                        _ => {
                            continue;
                        }
                    }
                }
                //接收到使用者代號與順序
                MessageType::NumMessage(get_user_list) => {
                    println!("set user_list");
                    println!("{:?}", get_user_list);
                    user_list = get_user_list;
                    for (num, &data) in user_list.iter().enumerate(){
                        if data == user_id.code_name{
                            user_id.num = num;
                            println!("編號為: {}", user_id.num);
                        }
                    }
                }
                //Point
                MessageType::PointMessage(points) => {
                    println!("reset point step_now:{}", step_now);
                    if point_flag == false {
                        let mut point = thread_point.lock().unwrap();
                        point_flag = true;
                        *point = points.clone();
                        step_now = 1;
                        //隨機中心點
                        if user_id.num == 0 {
                            println!("發送初始中心點");
                            let mut k_num = thread_k_num.lock().unwrap();
                            *k_num = random_center(seed_num_temp, dot_num);
                            let msg_type =
                                MessageType::ResetKNumMessage(k_num.clone()); //發送初始中心點
                            send_message(&socket_send, msg_type, ports.clone());
                        }
                    } else {
                        println!("無法接收point，需等到處理完畢");
                    }
                }
                //初始化k_num
                MessageType::ResetKNumMessage(get_k_num) => {
                    println!("reset k_num step_now:{}", step_now);
                    if user_id.num == TASK_PUBLLISHER{
                        start_time = Instant::now();
                        println!("****Set start_time");
                    }
                    if point_flag == true {
                        let mut k_num = thread_k_num.lock().unwrap();
                        *k_num = get_k_num.clone();
                        last_k_num = k_num.clone();
                        seed_num = k_num.len();
                        // println!("重設k_num_list; k_num_list.len():", k_num_list.len());
                        k_num_list.resize(seed_num, Vec::new());
                        let point = thread_point.lock().unwrap();
                        team_list[user_id.num] = cluster(&point, &k_num, user_id.num, max_user); //計算team
                        step_now += 1;
                        let msg_type =
                            MessageType::TeamMessage((step_now, user_id.num, team_list[user_id.num].clone())); //發送team
                        println!("-------add step; step_now:{}\n", step_now);
                        send_message(&socket_send, msg_type, ports.clone());
                    } else {
                        println!("無法接收k_num，需等到處理完畢");
                    }
                }
                //群資料
                MessageType::TeamMessage((get_step, get_num, get_team)) => {
                    println!("received team step_now:{}", step_now);
                    // println!("get_team:{:?}", get_team);
                    println!("get_step:{} step_now:{}", get_step, step_now);
                    if point_flag == true && get_step == step_now && team_flag < max_user {
                        println!("team_flag:{}", team_flag);
                        team_list[get_num] = get_team.clone();
                        // println!("get_num:{}\nget_team:{:?}", get_num, get_team.clone());
                        team_flag += 1;
                        println!("team_flag:{} max_user:{}", team_flag, max_user);
                        if team_flag == max_user {
                            team_flag = 0;
                            let mut team = thread_team.lock().unwrap();
                            let point = thread_point.lock().unwrap();
                            team.clear();
                            for _i in 0..seed_num {
                                team.push(Vec::new());
                            }
                            // println!("team_list:{:?}", team_list);
                            for i in 0..max_user {
                                println!("team_list[{}].len():{}", i, team_list[i].len());
                                for j in 0..seed_num {
                                    //j會超出
                                    team[j].extend(&team_list[i][j]);
                                }
                            }
                            // println!("team:\n{:?}", team);
                            //計算並發送k_num
                            // println!("{} {}",k_num_list.len(), user_id.num);
                            k_num_list.resize(seed_num, Vec::new());
                            k_num_list[user_id.num] = re_seed(&point, &team, user_id.num, max_user);
                            step_now += 1;
                            k_num_flag = 0;
                            let msg_type = MessageType::KNumMessage((
                                step_now,
                                user_id.num,
                                k_num_list[user_id.num].clone(),
                            )); //發送中心點
                            send_message(&socket_send, msg_type, ports.clone());
                        }
                    } else {
                        println!("Please input point. --get_team");
                    }
                }
                //中心點
                MessageType::KNumMessage((get_step, get_num, get_k_num)) => {
                    println!("received k_num step_now:{}", step_now);
                    if point_flag == true && get_step == step_now && k_num_flag < max_user {
                        k_num_list[get_num] = get_k_num.clone();
                        k_num_flag += 1;
                        println!("{} {}", k_num_flag, seed_num);
                        if k_num_flag == max_user {
                            k_num_flag = 0;
                            let mut k_num = thread_k_num.lock().unwrap();
                            k_num.clear();
                            for i in &k_num_list {
                                k_num.extend(i);
                            }
                            // println!("knum:\n{:?}", k_num);
                            if *k_num == last_k_num {
                                if user_id.num == TASK_PUBLLISHER{
                                    let end_time: Instant = Instant::now();
                                    let elapsed_time = end_time - start_time;
                                    println!("\n{}ms", elapsed_time.as_millis());
                                }
                                break;
                            }
                            last_k_num = k_num.clone();
                            let point = thread_point.lock().unwrap();
                            team_list[user_id.num] = cluster(&point, &k_num, user_id.num, max_user); //計算team
                            step_now += 1;
                            let msg_type = MessageType::TeamMessage((
                                step_now,
                                user_id.num,
                                team_list[user_id.num].clone(),
                            ));
                            send_message(&socket_send, msg_type, ports.clone());
                        }
                    } else {
                        println!("Please input point. --get_k_num");
                        println!("{} {}", k_num_flag, seed_num);
                    }
                }
                //
                MessageType::HandshakingMessage((get_str, get_type)) => {
                    println!("get_str:\"{}\" get:type:{}", get_str, get_type);
                }
            }
        }
    });

    thread::sleep(Duration::from_secs(2)); // 等待接收线程启动
    if num == TASK_PUBLLISHER {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        let task = rand::thread_rng().gen_range(0..std::usize::MAX as usize);
        let msg_type = MessageType::TaskNameMessage(task.to_string(), max); //產生隨機點
        let ports = vec!["127.0.0.1:8888", "127.0.0.1:8889", "127.0.0.1:8890"];
        send_message(&socket, msg_type, ports.clone());
        thread::sleep(Duration::from_secs(1));
    }

    handle.join().unwrap();
    let out_point = point.lock().unwrap();
    let teams = team.lock().unwrap();
    let k_nums = k_num.lock().unwrap();
    // println!("Final result: {:?}", *out_point);
    (k_nums.clone(), teams.clone(), out_point.clone())
}
