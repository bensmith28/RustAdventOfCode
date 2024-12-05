mod year2024day4 {
    use regex::Regex;
    use crate::read_lines;

    struct Panel {
        panel: Vec<char>,
        height: usize,
        width: usize,
    }

    impl Panel {
        fn new(filename: &str) -> Self {
            let mut panel: Vec<char> = Vec::new();
            let mut height = 0;
            let mut width = 0;
            let mut lines = read_lines(filename);
            while let Some(Ok(line)) = lines.next() {
                height += 1;
                width = line.len();
                for c in line.chars() {
                    panel.push(c);
                }
            }
            Panel { panel, height, width }
        }

        fn rows(&self) -> Vec<Vec<char>> {
            let mut rows = Vec::new();
            for r in 0..self.height {
                let mut row: Vec<char> = Vec::new();
                for i in r*self.width..r*self.width+self.width {
                    row.push(self.panel[i])
                }
                rows.push(row)
            }
            rows
        }

        fn columns(&self) -> Vec<Vec<char>> {
            let mut columns = Vec::new();
            for c in 0..self.width {
                let mut column = Vec::new();
                for i in (c..self.panel.len()).step_by(self.width) {
                    column.push(self.panel[i])
                }
                columns.push(column)
            }
            columns
        }
        
        fn diagonals(&self) -> Vec<Vec<char>> {
            let mut diagonals = Vec::new();
            let h = self.height as isize;
            let w = self.width as isize;
            let adjust = w-1;
            for r in 0..h+adjust {
                // up and right
                let mut diagonal = Vec::new();
                for i in (r..=r*w).step_by(self.width-1) {
                    if i < 0 || i >= self.panel.len() as isize { continue; }
                    diagonal.push(self.panel[i as usize])
                }
                while r >= h && diagonal.len() as isize > 2*h - 1 - r {
                    diagonal.remove(0);
                }
                if diagonal.len() > 0 { 
                    diagonals.push(diagonal); 
                }
                // up and left
                diagonal = Vec::new();
                for i in (w-1-r..w+r*w).step_by(self.width+1) {
                    if i < 0 || i >= self.panel.len() as isize { continue; }
                    if r >= h && i < (r-h)*w { continue; }
                    diagonal.push(self.panel[i as usize])
                }
                if diagonal.len() > 0 { 
                    diagonals.push(diagonal); 
                }
            }
            diagonals
        }
        
        fn all_lines(&self) -> Vec<Vec<char>> {
            let mut all_lines = Vec::new();
            for line in self.rows() {
                all_lines.push(line);
            }
            for line in self.columns() {
                all_lines.push(line);
            }
            for line in self.diagonals() {
                all_lines.push(line);
            }
            all_lines
        }
        
        fn all_windows(&self) -> Vec<String> {
            let mut windows = Vec::new();
            for r in 0..self.height-2 {
                for c in 0..self.width-2 {
                    let mut window = String::new();
                    for i in 0..3 {
                        for j in 0..3 {
                            window.push(self.panel[(r+i)*self.width+c+j])
                        }
                    }
                    windows.push(window);
                }
            }
            windows
        }
    }
    
    fn part1(filename: &str) -> usize {
        let panel = Panel::new(filename);
        let mut result = 0;
        for line in panel.all_lines() {
            for window in line.windows(4) {
                let s = window.iter().collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    result += 1;
                }
            }
        }
        result
    }
    
    fn part2(filename: &str) -> usize {
        let panel = Panel::new(filename);
        let mut result = 0;
        // M.S M.M S.M S.S
        // .A. .A. .A. .A.
        // M.S S.S S.M M.M
        let pattern = Regex::new(r"M.S.A.M.S|M.M.A.S.S|S.M.A.S.M|S.S.A.M.M").unwrap();
        for window in panel.all_windows() {
            if pattern.is_match(window.as_str()) {
                result += 1;
            }
        }
        result
    }

    #[cfg(test)]
    mod test {
        mod parse {
            use crate::year2024day4::year2024day4::Panel;

            #[test]
            fn example() {
                let actual = Panel::new("input/2024-04-e1.txt");
                assert_eq!(actual.height, 5);
                assert_eq!(actual.width, 6);
                let rows: Vec<String> = actual.rows().iter().map(|r| r.iter().collect()).collect();
                assert_eq!(rows[0], "..X...");
                assert_eq!(rows[1], ".SAMX.");
                assert_eq!(rows[2], ".A..A.");
                assert_eq!(rows[3], "XMAS.S");
                assert_eq!(rows[4], ".X....");
            }
            
            #[test]
            fn example_diagonals() {
                let actual = Panel::new("input/2024-04-e2.txt").diagonals();
                assert_eq!(actual.len(), 14)
            }
        }
        
        mod part1 {
            use crate::year2024day4::year2024day4::part1;

            #[test]
            fn example() {
                assert_eq!(part1("input/2024-04-e1.txt"), 4);
                assert_eq!(part1("input/2024-04-e3.txt"), 18);
            }
            
            #[test]
            fn solution() {
                assert_eq!(part1("input/2024-04-input.txt"), 2344);
            }
        }
        
        mod part2 {
            use crate::year2024day4::year2024day4::part2;

            #[test]
            fn example() {
                assert_eq!(part2("input/2024-04-e3.txt"), 9);
            }
            
            #[test]
            fn solution() {
                assert_eq!(part2("input/2024-04-input.txt"), 1815);
            }
        }
    }
}