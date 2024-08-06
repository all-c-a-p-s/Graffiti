use pyo3::prelude::*;
use pyo3::prepare_freethreaded_python;
use std::fs::*;
use std::io::Write;
use std::path::Path;

pub fn write_holds_to_file(
    start_holds: Vec<String>,
    finish_holds: Vec<String>,
    intermediate_holds: Vec<String>,
) -> std::io::Result<()> {
    let path = Path::new("/Users/seba/rs/graffiti/src/model/model.py");
    let file_contents = std::fs::read_to_string(path).expect("failed to read from file");
    let mut lines = file_contents
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| String::from(*x))
        .collect::<Vec<String>>();

    for i in 0..lines.len() {
        let line = lines[i].clone();
        let words = line
            .as_str()
            .split_whitespace()
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        if words.len() == 0 {
            continue;
        }

        match words[0].as_str() {
            "START_HOLDS" => lines[i] = edit_line(String::from("START_HOLDS"), start_holds.clone()),
            "INTERMEDIATE_HOLDS" => {
                lines[i] = edit_line(
                    String::from("INTERMEDIATE_HOLDS"),
                    intermediate_holds.clone(),
                )
            }
            "FINISH_HOLDS" => {
                lines[i] = edit_line(String::from("FINISH_HOLDS"), finish_holds.clone())
            }
            _ => {}
        }
    }

    let rewritten_lines = lines
        .iter()
        .fold(String::new(), |acc, l| acc + (l.to_owned() + "\n").as_str());
    std::fs::write(
        "/Users/seba/rs/graffiti/src/model/model.py",
        rewritten_lines.as_str().as_bytes(),
    );
    Ok(())
}

pub fn edit_line(first_word: String, mut holds: Vec<String>) -> String {
    //this should only be called with the START_HOLDS, FINISH_HOLDS, and INTERMEDIATE_HOLDS lines
    let mut arr: String = String::from("[");
    for i in 0..holds.len() {
        arr += r#"""#;
        arr += holds[i].as_str();
        arr += r#"""#;
        if i != holds.len() - 1 {
            arr += ", "
        }
    }
    arr += "]";

    match first_word.as_str() {
        "START_HOLDS" => String::from("START_HOLDS = ") + arr.as_str(),
        "FINISH_HOLDS" => String::from("FINISH_HOLDS = ") + arr.as_str(),
        "INTERMEDIATE_HOLDS" => String::from("INTERMEDIATE_HOLDS = ") + arr.as_str(),
        _ => unreachable!(),
    }
}

pub fn run_model(
    start_holds: Vec<String>,
    finish_holds: Vec<String>,
    intermediate_holds: Vec<String>,
) -> String {
    match write_holds_to_file(start_holds, finish_holds, intermediate_holds) {
        Ok(_) => println!("wrote holds to file"),
        Err(_) => panic!("failed to write holds to file"),
    }

    //important: do NOT use the include_str!() macro to include this at compile time
    //or the updated file will not be used
    let model = std::fs::read_to_string("/Users/seba/rs/graffiti/src/model/model.py")
        .expect("failed to read model file");
    let auto = std::fs::read_to_string("/Users/seba/rs/graffiti/src/model/auto.py")
        .expect("failed to read model file");

    prepare_freethreaded_python();
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from_code_bound(py, auto.as_str(), "auto", "auto")?;
        let model: Py<PyAny> = PyModule::from_code_bound(py, model.as_str(), "", "")?
            .getattr("run_model")?
            .into();
        model.call0(py).expect("failed to call model").extract(py)
    })
    .expect("failed to convert output of model");
    let output = format!("{}", from_python);
    println!("{}", output);
    let probablities = output[2..output.len() - 2]
        .split_whitespace()
        .map(|x| str::parse::<f32>(x).expect("failed to convert to float"))
        .collect::<Vec<f32>>();

    let mut max: f32 = 0.0;
    let mut most_likely_grade = 4;

    for i in 0..probablities.len() {
        if probablities[i] > max {
            max = probablities[i];
            most_likely_grade = i + 4;
        }
    }
    format!("I guess grade v{}", most_likely_grade)
}
