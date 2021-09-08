# pcisph-rs
2D Predictive-Corrective smoothed particle hydrodynamics (SPH) implementation in Rust

Reimplementation of my [previous C++ PCISPH](https://github.com/cerrno/pcisph) repository now including a parallel solver

For further information, see SPH tutorials on [my website](https://lucasschuermann.com/writing), including an introduction to [SPH math](https://lucasschuermann.com/writing/particle-based-fluid-simulation) and a [simple SPH solver](https://lucasschuermann.com/writing/implementing-sph-in-2d).

## Usage
Run with cargo:
```
RUST_LOG=info cargo r --release
```
Press `r` to reset simulation or `space` to add a block of particles

## Demo video
[![Demo video](http://img.youtube.com/vi/_Kxp5dJ7HM8/0.jpg)](http://www.youtube.com/watch?v=_Kxp5dJ7HM8 "Predictive-Corrective Incompressible SPH")

## Note
This solver is not exactly PCISPH, but can be viewed as 1-iteration of SPH relaxation plus sub-stepping. The “prediction-relaxation” scheme of my implementation actually comes mainly from the (much easier to follow) paper "Particle-based Viscoelastic Fluid Simulation”, as opposed to "Predictive-Corrective Incompressible SPH”.
