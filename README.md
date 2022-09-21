# Physics-Pi-Calculator

A somewhat simple physics simulation that calculates the digits of pi by counting the collisions between two rectangles inside a 2D space.
This was made in Rust with the [bevy](https://github.com/bevyengine/bevy) and [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon) crates. 

The idea was taken from the [3Blue1Brown](https://www.youtube.com/c/3blue1brown) video on the same topic. [Watching it](https://youtu.be/jsYwFizhncE) is recomended in order to understand the logic behind why this happens.

# Crates

> The [bevy engine](https://github.com/bevyengine/bevy) is used in order to generate the window and spawn the objects.  
> The [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon) crate is used in order to draw the two rectangles.

# Limitations

There are some limitations with this spesific simulation:
> - Computing more than **four** digits will cause the program to panic at an **overflow error**.
> - An **unrealistic amount of time** is required in order to compute the **fourth** digit. 

# Additional Notes

> - This is not a **perfomance efficient** neither a **practical** way to calculate the digits of pi. It is just a simple simulation that I made for fun while figuring out [bevy](https://github.com/bevyengine/bevy).  
> - Check out the comments within the source code in order to get a better understanding.
