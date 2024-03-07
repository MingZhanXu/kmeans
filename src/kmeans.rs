use crate::kmeans_struct::*;
/// 產生隨機群中心點
///
/// # 變數說明
///
/// * `seed_num`: 所要產生中心點數量。
/// * `dot_num`: 座標總數。
/// * `回傳` : `k_num`: k_num\[中心點順序\]
/// # 使用範例
///
/// ```
/// use kmeans::kmeans_struct::Point;
/// use kmeans::kmeans::random_center;
/// let seed_num = 5;
/// let dot_num = 10;
/// let result = random_center(seed_num, dot_num);
/// assert_eq!(result.len(), seed_num);
/// ```
///
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
/// 將座標分群
///
/// # 變數說明
///
/// * `point`: point\[座標編號\]。
/// * `k_num`: k_num\[中心點順序\]。
/// * `num`: 使用者編號。
/// * `max`: 總運算人數。
/// * `回傳` : `team`: team\[使用者編號\]\[中心點順序\]
/// # 使用範例
///
/// ```
/// use kmeans::kmeans_struct::Point;
/// use kmeans::kmeans::cluster;
/// let point = vec![Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }, Point { x: 5.0, y: 6.0 }, Point { x: 8.0, y: 8.0}];
/// let k_num = vec![2, 3];
/// let num = 0;
/// let max = 1;
/// let result = cluster(&point, &k_num, num, max);
/// assert_eq!(result, vec![vec![0, 1, 2], vec![3]]);
/// ```
///
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
/// * `point`: point\[座標編號\]。
/// * `team`: team\[使用者編號\]\[中心點順序\]。
/// * `num`: 使用者編號。
/// * `max`: 總運算人數。
/// * `回傳` : k_num\[中心點順序\]
/// # 使用範例
///
/// ```
/// use kmeans::kmeans_struct::Point;
/// use kmeans::kmeans::re_seed;
/// let point = vec![Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }, Point { x: 5.0, y: 6.0 }, Point { x: 8.0, y: 8.0}];
/// let team = vec![vec![0, 1, 2], vec![3]];
/// let num = 0;
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