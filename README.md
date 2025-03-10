# [Collision](https://naokihori.github.io/Collision/index.html)

![License](https://img.shields.io/github/license/NaokiHori/Collision)
[![Last Commit](https://img.shields.io/github/last-commit/NaokiHori/Collision/main)](https://github.com/NaokiHori/Collision/commits/main)
[![Deploy](https://github.com/NaokiHori/Collision/actions/workflows/deploy.yml/badge.svg?branch=main)](https://github.com/NaokiHori/Collision/actions/workflows/deploy.yml)

[![YouTube](https://img.shields.io/badge/youtube-%23EE4831.svg?&style=for-the-badge&logo=youtube&logoColor=white)](https://youtu.be/k8hbpa3CsCg)

[![Cover Image](https://github.com/NaokiHori/Collision/blob/main/cover.jpg)](https://youtu.be/k8hbpa3CsCg)

## Overview

Event-driven simulator of many colliding particles.

## Quick start

Visit [the main page](https://naokihori.github.io/Collision/index.html).

Several URL parameters are optionally available on initializing the solver:

- `length`: size of the domain (the longer of the domain width and the domain height)
- `aspect-ratio`: the ratio of the domain width to the domain height.
- `nitems`: number of particles
- `rate`: draw rate (the smaller the smoother but the more demanding)

The default values (`length = 256`, `rate = 0.05`, `aspect-ratio` being identical to the screen aspect) are used if the corresponding parameter is not specified.
The number of particles `nitems` is given by `width * height / 6` by default to reasonably populate the domain.
Note that it is clamped to enforce the volume fraction being less than `40%`, even when a huge value is assigned.

The domain is assumed to be periodic in the horizontal direction and wall-bounded conditions are imposed in the vertical direction.
The particle radii are fixed at `0.5`, and the restitution coefficient between particles is set to `0.99`.
Each particle stores `temperature`-like information which are exchanged on the collision events, which is to mimic thermal convections by giving pseudo buoyancy force in the vertical direction.

## Method

In this project, I aim at simulating finite-sized particles following Newtonian mechanics with overlaps prohibited.
To this end, I need to properly detect all inter-particle collisions, which inherently requires `O(N_p^2)` operations, where `N_p` is the number of particles.
It is necessary to reduce this cost in order to handle, say, millions of particles.

1. **Event-driven approach**

   One possible way to handle inter-particle interactions is to introduce repulsive forces between particles, such that they repel each other when too close.
   Each particle's motion is given by ordinary-differential equations, which can be solved by an appropriate time marcher (e.g., Runge-Kutta method).
   However, the repellent force is arbitrarily chosen and needs to be carefully designed.
   To address these issues, I adopt the so-called event-driven approach, which has a collision detection cost of `O(N_p)`.
   Moreover, this is an ODE-free method, which eliminates the numerical errors of time-marching schemes.

1. **Cell method**

   To only consider nearby particles, the domain is split into many cells using the so-called cell method.
   This reduces the cost to `O(N_p^2 / N_c)`, where `N_c` is the number of cells.
   Since `N_c` can be chosen arbitrarily, the cost to detect collisions results in `O(1)`.
   However, cell-particle events (particles passing through cell boundaries) also need to be considered.
   The cost to update events for each step remains `O(1)`.

1. **Scheduler**

   Events are queued separately for each cell, but fetching the latest event among all cells requires `O(N_c)` operations if implemented naively.
   To reduce this cost, a minimum binary heap with a cost of `O(log N_c)` is adopted.

1. **Local time**

   Updating particle positions and velocities requires `O(N_p)` operations, and doing this process for each step is verbose.
   This is mitigated by introducing particle-based local time so that particles are only updated when they are involved.

## Try locally

Install the following:

- [`npm`](https://www.npmjs.com)
- [`rust`](https://www.rust-lang.org)
- [`cargo`](https://doc.rust-lang.org/stable/cargo/)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)

Build all:

```bash
npm install
wasm-pack build --release --target web
```

Launch a server:

```
npm run dev
```

Visit `http://localhost:5173`.

## Acknowledgement

I would like to thank Prof. Stefan Luding for a stimulating lecture in a JMBC course _Particle-based modeling techniques_.
