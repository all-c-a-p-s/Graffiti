"""
Method of generating new climbs:
create a dataset by removing holds actual moonboard climbs, and asking it to guess which next holds are most likely to be added
In order for it to set actual climbs with direction (i.e. actual going up the wall and not just adding random holds),
I may need to split the wall into varoius sections. However, I think it is possible that it will be able to set "directed"
routes even without this. For instance, if there are no holds on row 18, it can learn that one of these is present in every
route, and will therefore add one.

The other problem is that to be useful the AI should generate a route based on the user's grade input. To try to solve this,
I will input the grade of the route as well as the holds in the route which haven't been removed.

I also need to decide on a loss function. This will be difficult because of the fairly arbitrary nature of routesetting, meaning
that it will be impossible for my model to reliably predict which exact holds are added to the route. One approach could be for the
model to rank all the holds in order of likelihood to be in the route, and then take the log/sqrt of the rank which it gave to the actual
climb. I think choosing a concave function is a good idea because it doesn't matter very much whether a hold was ranked 100 or 150th, but
does matter a lot whether a hold was ranked first or tenth.

It will probably also work best if the "next" hold in a route is defined as the lowest down hold in the route which is not inputted to the
model, because it is most intuitive for the user if the climb is set one move at a time going up the wall.

Instead of using some arbitrary metric fo rit to know when to stop the climb, some complete climbs could also be inputted, and one of its options
to guess could be no hold (i.e. the route is already finished)
"""

# INPUT: matrix of holds already in the route + grade
# OUTPUT: 199 dimensional vector (198 holds + 1 for route already finished) of probabilities

import math
import random
import numpy as np
import pandas as pd
import json
import keras
import tensorflow as tf

from sklearn.model_selection import train_test_split
from tensorflow.keras.models import Model
from tensorflow.keras.layers import Flatten, Dense, Input, Concatenate, Conv1D, Reshape, Dropout, MaxPooling1D
from tensorflow.keras.utils import to_categorical

# first some preprocessing code:
def convert_coordinate_to_int(coordinate) -> int:
    letter = coordinate[0]
    column = ord(letter.upper()) - ord("A")
    row = int(coordinate[1:])
    return 11 * (row - 1) + column


# used to convert string labels to tensor-convertible integers
def convert_grade_to_int(grade) -> int:
    match grade:
        case "6B" | "6B+":
            return 0
        case "6C" | "6C+":
            return 1
        case "7A":
            return 2
        case "7A+":
            return 3
        case "7B" | "7B+":
            return 4
        case "7C":
            return 5
        case "7C+":
            return 6
        case "8A":
            return 7
        case "8A+":
            return 8
        case "8B":
            return 9
        case "8B+":
            return 10
        case _:
            print(f"error converting grade to int: unexpected grade found {grade}")
            exit(1)


path = "/Users/seba/rs/graffiti/json/2016.json"

with open(path) as file:
    json_data = json.load(file)

# load the climbs from the data field
climbs = json_data["data"]

# Prepare lists to construct DataFrame
grades = []
partial_climbs = []
answers = []

def count_holds(holds_vector):
    res = 0
    for h in holds_vector:
        if h > 0:
            res += 1
    return res

def remove_n_holds(hold_vector, holds_to_remove):
    if holds_to_remove == 0:
        return hold_vector, 198
    holds_removed = 0
    result = hold_vector.copy()
    for i in reversed(range(0, 198)):
        if result[i] > 0:
            result[i] = 0
            holds_removed += 1
            if holds_removed >= holds_to_remove:
                return result, i


# Process each element in the data list
for entry in climbs:
    grade = entry["grade"]
    moves_list = entry["moves"]

    hold_vector = [0 for _ in range(0, 198)]

    for move in moves_list:
        index = convert_coordinate_to_int(move["description"])
        if move["isStart"]:
            hold_vector[index] = 1
        elif move["isEnd"]:
            hold_vector[index] = 2
        else:
            hold_vector[index] = 3

    holds_to_remove = random.randint(0,count_holds(hold_vector))
    hold_vector, lowest_hidden = remove_n_holds(hold_vector, holds_to_remove)

    grades.append(convert_grade_to_int(grade))
    partial_climbs.append(np.array(hold_vector))
    answers.append(lowest_hidden)

partial_climbs = np.array(partial_climbs)
partial_climbs_matrix = np.reshape(partial_climbs, (-1, 18, 11))
grades = np.array(grades)
y = to_categorical(np.array(answers), 199)

"""
test performance with two different loss functions + also categorical_crossentropy
"""
def rank_sqrt_loss(prediction, answer):
    correct_index = 0
    number_ranked_above = 0
    for i in answer:
        if answer[i] == 1:
            correct_index = i
    predicted_probability = prediction[correct_index]
    for i in prediction:
        if i > predicted_probability:
            number_ranked_above += 1
    return math.sqrt(number_ranked_above)

def prob_difference_squared_loss(prediction, answer):
    correct_index = 0
    for i in answer:
        if answer[i] == 1:
            correct_index = i
    predicted_probability = prediction[correct_index]
    return (1 - predicted_probability) ^ 2

# test number of neuron in deep layer (probably need more)
# 11 inputs to embedding layer because of grade

"""
to test - best so far
dense layer 1 neurons - 512
dense activation function - relu
dense layer 2 neurons - 256
dropout rate - 0.2
"""

#I tried embedding input_holds but this seems to make it set weird climbs
input_holds = Input(shape=(18,11))
input_grades = Input(shape=(1,))

conv_1 = Conv1D(filters=128, kernel_size=1, activation="relu")(input_holds)
dropout_1 = Dropout(0.3)(conv_1)
conv_2 = Conv1D(filters=64, kernel_size=1, activation="relu")(conv_1)
dropout_2 = Dropout(0.3)(conv_2)

conv_3 = Conv1D(filters=32, kernel_size=1, activation="relu")(conv_2)
flatten = Flatten()(conv_3)

dropout_3 = Dropout(0.3)(flatten)

concat = Concatenate()([dropout_3, input_grades])

dense_1 = Dense(512, activation="relu")(concat)
dropout_4 = Dropout(0.2)(dense_1)

dense_2 = Dense(256, activation="relu")((dropout_4))
dropout_5 = Dropout(0.2)(dense_2)

outputs = Dense(199, activation="softmax")(dropout_5)

"""
loss function - categorical_crosssentropy
optimizer - rmsprop
"""
model = Model([input_holds, input_grades], outputs)
model.compile(optimizer="rmsprop", loss="categorical_crossentropy", metrics=["accuracy"])

"""
epochs - 6
batch size - 128
"""

model.fit([partial_climbs_matrix, grades], y, epochs=8, batch_size=128, validation_split=0.2)

# save test/experimental models to test file and then copy to main file if the model is an improvement
model.save("/Users/seba/rs/graffiti/models/routeset_test.keras")

