# Raytracing

This project implements and extends the "Raytracing in One Weekend" <https://raytracing.github.io/> book in the Rust language as a way of learning Rust and also raytracing.  I use enums for the main variable structures such as shape abstractions (Hittables), materials, and signed distance fields (my own addition).  Initially I used traits to implement these but I found sharing the scene between a parallel iteraor (using the Rayon crate) was easier with the enum implementation.

Look at the output folder for some examples of shiny renders that I've created along the way!
