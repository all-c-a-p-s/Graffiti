pub mod json_parse;

use std::fmt::Display;
use std::io;

use json_parse::get_climbs;

pub const ROWS: usize = 18;
pub const COLUMNS: usize = 11;

pub enum Grade {
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Grade::V4 => write!(f, "V4"),
            Grade::V5 => write!(f, "V5"),
            Grade::V6 => write!(f, "V6"),
            Grade::V7 => write!(f, "V7"),
            Grade::V8 => write!(f, "V8"),
            Grade::V9 => write!(f, "V9"),
            Grade::V10 => write!(f, "V10"),
            Grade::V11 => write!(f, "V11"),
            Grade::V12 => write!(f, "V12"),
            Grade::V13 => write!(f, "V13"),
            Grade::V14 => write!(f, "V14"),
        }
    }
}

pub struct HoldMatrix {
    holds: [[bool; 11]; 18],
}

impl HoldMatrix {
    fn from(arr: [[bool; 11]; 18]) -> Self {
        Self { holds: arr }
    }
}

pub struct Route {
    name: String,
    grade: Grade,
    start_holds: HoldMatrix,
    finish_holds: HoldMatrix,
    feet_only_holds: HoldMatrix,
    handholds: HoldMatrix,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            name: String::from("default"),
            grade: Grade::V4,
            start_holds: HoldMatrix::from([[false; 11]; 18]),
            finish_holds: HoldMatrix::from([[false; 11]; 18]),
            feet_only_holds: HoldMatrix::from([[false; 11]; 18]),
            handholds: HoldMatrix::from([[false; 11]; 18]),
        }
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in (0..ROWS).rev() {
            for column in 0..COLUMNS {
                if self.start_holds.holds[row][column] {
                    s += "S"
                } else if self.finish_holds.holds[row][column] {
                    s += "T"
                } else if self.feet_only_holds.holds[row][column] {
                    s += "F"
                } else if self.handholds.holds[row][column] {
                    s += "H"
                } else {
                    s += "_"
                }
                if column != COLUMNS - 1 {
                    s += " "
                }
            }
            s += "\n"
        }
        s += format!("Name: {}\n", self.name).as_str();
        s += format!("Grade: {}\n", self.grade).as_str();
        write!(f, "{}", s)
    }
}

impl HoldMatrix {
    fn count_holds(&self) -> usize {
        let mut res: usize = 0;
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                if self.holds[row][column] {
                    res += 1;
                }
            }
        }
        res
    }
}

impl Route {
    fn check_valid(&self) -> std::io::Result<()> {
        match self.start_holds.count_holds() {
            0 => panic!("This route is invalid becuase it has no start holds"),
            1..=2 => {}
            k => panic!(
                "This route is invalid because it has {} start holds but it must have 2 or less",
                k
            ),
        }
        match self.finish_holds.count_holds() {
            0 => panic!("This route is invalid because it has no finish holds"),
            1..=2 => {}
            k => panic!(
                "This route is invalid becuase it has {} finish holds but it must have 2 or less",
                k
            ),
        }

        fn count_true(
            index: (usize, usize),
            h1: &HoldMatrix,
            h2: &HoldMatrix,
            h3: &HoldMatrix,
            h4: &HoldMatrix,
        ) -> usize {
            let mut res = 0;
            if h1.holds[index.0][index.1] {
                res += 1
            }
            if h2.holds[index.0][index.1] {
                res += 1
            }
            if h3.holds[index.0][index.1] {
                res += 1
            }
            if h4.holds[index.0][index.1] {
                res += 1
            }
            res
        }

        for row in 0..ROWS {
            for column in 0..COLUMNS {
                if count_true(
                    (row, column),
                    &self.start_holds,
                    &self.finish_holds,
                    &self.handholds,
                    &self.feet_only_holds,
                ) > 1
                {
                    panic!(
                        "This route is invalid because hold {} used more than once",
                        hold_index_to_name((row, column))
                    )
                }
            }
        }
        return Ok(());
    }

    fn from(
        name: String,
        grade: Grade,
        start_holds: HoldMatrix,
        finish_holds: HoldMatrix,
        feet_only_holds: HoldMatrix,
        handholds: HoldMatrix,
    ) -> Self {
        Self {
            name,
            grade,
            start_holds,
            finish_holds,
            feet_only_holds,
            handholds,
        }
    }
}

