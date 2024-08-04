import os

os.environ["KERAS_BACKEND"] = "tensorflow"

import numpy as np
import tensorflow as tf
import keras
from keras import utils
import pandas as pd
import json


"""
INPUT: list of integers which represent the holds in the route in order
OUTPUT: integer 4-14 which is the grade of the climb
"""

#first some preprocessing code:

def convert_coordinate_to_int(coordinate) -> int:
    letter = coordinate[0]
    column = ord(letter.upper()) - ord("A")
    row = int(coordinate[1:])
    return 11 * (row - 1) + column

#used to convert string labels to tensor-convertible integers
def convert_grade_to_int(grade) -> int:
    match grade:
        case "6B" | "6B+":
            return 4
        case "6C" | "6C+":
            return 5
        case "7A":
            return 6
        case "7A+":
            return 7
        case "7B" | "7B+":
            return 8
        case "7C":
            return 9
        case "7C+":
            return 10
        case "8A":
            return 11
        case "8A+":
            return 12
        case "8B":
            return 13
        case "8B+":
            return 14
        case _:
            print(f"error converting grade to int: unexpected grade found {grade}")
            exit(1)


path = "json/2016.json"

with open(path) as file:
    json_data = json.load(file)

#load the climbs from the data field
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
        hold_vector[convert_coordinate_to_int(move["description"])] = 1
    
    moves.append(hold_vector)
    grades.append(grade)

dataframe = pd.DataFrame({
    "holds": moves,
    "grades": grades
})

dataframe["holds"] = np.array([np.array(dataframe["holds"][i]) for i in range(0, len(dataframe["holds"]))])
dataframe["grades"] = dataframe["grades"].apply(convert_grade_to_int)

val_dataframe = dataframe.sample(frac=0.2, random_state=1337)
train_dataframe = dataframe.drop(val_dataframe.index)

def dataframe_to_dataset(dataframe):
    dataframe = dataframe.copy()

    features = np.array(dataframe.pop("holds"))
    labels = np.array(dataframe.pop("grades"))

    dataset = tf.data.Dataset.from_tensor_slices((features,  labels))
    dataset = dataset.shuffle(buffer_size=len(dataframe))
    return dataset

train_dataset = dataframe_to_dataset(train_dataframe)
val_dataset = dataframe_to_dataset(val_dataframe)