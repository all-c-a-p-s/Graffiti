import numpy as np
import json
import pandas as pd

# for some reason Pylance claims it cannot resolve these four imports even though they actually work fine...
from tensorflow.keras.models import Model
from tensorflow.keras.layers import Embedding, Flatten, Dense, Input

# from tensorflow.keras.optimizers import Adam
from tensorflow.keras.utils import to_categorical

from sklearn.model_selection import train_test_split

"""
The code in this file uses a custom model, which is ultimately able to give better results that the Automodel() 
which I originally used (validation loss of ~1.25)
"""

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
moves = []
grades = []

# Process each element in the data list
for entry in climbs:
    grade = entry["grade"]
    moves_list = entry["moves"]

    hold_vector = [0 for _ in range(0, 198)]

    for move in moves_list:
        # Append each move's description and associated grade
        index = convert_coordinate_to_int(move["description"])
        if move["isStart"]:
            hold_vector[index] = 1
        if move["isEnd"]:
            hold_vector[index] = 2
        else:
            hold_vector[index] = 3

    moves.append(hold_vector)
    grades.append(grade)

dataframe = pd.DataFrame({"holds": moves, "grades": grades})

x = np.array(
    [np.array(dataframe["holds"][i]) for i in range(0, len(dataframe["holds"]))]
)
y = to_categorical(np.array(dataframe["grades"].apply(convert_grade_to_int)), 11)

x_train, x_test, y_train, y_test = train_test_split(
    x, y, test_size=0.2, random_state=42
)

"""
TO TEST:
parameter - best so far
diffrent output_dim - 8
different activation function in first dense layer - sigmoid
fist dense layer units - 128
different activation function in output layer - softmax
"""

# embedded layer + deep layer to capture interactions between features
# this seems like a good idea for this purpose where the moves between
# holds are what determine the difficulty of a route
inputs = Input(shape=(198,))
embed = Embedding(input_dim=4, output_dim=8)(inputs)
flatten = Flatten()(embed)
dense = Dense(64, activation="relu")(flatten)
dense_2 = Dense(32, activation="relu")(dense)
outputs = Dense(11, activation="softmax")(dense_2)

"""
TO TEST:
different optimiser - rmsprop
different loss function - categorical crossentropy
"""
model = Model(inputs, outputs)
model.compile(
    optimizer="rmsprop", loss="categorical_crossentropy", metrics=["accuracy"]
)

"""
TO TEST:
number of epochs - 6
batch size - 64
"""
model.fit(x_train, y_train, epochs=6, batch_size=64, validation_data=(x_test, y_test))

accuracy = model.evaluate(x_test, y_test)
print(f"Test accuracy: {accuracy}")

# save test/experimental models to test file and then copy to main file if the model is an improvement
model.save("/Users/seba/rs/graffiti/models/custom_model_test.keras")
