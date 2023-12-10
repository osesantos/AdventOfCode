pub fn day9_1(lines: &Vec<String>) -> u32 {
    let mut lines_sequences:  Vec<Vec<usize>> = lines.into_iter().map(|l| l.split(" ").map(|s| s.parse::<usize>().unwrap()).collect()).collect();
    let mut sum = 0;
    for sequence in lines_sequences {
        let mut list_of_deriv_sequences: Vec<Vec<usize>> = vec![];
        list_of_deriv_sequences.push(sequence);

        let mut exit = false;
        // Create a map like stated on the problem
        while !exit {
            || {
                list_of_deriv_sequences.last().into_iter().enumerate().for_each(|(i, elem)| {
                    let mut deriv: Vec<usize> = vec![];
                    for i in 0..elem.len() {
                        if i == 0 {
                            return;
                        } else {
                            deriv.push(elem.into_iter().nth(i).unwrap() - elem.into_iter().nth(i-1).unwrap());
                        }
                    }
                    list_of_deriv_sequences.push(deriv);
                })
            };
            exit = list_of_deriv_sequences.last().into_iter().find(|e| !e.contains(&0)) == None
        }

        // Add the last element backwards

        // add the last elem from the first sequence to the sum 
    }


    0
}

pub fn day9_2(input: &Vec<String>) -> u32 {
    0
}
