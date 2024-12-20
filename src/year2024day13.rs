use regex::Regex;
use crate::read_string;

struct MachineDetails {
    a_x_delta: isize,
    a_y_delta: isize,
    b_x_delta: isize,
    b_y_delta: isize,
    prize_x: isize,
    prize_y: isize,
}

impl MachineDetails {
    fn read_file(filename: &str, offset: isize) -> Vec<Self> {
        let mut machines = Vec::new();
        let pattern = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)\n?"
        ).unwrap();
        let input = read_string(filename);
        for capture in pattern.captures_iter(input.as_str()) {
            machines.push(Self {
                a_x_delta: capture[1].parse().unwrap(),
                a_y_delta: capture[2].parse().unwrap(),
                b_x_delta: capture[3].parse().unwrap(),
                b_y_delta: capture[4].parse().unwrap(),
                prize_x: capture[5].parse::<isize>().unwrap() + offset,
                prize_y: capture[6].parse::<isize>().unwrap() + offset,
            })
        }
        
        machines
    }
    
    fn cost(&self) -> Option<usize> {
        let b = (self.prize_x*self.a_y_delta - self.prize_y*self.a_x_delta) /
            (self.b_x_delta*self.a_y_delta - self.b_y_delta*self.a_x_delta);
        let a = (self.prize_x-b*self.b_x_delta) / self.a_x_delta;
        
        if self.a_x_delta*a + self.b_x_delta*b == self.prize_x &&
            self.a_y_delta*a + self.b_y_delta*b == self.prize_y 
        {
            Some((3 * a + b) as usize)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        use crate::year2024day13::MachineDetails;

        #[test]
        fn parse() {
            let machine_details = MachineDetails::read_file("input/2024-13-e1.txt", 0);
            assert_eq!(4, machine_details.len());
        }
        
        #[test]
        fn example() {
            let machine_details = MachineDetails::read_file("input/2024-13-e1.txt", 0);
            let cost: usize = machine_details.iter().filter_map(|m| m.cost()).sum();
            assert_eq!(480, cost);
        }

        #[test]
        fn solution() {
            let machine_details = MachineDetails::read_file("input/2024-13-input.txt", 0);
            let cost: usize = machine_details.iter().filter_map(|m| m.cost()).sum();
            assert_eq!(28753, cost);
        }
    }
    
    mod part2 {
        use crate::year2024day13::MachineDetails;

        #[test]
        fn solution() {
            let machine_details = MachineDetails::read_file("input/2024-13-input.txt", 10000000000000);
            let cost: usize = machine_details.iter().filter_map(|m| m.cost()).sum();
            assert_eq!(102718967795500, cost);
        }
    }
}