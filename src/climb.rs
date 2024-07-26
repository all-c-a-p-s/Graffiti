use std::fmt::Display;
use std::io;

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

#[derive(Copy, Clone, PartialEq)]
pub enum PositionState {
    Start,
    Finish,
    Handhold,
    Empty,
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
    pub fn from(arr: [[bool; 11]; 18]) -> Self {
        Self { holds: arr }
    }
}

// how to store route holds in order
// hold struct which has properties of holds
// then use Vec<Hold>

#[derive(Copy, Clone, PartialEq)]
pub struct Hold {
    pub state: PositionState,
    pub order: usize,
}

impl Hold {
    pub fn from(state: PositionState, order: usize) -> Self {
        Self {
            state,
            order,
        }
    }
}

impl Default for Hold {
    fn default() -> Self {
        Self {
            state: PositionState::Empty,
            order: 0,
        }
    }
}

pub struct Route {
    pub name: String,
    pub grade: Grade,
    pub holds: [[Hold; 11]; 18],
}

impl Default for Route {
    fn default() -> Self {
        Self {
            name: String::from("default"),
            grade: Grade::V4,
            holds: [[Hold::default(); 11]; 18],
        }
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in (0..ROWS).rev() {
            for column in 0..COLUMNS {
                match self.holds[row][column].state {
                    PositionState::Start => s += "S",
                    PositionState::Finish => s += "T",
                    PositionState::Handhold => s += "H",
                    PositionState::Empty => s += "_",
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
    pub fn count_holds(&self) -> usize {
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
    pub fn count_target(&self, target: PositionState) -> usize {
        let mut count: usize = 0;
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                if self.holds[row][column].state == target {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn check_valid(&self) -> std::io::Result<()> {
        match self.count_target(PositionState::Start) {
            0 => panic!("This route is invalid becuase it has no start holds"),
            1..=2 => {}
            k => panic!(
                "This route is invalid because it has {} start holds but it must have 2 or less",
                k
            ),
        }
        match self.count_target(PositionState::Finish) {
            0 => panic!("This route is invalid because it has no finish holds"),
            1..=2 => {}
            k => panic!(
                "This route is invalid becuase it has {} finish holds but it must have 2 or less",
                k
            ),
        }
        return Ok(());
    }

    pub fn from(
        name: String,
        grade: Grade,
        holds: [[Hold; 11]; 18],
    ) -> Self {
        Self {
            name,
            grade,
            holds,
        }
    }
}

pub fn read_route_name() -> String {
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

impl Grade {
    pub fn from_string(grade: String) -> Self {
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
            Ok(_) => {
                panic!("the grade is not between v4 and v14 (the bounds for moonboard grades)")
            }
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

    pub fn from_font(font_grade: String) -> Grade {
        let hueco = match font_grade.as_str() {
            "6B" | "6B+" => "V4",
            "6C" | "6C+" => "V5",
            "7A" => "V6",
            "7A+" => "V7",
            "7B" | "7B+" => "V8",
            "7C" => "V9",
            "7C+" => "V10",
            "8A" => "V11",
            "8A+" => "V12",
            "8B" => "V13",
            "8B+" => "V14",
            _ => panic!("invalid font grade {}", font_grade),
        };
        Grade::from_string(String::from(hueco))
    }
}

pub fn read_grade() -> Grade {
    println!("Enter route grade (Hueco): ");
    let mut grade = String::new();
    let read_result = io::stdin().read_line(&mut grade);
    match read_result {
        Ok(_) => {}
        Err(e) => panic!("failed to read in route grade becuase of error {}", e),
    };
    Grade::from_string(grade)
}

pub fn name_to_arr_index(hold: &str) -> (usize, usize) {
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

pub fn hold_index_to_name(i: (usize, usize)) -> String {
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
    pub fn from_hold_names(s: String) -> Self {
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

pub fn set_from_hold_names(s: String, route: &mut Route, to_set: PositionState) {
    //taken in as space-separated hold names e.g. A2 B5 D11 etc.
    let indidices = s
        .split_whitespace()
        .map(|x| name_to_arr_index(x))
        .collect::<Vec<(usize, usize)>>();
    let mut count = 0;
    for i in indidices {
        route.holds[i.0][i.1] = Hold::from(to_set, count);
        count += 1;
    }
}

pub fn read_holds() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read in holds");
    println!();
    input
}

//TODO: implementation for hold order
pub fn read_route() -> Route {
    let mut route = Route::default();
    let name = read_route_name();
    route.name = name;
    let grade = read_grade();
    route.grade = grade;

    println!("Enter start holds: ");
    set_from_hold_names(read_holds(), &mut route, PositionState::Start);

    println!("Enter finish holds: ");
    set_from_hold_names(read_holds(), &mut route, PositionState::Finish);

    println!("Enter handholds: ");
    set_from_hold_names(read_holds(), &mut route, PositionState::Handhold);

    match route.check_valid() {
        Ok(_) => return route,
        Err(e) => panic!("{}", e),
    }
}
