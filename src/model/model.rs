use std::io::BufReader;

use tract_core::ndarray;
use tract_onnx::prelude::*;

use crate::hold_index_to_name;
use crate::name_to_arr_index;

pub fn run_model(start_holds: Vec<String>, finish_holds: Vec<String>, intermediate_holds: Vec<String>) -> TractResult<String> {
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

    for i in 0..probabilities.len() {
        if probabilities[i] > max {
            max = probabilities[i];
            most_likely_grade = i + 4;
        }
    }
    Ok(format!("I guess grade v{}", most_likely_grade))
}
