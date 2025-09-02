// Library module for performance comparison
pub fn process_data(data: &[i32]) -> i32 {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .sum()
}