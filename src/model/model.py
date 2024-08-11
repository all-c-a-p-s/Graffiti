import tensorflow as tf
import tf2onnx
import numpy as np
import auto

model = tf.keras.models.load_model("/Users/seba/rs/graffiti/models/custom_model.keras")


def set_route(start_holds, finish_holds, intermediate_holds):
    # set the holds specified to start holds
    r = np.zeros(198)
    for h in start_holds:
        r[auto.convert_coordinate_to_int(h)] = 1
    for h in finish_holds:
        r[auto.convert_coordinate_to_int(h)] = 2
    for h in intermediate_holds:
        r[auto.convert_coordinate_to_int(h)] = 3
    return r


def analyse_output(output):
    # expects a tensor (primtitive type 2d np.array()) to be inputted
    # in my usage of the model.predict() function, this will only ever have one element
    s = ""
    s += "------------------\n"
    max = 0
    best_grade = ""
    for i in range(0, 11):
        grade = "v" + str(i + 4)
        probability = str(round(output[0][i] * 100))
        if output[0][i] > max:
            best_grade = grade
            max = output[0][i]

        # this is done so that the table is formatted nicely
        if len(grade) == 2 and len(probability) == 1:
            s += "|   " + grade + "   |   " + probability + "   |\n"
        elif len(grade) == 2 and len(probability) == 2:
            s += "|   " + grade + "   |   " + probability + "  |\n"
        elif len(grade) == 2 and len(probability) == 3:
            s += "|   " + grade + "   |  " + probability + "  |\n"
        elif len(grade) == 3 and len(probability) == 1:
            s += "|   " + grade + "  |   " + probability + "   |\n"
        elif len(grade) == 3 and len(probability) == 2:
            s += "|   " + grade + "  |   " + probability + "  |\n"
        elif len(grade) == 3 and len(probability) == 3:
            s += "|   " + grade + "  |  " + probability + "  |\n"

        s += "------------------\n"

    s += f"\nPredicted Grade: {best_grade}"
    return s

def hold_index_to_coordinate(index):
    column_number = index % 11
    letter = chr(ord('a') + column_number)
    row_number = str(int((index - column_number) / 11) + 1)
    return letter + row_number

START_HOLDS = []
INTERMEDIATE_HOLDS = []
FINISH_HOLDS = []

c = set_route(START_HOLDS, INTERMEDIATE_HOLDS, FINISH_HOLDS)


def run_model():
    input = np.array([c])
    output = model.predict(input)
    print(analyse_output(output))
    # print(output)
    return output


def save_model_directory():
    model = tf.keras.models.load_model(
        "/Users/seba/rs/graffiti/models/custom_model.keras"
    )
    model.export("/Users/seba/rs/graffiti/models/")


def save_as_onnx():
    model = tf.keras.models.load_model(
        "/Users/seba/rs/graffiti/models/custom_model.keras"
    )
    onnx_model, _ = tf2onnx.convert.from_keras(model)
    with open("/Users/seba/rs/graffiti/models/custom_model.onnx", "wb") as f:
        f.write(onnx_model.SerializeToString())

def save_routeset_as_onnx():
    model = tf.keras.models.load_model(
        "/Users/seba/rs/graffiti/models/routeset/routeset.keras"
    )

    spec = (tf.TensorSpec((None, 18, 11), tf.float32, name="input_holds"),
        tf.TensorSpec((None, 1), tf.float32, name="input_grades"))
    model_proto, _ = tf2onnx.convert.from_keras(model, input_signature=spec, output_path="/Users/seba/rs/graffiti/models/routeset/routeset.onnx")

#to give more variety in routes, maybe can choose from some of the most likely holds
def run_routeset(grade):
    model = tf.keras.models.load_model(
        "/Users/seba/rs/graffiti/models/routeset.keras"
    )
    input = list(c)
    output = model.predict([np.reshape(np.array([input]), (-1, 18, 11)), np.array([grade])])
    probabilities = output[0]

    highest_probability = 0
    most_likely_index = 0
    for i in range(0, 199):
        if i == 198 and probabilities[i] > highest_probability:
            return None
        if probabilities[i] > highest_probability and c[i] == 0:
            highest_probability = probabilities[i]
            most_likely_index = i

    print(highest_probability)
    return most_likely_index

def set_a_route(grade):
    next_hold = run_routeset(grade)
    start_holds = 0
    while next_hold != None:
        if start_holds == 0:
            c[next_hold] = 1
        elif next_hold >= auto.convert_coordinate_to_int("a18"):
            c[next_hold] = 2
        else:
            c[next_hold] = 3
        next_hold = run_routeset(grade)
    
    holds = []    
    for i in range(0, 198):
        if c[i] > 0:
            holds.append(hold_index_to_coordinate(int(i)))
    return holds
    

# run_model()
# print(set_a_route(4))
save_routeset_as_onnx()

