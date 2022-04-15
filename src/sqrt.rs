


pub fn my_sqrt(to_root: f64) -> f64 {
    let mut approximative_nb = to_root/10.0;
    let mut previous_approximative_nb = to_root;
    while approximative_nb != previous_approximative_nb {
        previous_approximative_nb = approximative_nb;
        approximative_nb = (approximative_nb + to_root / approximative_nb) / 2.0;
    }
    approximative_nb
}

#[test]
    fn test_root() {
        assert_eq!(my_sqrt(4.0), 2.0);
    }
    #[test]
    fn test_root_2() {
        assert_eq!(my_sqrt(9.0), 3.0);
    }
    #[test]
    fn test_root_3() {
        let root_square = my_sqrt(2.0);
        assert!(1.4 < root_square && root_square < 1.42);
    }

    #[test]
    fn test_root_4() {
        let root_square = my_sqrt(9344734347.0);
        assert!(96668.1 < root_square && root_square < 96668.2);
    }

    #[test]
    fn test_root_5() {
        let root_square = my_sqrt(123456789012345678890.0);
        assert!(11111111061.0 < root_square && root_square < 11111111061.2);
    }

