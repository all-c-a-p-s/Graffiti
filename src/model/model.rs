use std::collections::HashMap;
use std::io::BufReader;

use tract_core::ndarray;
use tract_data::internal::tract_smallvec::SmallVec;
use tract_ndarray::Array1;
use tract_ndarray::Array2;
use tract_ndarray::ArrayD;
use tract_ndarray::IxDyn;
use tract_onnx::prelude::*;

use ort::{Environment, GraphOptimizationLevel, Session, SessionBuilder, Tensor};

use crate::hold_index_to_name;
use crate::name_to_arr_index;

pub fn run_model(
    start_holds: Vec<String>,
    finish_holds: Vec<String>,
    intermediate_holds: Vec<String>,
) -> ort::Result<String> {
    let mut holds_data: Vec<f32> = vec![0.0; 198];
    for hold in start_holds {
        let idx = name_to_arr_index(hold.as_str());
        holds_data[idx.0 * 11 + idx.1] = 1.0;
    }
    for hold in finish_holds {
        let idx = name_to_arr_index(hold.as_str());
        holds_data[idx.0 * 11 + idx.1] = 2.0;
    }
    for hold in intermediate_holds {
        let idx = name_to_arr_index(hold.as_str());
        holds_data[idx.0 * 11 + idx.1] = 3.0;
    }

    //include NN data at compile time
    let nn_data = include_bytes!("/Users/seba/rs/graffiti/models/custom_model.onnx");
    let session = Session::builder()?.commit_from_memory(nn_data)?;

    let input_holds = Tensor::from_array(([1, 198], holds_data.clone().into_boxed_slice()))?;
    let mut inputs = HashMap::new();
    inputs.insert("input_layer", input_holds);

    let outputs = session.run(inputs)?;

    let mut probabilities: Vec<f32> = Vec::new();

    for (output_name, output_value) in outputs.iter() {
        probabilities = output_value
            .to_owned()
            .try_extract_tensor::<f32>()?
            .iter()
            .cloned()
            .collect::<Vec<f32>>();
    }

    let mut max: f32 = 0.0;
    let mut most_likely_grade = 4;

    for i in 0..probabilities.len() {
        if probabilities[i] > max {
            max = probabilities[i];
            most_likely_grade = i + 4;
        }
    }
    Ok(format!("I guess grade v{}", most_likely_grade))
}

pub fn run_routeset_model(
    start_holds: &Vec<String>,
    finish_holds: &Vec<String>,
    intermediate_holds: &Vec<String>,
    grade: f32,
) -> ort::Result<Option<String>> {
    println!("{}", grade);
    let mut holds_data: Vec<Vec<f32>> = vec![vec![0.0f32; 11]; 18];
    for hold in start_holds {
        let idx = name_to_arr_index(hold.as_str());
        holds_data[idx.0][idx.1] = 1.0;
    }
    for hold in finish_holds {
        let idx = name_to_arr_index(hold.as_str());
        holds_data[idx.0][idx.1] = 2.0;
    }
    for hold in intermediate_holds {
        let idx = name_to_arr_index(hold.as_str());
        holds_data[idx.0][idx.1] = 3.0;
    }

    let nn_data = include_bytes!("/Users/seba/rs/graffiti/models/routeset/routeset.onnx");

    let session = Session::builder()?.commit_from_memory(nn_data)?;

    let input_vector = holds_data.iter().cloned().flatten().collect::<Vec<f32>>();
    let input_holds = Tensor::from_array(([1, 18, 11], input_vector.clone().into_boxed_slice()))?;
    let input_grade = Tensor::from_array(([1, 1], vec![grade].into_boxed_slice()))?;

    let mut inputs = HashMap::new();
    inputs.insert("input_holds", input_holds);
    inputs.insert("input_grades", input_grade);

    let outputs = session.run(inputs)?;

    let mut probabilities: Vec<f32> = Vec::new();
    for (output_name, output_value) in outputs.iter() {
        probabilities = output_value
            .to_owned()
            .try_extract_tensor::<f32>()?
            .iter()
            .cloned()
            .collect::<Vec<f32>>();
    }

    let mut max: f32 = 0.0;
    let mut most_likely_hold = Some(String::new());

    for i in 0..probabilities.len() {
        if i != 198 && (probabilities[i] > max) && (input_vector[i] == 0.0) {
            max = probabilities[i];
            most_likely_hold = Some(hold_index_to_name((i)));
        } else if i == 198 && probabilities[i] > max {
            return Ok(None);
        }
    }
    Ok(most_likely_hold)
}

pub fn generate_route(
    mut start_holds: Vec<String>,
    mut finish_holds: Vec<String>,
    mut intermediate_holds: Vec<String>,
    grade: usize,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut s = start_holds.clone();
    let mut f = finish_holds.clone();
    let mut i = intermediate_holds.clone();
    let mut next_hold = run_routeset_model(
        &start_holds,
        &finish_holds,
        &intermediate_holds,
        grade as f32,
    )
    .expect("failed to run model");
    while next_hold.is_some() {
        match name_to_arr_index(next_hold.clone().unwrap().as_str()) {
            _ if s.len() == 0 => s.push(next_hold.clone().unwrap()),
            (17, _) => f.push(next_hold.clone().unwrap()),
            _ => i.push(next_hold.clone().unwrap()),
        }
        next_hold = run_routeset_model(&s, &f, &i, grade as f32).expect("failed to run model");
    }
    (s, f, i)
}
