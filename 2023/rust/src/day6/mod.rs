use std::collections::VecDeque;

#[derive(Debug)]
struct Race {
    time: f64,
    record: f64,
    hold_time_winners: Vec<f64>,
}


pub fn day6_1(input: &Vec<String>) -> f64 {
    let times_raw = input.iter().nth(0).unwrap().replace("Time:", "");
    let distances_raw = input.iter().nth(1).unwrap().replace("Distance:", "");
    let times: Vec<_> = times_raw.trim().split(" ").filter(|t| !t.is_empty()).collect();
    let distances: Vec<_> = distances_raw.trim().split(" ").filter(|d| !d.is_empty()).collect();

    if times.len() != distances.len() {
        panic!("times and distances are not equal!");
    }

    let mut races: VecDeque<Race> = VecDeque::new(); 

    for i in 0..times.len() {
        races.push_front(Race{
            time: times[i].parse::<usize>().unwrap() as f64,
            record: distances[i].parse::<usize>().unwrap() as f64,
            hold_time_winners: vec![],
        });
    }

    for r in races.iter_mut() {
        let mut hold_times: Vec<f64> = vec![];
        for x in 0..(r.time as usize) {
            if calc_quadratic_fn(x as f64, r.time) > r.record {
                hold_times.push(x as f64);
            }
        }
        r.hold_time_winners = hold_times;
    }

    races.into_iter().map(|r| r.hold_time_winners.len()).reduce(|a, b| a * b).unwrap() as f64

}

// will use the f(x) = -x²+yx in order to get the distance
// x -> time to hold the button
// y -> racing time
// first eg. is f(x) = -x²+7x
fn calc_quadratic_fn(x: f64, time: f64) -> f64 {
    -x.powi(2)+time*x
} 

pub fn day6_2(input: &Vec<String>) -> u32 {
    let times_raw = input.iter().nth(0).unwrap().replace("Time:", "");
    let distances_raw = input.iter().nth(1).unwrap().replace("Distance:", "");

    let time = times_raw.replace(" ", "").parse::<usize>().unwrap();
    let distance = distances_raw.replace(" ", "").parse::<usize>().unwrap();

    let mut counter = 0;
    for x in 0..(time) {
        if calc_quadratic_fn(x as f64, time as f64) > distance as f64 {
            counter = counter + 1;
        }
    }

    counter
}
