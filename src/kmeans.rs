use crate::kmeans_struct::*;
/// 產生隨機中心點
pub fn random_center(seed_num: usize, dot_num: usize) -> Vec<usize> {
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
/// 將點位分群
pub fn cluster(point: &Vec<Point>, k_num: &Vec<usize>, num: usize, max: usize) -> Vec<Vec<usize>> {
    let dot_num = point.len();
    let mut dot_range = dot_num / max;
    let start = dot_range * num;
    if num == max - 1 {
        dot_range = dot_num - start;
    }
    let seed_num = k_num.len();
    let mut team: Vec<Vec<usize>> = vec![vec![]; seed_num];
    for i in start..start + dot_range {
        let mut mid_dis = f64::MAX;
        let mut flag: usize = 0;
        for j in 0..seed_num {
            let distant = point[i].dis(&point[k_num[j]]);
            if distant < mid_dis {
                mid_dis = distant;
                flag = j;
            }
        }
        team[flag].push(i);
    }
    team
}
/// 尋找新的群中心
///
/// # 變數說明
///
/// * `point`: 一個包含所有點的向量。
/// * `team`: 一個包含所有群的向量，每個群都包含多個中心點的向量。
/// * `num`: 編號。
/// * `max`: 運算人數。
///
/// # 使用範例
///
/// ```
/// let point = vec![Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }, Point { x: 5.0, y: 6.0 }, Point { x: 8.0, y: 8.0}];
/// let team = vec![vec![0, 1, 2], vec![3]];
/// let num = 1;
/// let max = 1;
/// let result = re_seed(&point, &team, num, max);
/// assert_eq!(result, vec![1, 3]);
/// ```
///
pub fn re_seed(point: &Vec<Point>, team: &Vec<Vec<usize>>, num: usize, max: usize) -> Vec<usize> {
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