fn read_route_name() -> String {
    println!("Enter route name: ");
    let mut name = String::new();
    let read_result = io::stdin().read_line(&mut name);
    match read_result {
        Ok(_) => {}
        Err(e) => panic!("failed to read in route name because of error {}", e),
    };
    println!();
    String::from(name.trim())
}

fn read_grade() -> Grade {
    println!("Enter route grade (Hueco): ");
    let mut grade = String::new();
    let read_result = io::stdin().read_line(&mut grade);
    match read_result {
        Ok(_) => {}
        Err(e) => panic!("failed to read in route grade becuase of error {}", e),
    };
    let first_char = grade.as_str().chars().collect::<Vec<char>>()[0];
    match first_char {
        'v' | 'V' => {},
        _ => panic!("Inputted grade is invalid because the first character {} is invalid (expected v or V). Please make sure you input the grade in the Hueco (v-based) grade system", first_char),
    };
    let number_part = grade
        .as_str()
        .get(1..)
        .expect("grade inputted is invalid because it only contains one character")
        .trim();
    let number = number_part.parse::<u8>();
    let number = match number {
        Ok(n) if (n >= 4 && n <= 14) => n,
        Ok(_) => panic!("the grade is not between v4 and v14 (the bounds for moonboard grades)"),
        Err(e) => panic!(
            "expcted a number after the {} character in the grade but found error {}",
            first_char, e
        ),
    };
    match number {
        4 => Grade::V4,
        5 => Grade::V5,
        6 => Grade::V6,
        7 => Grade::V7,
        8 => Grade::V8,
        9 => Grade::V9,
        10 => Grade::V10,
        11 => Grade::V11,
        12 => Grade::V12,
        13 => Grade::V13,
        14 => Grade::V14,
        _ => unreachable!(),
    }
}

fn name_to_arr_index(hold: &str) -> (usize, usize) {
    let first_char = hold.chars().collect::<Vec<char>>()[0];
    match first_char {
        'A'..='K' => {}
        'a'..='k' => {}
        _ => panic!("Invalid column {}", first_char),
    };
    let number_part = hold
        .get(1..)
        .expect("grade inputted is invalid because it only contains one character");
    let row = number_part.parse::<usize>();
    let row = match row {
        Ok(n) if (n >= 1 && n <= 18) => n,
        Ok(_) => panic!("the row is not between 1 and 18"),
        Err(e) => panic!(
            "expcted a number after the {} character in the grade but found error {}",
            first_char, e
        ),
    } - 1;
    let column = match first_char.to_ascii_lowercase() {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        _ => unreachable!(),
    };
    (row, column)
}

fn hold_index_to_name(i: (usize, usize)) -> String {
    let column = match i.0 {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        _ => unreachable!(),
    }
    .to_string();
    let row = (i.1 - 1).to_string();
    column + row.as_str()
}

impl HoldMatrix {
    fn from_hold_names(s: String) -> Self {
        //taken in as space-separated hold names e.g. A2 B5 D11 etc.
        let mut arr = [[false; 11]; 18];
        let indidices = s
            .split_whitespace()
            .map(|x| name_to_arr_index(x))
            .collect::<Vec<(usize, usize)>>();
        for i in indidices {
            arr[i.0][i.1] = true;
        }
        HoldMatrix::from(arr)
    }
}

fn read_holds() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read in holds");
    println!();
    input
}

fn read_route() -> Route {
    let name = read_route_name();
    let grade = read_grade();

    println!("Enter start holds: ");
    let start_holds = HoldMatrix::from_hold_names(read_holds());

    println!("Enter finish holds: ");
    let finish_holds = HoldMatrix::from_hold_names(read_holds());

    println!("Enter feet only holds: ");
    let feet_only_holds = HoldMatrix::from_hold_names(read_holds());

    println!("Enter handholds: ");
    let handholds = HoldMatrix::from_hold_names(read_holds());
    let r = Route::from(
        name,
        grade,
        start_holds,
        finish_holds,
        feet_only_holds,
        handholds,
    );
    match r.check_valid() {
        Ok(_) => return r,
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    //let r = read_route();
    //println!("{}", r);
    match get_climbs() {
        Ok(v) => {},
        Err(e) => panic!("{}", e),
    };
}
