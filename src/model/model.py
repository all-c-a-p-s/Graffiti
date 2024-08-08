import tensorflow as tf
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


START_HOLDS = ["a5", "c5"]
INTERMEDIATE_HOLDS = ["b7", "d9", "c11", "d13", "e15", "d16"]
FINISH_HOLDS = ["c18"]

c = set_route(START_HOLDS, INTERMEDIATE_HOLDS, FINISH_HOLDS)


def run_model():
    input = np.array([c])
    output = model.predict(input)
    print(analyse_output(output))
    # print(output)
    return output


run_model()
