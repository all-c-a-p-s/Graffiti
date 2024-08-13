# Graffiti
Graffiti is a project which aims to integrate the power of AI into climbing training by providing tools to:
- predict the grade of a given route on the [moonboard](http://moonboard.com)
- set its own original routes on the moonboard

Imagine you've come up with the perfect start to a moonboard route, but are struggling to come up with ideas for how to complete the route. Simply the holds you have so far and the desired grade of the route, and generate ideas for the rest of the route with the AI model. Access the website [here](https://all-c-a-p-s.github.io/Graffiti/).

![Image 11-08-2024 at 21 30](https://github.com/user-attachments/assets/1462e2c4-b043-4eff-b1d1-daee710c5ffd)


## The Grade Prediction Model
The grade prediction model is trained on the dataset of existing moonboard climbs, using embedding and a DNN with 2 hidden layers. At present, it is able to guess the grade of a route with roughly 50 percent accuracy. According to [an excellent paper](https://ar5iv.labs.arxiv.org/html/2311.12419) on this subject, this is slightly better than human accuracy.

## The Routesetting Model
The routesetting model is trained by removing random numbers of holds from existing climbs on the moonboard, and training it to predict which hold is most likely to be next in the route, given the current holds in the route. It then generates routes by repeatedly predicting the most likely next hold until it arrives at the top of the board. This means that it can be used to "complete" the setting of routes from any configuration of the moonboard. It uses a CNN with 5 hidden layers.


## TODOs
- some randomisation in routesetting for variety
- if possible, fix the issue that my model almost never guesses a grade harder than v11 because climbs with grades > v11 are so rare
- more experimentation with different models/architectures
- expand the project to use differrent kinds of board (2019 moonboard, Kilter Board, Tension Board etc.)
- try out more ideas involving climbing and AI (reading routes from images, setting routes on non-standardised walls)

## Contributing
Not accepting code contributions as I may end up using this for a school project.
However, since I am very new to ML, so I would welcome any suggestions on how to improve the architecture of the model.
