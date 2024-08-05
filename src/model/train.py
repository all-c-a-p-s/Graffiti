import os

os.environ["KERAS_BACKEND"] = "tensorflow"

import numpy as np
#import tensorflow as tf
import pandas as pd
import json
import autokeras as ak
from sklearn.model_selection import train_test_split

# TODO: differentiate between startholds, end holds and normal holds

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
        index = convert_coordinate_to_int(move["description"])
        if move["isStart"]:
            hold_vector[index] = 1
        if move["isEnd"]:
            hold_vector[index] = 2
        else:
            hold_vector[index] = 3
    
    moves.append(hold_vector)
    grades.append(grade)

dataframe = pd.DataFrame({
    "holds": moves,
    "grades": grades
})

x = np.array([np.array(dataframe["holds"][i]) for i in range(0, len(dataframe["holds"]))])
y = np.array(dataframe["grades"].apply(convert_grade_to_int))

def train():
    x_train, x_test, y_train, y_test = train_test_split(x, y, test_size=0.2, random_state=42)

    input_node = ak.Input()
    output_node = ak.ClassificationHead(num_classes=11)(input_node)
    
    model = ak.AutoModel(inputs=input_node, outputs=output_node, overwrite=True)
    
    #stops in the case that validation loss fails to improve for 10 epochs
    #so it is ok to leave for default (1000) epochs and not worry about overfitting
    model.fit(x_train, y_train)
    
    accuracy = model.evaluate(x_test, y_test)
    print(f"Test accuracy: {accuracy}")
    
    model.export_model().save("model.keras")