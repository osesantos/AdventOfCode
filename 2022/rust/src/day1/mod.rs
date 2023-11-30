use crate::utils::get_file_lines;

pub fn day1_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let mut cal_sum = 0;
    let mut highest = 0;
    lines.into_iter().for_each(|s| {
        match s.parse::<i32>() {
            Ok(i) => cal_sum += i,
            Err(_) => {
                if cal_sum > highest {
                    highest = cal_sum
                }
                cal_sum = 0;
            }
        }
    });
    return highest.to_string()
}

pub fn day1_2(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let mut cal_sum = 0;
    let mut sums = Vec::new();
    lines.into_iter().for_each(|s| {
        match s.parse::<i32>() {
            Ok(i) => cal_sum += i,
            Err(_) => {
                sums.push(cal_sum);
                cal_sum = 0;
            }
        }
    });
    sums.sort_by(|a, b| b.cmp(a));
    let top = &sums[0..3];
    return top.into_iter().sum::<i32>().to_string();
}