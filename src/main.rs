use std::time::Instant;

fn main() {
    println!("-- the lab - Floyd cycle detection --");

    let now = Instant::now();

    println!("The answer is {}", get_answer(27)); //1_000_000_000));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

fn get_answer(max_cycles: usize) -> usize {
    let mut cycle = 0;
    let mut found = false;
    let start_value = 0;
    let mut tortue = start_value;
    let mut lievre = start_value;
    // in the worst case (less than a cycle or no cycle), we do all the iterations
    for i in 1..=max_cycles {
        iterate(&mut tortue, 1);
        iterate(&mut lievre, 2);
        // compare
        if tortue == lievre {
            cycle = i;
            found = true;
            println!("cycle found !! : {}", cycle);
            break;
        }
    }

    let iterations_done = if found {
        let remaining_iterations = max_cycles % cycle;
        println!("remaining iterations : {}", remaining_iterations);
        iterate(&mut tortue, remaining_iterations);
        cycle + remaining_iterations
    } else {
        max_cycles
    };
    println!(
        "The value after {max_cycles} iterations is {tortue}. It needed {iterations_done} iterations"
    );
    tortue
}

fn iterate(i: &mut usize, cycles: usize) {
    for _ in 0..cycles {
        cyclic_funtion(i);
        //println!("{i}");
    }
}

fn cyclic_funtion(i: &mut usize) {
    *i += 1;
    if *i > 27 {
        *i = 3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(1), 1);
        assert_eq!(get_answer(27), 27);
        assert_eq!(get_answer(28), 3);
        assert_eq!(get_answer(29), 4);
        assert_eq!(get_answer(49), 24);
        assert_eq!(get_answer(50), 25);
        assert_eq!(get_answer(52), 27);
        assert_eq!(get_answer(53), 3);
    }
}
