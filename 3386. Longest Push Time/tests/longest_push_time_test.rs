use longest_push_time::Solution;

#[test]
fn example1() {
    let release_times = vec![9, 29, 49, 50];
    let keys_pressed = "cbcd".to_string();
    assert_eq!(Solution::slowest_key(release_times, keys_pressed), 'c');
}

#[test]
fn example2() {
    let release_times = vec![12, 23, 36, 46, 62];
    let keys_pressed = "spuda".to_string();
    assert_eq!(Solution::slowest_key(release_times, keys_pressed), 'a');
}