mod kmeans_struct;
use kmeans_struct::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point_display() {
        let point = Point { x: 3.0, y: 4.0 };
        assert_eq!(format!("{}", point), "(3.0, 4.0)");
    }

    #[test]
    fn test_point_equality() {
        let point1 = Point { x: 3.0, y: 4.0 };
        let point2 = Point { x: 3.0, y: 4.0 };
        assert_eq!(point1, point2);
    }

    #[test]
    fn test_point_distance() {
        let point1 = Point { x: 0.0, y: 0.0 };
        let point2 = Point { x: 3.0, y: 4.0 };
        assert_eq!(point1.dis(&point2), 5.0);
    }
    #[test]
    fn test_generate_point() {
        let dot_num = 10;
        let points = generate_point(dot_num);
        assert_eq!(points.len(), dot_num);

        // let mut distinct_points = points.clone();
        // distinct_points.dedup();
        // assert_eq!(points.len(), distinct_points.len());
    }
}
fn main(){

}