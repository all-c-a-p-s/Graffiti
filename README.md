# Graffiti
Graffiti is a work-in-progress project with the aim to set human-like climbing routes on the [moonboard](https://moonclimbing.com/moonboard). As an intermediate step to this, a model has been trained which is able to guess the Hueco grade (v4-v14) of a given route on the moonboard. 

<img width="863" alt="demo" src="https://github.com/user-attachments/assets/6d023c56-6152-468d-a3d2-2da39ab86da7">

## The Grade Prediction Model
The model uses the following architecture:

Model: "functional"
| Layer (type)             | Output Shape   | Param # |
|--------------------------|----------------|---------|
| input_layer (InputLayer) | (None, 198)    | 0       |
| embedding (Embedding)    | (None, 198, 8) | 32      |
| flatten (Flatten)        | (None, 1584)   | 0       |
| dense (Dense)            | (None, 128)    | 202,880 |
| dense_1 (Dense)          | (None, 11)     | 1,419   |

 Total params: 408,664 (1.56 MB)
 
 Trainable params: 204,331 (798.17 KB)
 
 Non-trainable params: 0 (0.00 B)
 
 Optimizer params: 204,333 (798.18 KB)

At present, it is able to guess the grade of a route with roughly 50 percent accuracy. According to [an excellent paper](https://ar5iv.labs.arxiv.org/html/2311.12419) on this subject, this is slightly better than human accuracy.


## TODOs
- some code to check for valid user inputs/handle erroneous inputs (such as trying to add holds that do not exist)
- if possible, fix the issue that my model almost never guesses a grade harder than v10 because climbs with grades > v10 are so rare
- improve GUI interface + deploy web app
- model to set original routes

## Contributing
Not accepting code contributions as I may end up using this for a school project.
However, since I am very new to ML, so I would welcome any suggestions on how to improve the architecture of the model.

