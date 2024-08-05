import tensorflow as tf
import numpy as np
import train

model = tf.keras.models.load_model("model.keras")

def set_route(start_holds, finish_holds, intermediate_holds):
    #set the holds specified to start holds
    r = np.zeros(198)
    for h in start_holds:
        r[train.convert_coordinate_to_int(h)] = 1
    for h in finish_holds:
        r[train.convert_coordinate_to_int(h)] = 2
    for h in intermediate_holds:
        r[train.convert_coordinate_to_int(h)] = 3
    return r

def analyse_output(output):
    #expects a tensor (primtitive type 2d np.array()) to be inputted
    #in my usage of the model.predict() function, this will only ever have one element
    print("------------------")
    max = 0
    best_grade = ""
    for i in range(0,11):
        grade = "v" + str(i + 4)
        probability = str(round(output[0][i] * 100))
        if output[0][i] > max:
            best_grade = grade
            max = output[0][i]
        
        #this is done so that the table is formatted nicely
        if len(grade) == 2 and len(probability) == 1:
            print("|   " + grade + "   |   " + probability + "   |")
        elif len(grade) == 2 and len(probability) == 2:
            print("|   " + grade + "   |   " + probability + "  |")
        elif len(grade) == 2 and len(probability) == 3:
            print("|   " + grade + "   |  " + probability + "  |")
        elif len(grade) == 3 and len(probability) == 1:
            print("|   " + grade + "  |   " + probability + "   |")
        elif len(grade) == 3 and len(probability) == 2:
            print("|   " + grade + "  |   " + probability + "  |")
        elif len(grade) == 3 and len(probability) == 3:
            print("|   " + grade + "  |  " + probability + "  |")


        print("------------------")

    print(f"\nPredicted Grade: {best_grade}")

START_HOLDS = ["j2"]
INTERMEDIATE_HOLDS = ["g7", "a10", "a11", "a13", "g15"]
FINISH_HOLDS = ["h18"]

c = set_route(START_HOLDS, INTERMEDIATE_HOLDS, FINISH_HOLDS)

def run_model():
    input = np.array([c])
    output = model.predict(input)
    analyse_output(output)
