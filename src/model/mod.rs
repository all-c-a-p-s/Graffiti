use std::io::BufReader;

use tract_core::ndarray;
use tract_onnx::prelude::*;

use crate::climb::hold_index_to_name;
use crate::climb::name_to_arr_index;

pub fn run_model(
    start_holds: Vec<String>,
    finish_holds: Vec<String>,
    intermediate_holds: Vec<String>,
) -> TractResult<String> {
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
    let mut reader = BufReader::new(&nn_data[..]);

    let model = tract_onnx::onnx()
        .model_for_read(&mut reader)?
        .into_optimized()?
        .into_runnable()?;

    let input = tract_ndarray::Array2::from_shape_vec((1, 198), holds_data)?;
    let input = Tensor::from(input);
    let output = model.run(tvec!(input.into()))?;

    let probabilities = output[0]
        .to_array_view::<f32>()
        .expect("failed to convert tensor to array")
        .as_slice()
        .unwrap()
        .to_owned();

    let mut max: f32 = 0.0;
    let mut most_likely_grade = 4;

    for (i, p) in probabilities.iter().enumerate() {
        if *p > max {
            max = *p;
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
) -> TractResult<Option<String>> {
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

    let nn_data = include_bytes!("/Users/seba/rs/graffiti/models/routeset/routeset.onnx");
    let mut reader = BufReader::new(&nn_data[..]);

    let model = tract_onnx::onnx()
        .model_for_read(&mut reader)?
        .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), [1, 18, 11]))?
        .with_input_fact(1, InferenceFact::dt_shape(f32::datum_type(), [1, 1]))?
        .into_optimized()?
        .into_runnable()?;

    let input_holds = Tensor::from(tract_ndarray::Array3::from_shape_vec(
        (1, 18, 11),
        holds_data.clone(),
    )?);
    let input_grade = Tensor::from(tract_ndarray::Array2::from_shape_vec((1, 1), vec![grade])?);

    let output = model.run(tvec!(input_holds.into(), input_grade.into()))?;

    let probabilities = output[0]
        .to_array_view::<f32>()
        .expect("failed to convert tensor to array")
        .as_slice()
        .unwrap()
        .to_owned();

    let mut max: f32 = 0.0;
    let mut most_likely_hold = Some(String::new());

    for i in 0..probabilities.len() {
        if i != 198 && (probabilities[i] > max) && (holds_data[i] == 0.0) {
            max = probabilities[i];
            most_likely_hold = Some(hold_index_to_name(i));
        } else if i == 198 && probabilities[i] > max {
            return Ok(None);
        }
    }
    Ok(most_likely_hold)
}

pub fn generate_route(
    start_holds: Vec<String>,
    finish_holds: Vec<String>,
    intermediate_holds: Vec<String>,
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
            _ if s.is_empty() => s.push(next_hold.clone().unwrap()),
            (17, _) => f.push(next_hold.clone().unwrap()),
            _ => i.push(next_hold.clone().unwrap()),
        }
        next_hold = run_routeset_model(&s, &f, &i, grade as f32).expect("failed to run model");
    }
    (s, f, i)
}
