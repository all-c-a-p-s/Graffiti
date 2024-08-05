# Graffiti
Graffiti is a work-in-progress project with the aim to set human-like climbing routes on the [moonboard](https://moonclimbing.com/moonboard). As an intermediate step to this, a model has been trained which is able to guess the Hueco grade (v4-v14) of a given route on the moonboard. 

## The Grade Prediction Model
The model uses the following architecture:
Model: "functional"
| Layer (type)                    | Output Shape | Param # |
|---------------------------------|--------------|---------|
| input_layer (InputLayer)        | (None, 198)  | 0       |
| cast_to_float32 (CastToFloat32) | (None, 198)  | 0       |
| dense (Dense)                   | (None, 11)   | 2,189   |
| classification_head_1 (Softmax) | (None, 11)   | 0       |

 Total params: 2,191 (8.56 KB)
 
 Trainable params: 2,189 (8.55 KB)
 
 Non-trainable params: 0 (0.00 B)
 
 Optimizer params: 2 (12.00 B)

At present, it is able to guess the grade of a route with between 45 and 50 percent accuracy, which I am fairly happy with, given that it has 11 possible grades to choose from.


## TODOs
There are several stages to the project which I still need to work on:
- GUI interface to interact with the model (+ mobile app?)
- if possible, integrating the tensorflow model with Rust GUI code
- model to set original routes
- experimenting with different network architectures

## Contributing
Not accepting code contributions as I may end up using this for a school project.
However, since I am very new to ML, so I would welcome any suggestions on how to improve the architecture of the model.

