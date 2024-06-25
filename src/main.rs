use proconio::input;

fn dir_s(next_dir: (i32, i32)) -> String {
    match next_dir {
        (0, 1) => "R",
        (0, -1) => "L",
        (-1, 0) => "U",
        (1, 0) => "D",
        _ => " ",
    }.to_string()
}

struct Solver {
    n: usize,
    h: Vec<Vec<i32>>,
    pos: (usize, usize),
    v: i32,
    next_dir: (i32, i32),
    cost: i32,
    base: i32,
    ans: Vec<String>,
}

impl Solver {
    fn new(n: usize, h: Vec<Vec<i32>>) -> Solver {
        let pos = (0, 0);
        let v = 0;
        let next_dir = (0, 1);
        let cost = 0;
        let base: i32 = h.iter().flatten().map(|x| x.abs()).sum();
        let ans: Vec<String> = Vec::new();

        Solver {n, h, pos, v, next_dir, cost, base, ans}
    }

    fn dump(&mut self) {
        let hij = self.h[self.pos.0][self.pos.1];
        if hij > 0 {
            self.ans.push(format!("+{}", hij));
            self.h[self.pos.0][self.pos.1] = 0;
            self.v += hij;
            self.cost += hij;
        } else if hij < 0 && self.v > 0 {
            if self.v >= -hij {
                self.ans.push(format!("-{}", -hij));
                self.h[self.pos.0][self.pos.1] = 0;
                self.v += hij;
                self.cost += -hij;
            } else {
                self.ans.push(format!("-{}", self.v));
                self.h[self.pos.0][self.pos.1] += self.v;
                self.cost += self.v;
                self.v = 0;
            }
        }
    }

    fn r#move(&mut self) {
        let dir = dir_s(self.next_dir);
        self.ans.push(dir.clone());
        let pos = self.pos;
        let next_dir = self.next_dir;
        self.pos.0 = (pos.0 as i32 + next_dir.0) as usize;
        self.pos.1 = (pos.1 as i32 + next_dir.1) as usize;
        self.cost += self.v + 100;
    }

    fn next_dir(&mut self, next_dir: (i32, i32)) {
        self.next_dir = next_dir;
    }

    fn result(&self) {
        for a in self.ans.iter() {
            println!("{}", a);
        }
        eprintln!("{{ \"cost\": {}, \"base\": {} }}", self.cost, self.base);
    }
}

fn main() {
    input! {
        n: usize,
        mut h: [[i32; n]; n],
    }

    let mut solver = Solver::new(n, h.clone());

    // (0, 0) => (19, 0)
    for _ in 0..(n*n) {
        // eprintln!("pos: {:?}, next_dir: {:?}", solver.pos, solver.next_dir);
        solver.dump();
        if !(solver.pos.0 == 19 && solver.next_dir.0 > 0) {
            solver.r#move();
            // 次の行動の決定
            let mut next_dir = solver.next_dir;
            let dir = dir_s(next_dir);
            let pos = solver.pos;
            if (pos.1 == 0 || pos.1 == n-1) && dir != "D" {
                next_dir = (1, 0);
            } else if pos.1 == 0 && dir == "D" {
                let row_sum: i32 = solver.h[pos.0].iter().map(|x| x.abs()).sum();
                if row_sum > 0 {
                    next_dir = (0, 1);
                }
            } else if pos.1 == n-1 && dir == "D" {
                let row_sum: i32 = solver.h[pos.0].iter().map(|x| x.abs()).sum();
                if row_sum > 0 {
                    next_dir = (0, -1);
                }
            }
            solver.next_dir(next_dir);
        }
    }

    // (19, 0) => (0, 0)
    let next_dir = if solver.pos.1 == 0 {
        (0, 1)
    } else {
        (0, -1)
    };
    solver.next_dir(next_dir);
    // eprintln!("pos: {:?}, next_dir: {:?}", solver.pos, solver.next_dir);
    for _ in 0..(n*n) {
        solver.dump();

        if !(solver.pos.0 == 0 && solver.next_dir.0 < 0) {
            solver.r#move();

            // 次の方向を決定
            let mut next_dir = solver.next_dir;
            let dir = dir_s(next_dir);
            let pos = solver.pos;
            if (pos.1 == 0 || pos.1 == n-1) && dir != "U" {
                next_dir = (-1, 0);
            } else if pos.1 == 0 && dir == "U" {
                let row_sum: i32 = solver.h[pos.0].iter().map(|x| x.abs()).sum();
                if row_sum > 0 {
                    next_dir = (0, 1);
                }
            } else if pos.1 == n-1 && dir == "U" {
                let row_sum: i32 = solver.h[pos.0].iter().map(|x| x.abs()).sum();
                if row_sum > 0 {
                    next_dir = (0, -1);
                }
            }
            solver.next_dir(next_dir);
        }
    }

    let mut opt_solver = solver;

    let mut solver = Solver::new(n, h.clone());
    solver.next_dir((1, 0));

    // (0, 0) => (19, 0)
    for _ in 0..(n*n) {
        // eprintln!("pos: {:?}, next_dir: {:?}", solver.pos, solver.next_dir);
        solver.dump();
        if !(solver.pos.1 == 19 && solver.next_dir.1 > 0) {
            solver.r#move();
            // 次の行動の決定
            let mut next_dir = solver.next_dir;
            let dir = dir_s(next_dir);
            let pos = solver.pos;
            if (pos.0 == 0 || pos.0 == n-1) && dir != "R" {
                next_dir = (0, 1);
            } else if pos.0 == 0 && dir == "R" {
                next_dir = (1, 0);
            } else if pos.1 == n-1 && dir == "R" {
                let row_sum: i32 = solver.h[pos.0].iter().map(|x| x.abs()).sum();
                next_dir = (-1, 0);
            }
            solver.next_dir(next_dir);
        }
    }

    // (19, 0) => (0, 0)
    let next_dir = if solver.pos.1 == 0 {
        (0, 1)
    } else {
        (0, -1)
    };
    solver.next_dir(next_dir);
    // eprintln!("pos: {:?}, next_dir: {:?}", solver.pos, solver.next_dir);
    for _ in 0..(n*n) {
        solver.dump();

        if !(solver.pos.0 == 0 && solver.next_dir.0 < 0) {
            solver.r#move();

            // 次の方向を決定
            let mut next_dir = solver.next_dir;
            let dir = dir_s(next_dir);
            let pos = solver.pos;
            if (pos.0 == 0 || pos.0 == n-1) && dir != "L" {
                next_dir = (0, -1);
            } else if pos.0 == 0 && dir == "L" {
                next_dir = (1, 0);
            } else if pos.0 == n-1 && dir == "L" {
                next_dir = (-1, 0);
            }
            solver.next_dir(next_dir);
        }
    }
    
    if opt_solver.cost > solver.cost {
        opt_solver = solver;
    }

    // 結果出力
    opt_solver.result();
}
