###############################################################
`Collision <https://naokihori.github.io/Collision/index.html>`_
###############################################################

|License|_ |LastCommit|_ |Deploy|_

.. |License| image:: https://img.shields.io/github/license/NaokiHori/Collision
.. _License: https://opensource.org/licenses/MIT

.. |LastCommit| image:: https://img.shields.io/github/last-commit/NaokiHori/Collision/main
.. _LastCommit: https://github.com/NaokiHori/Collision/commits/main

.. |Deploy| image:: https://github.com/NaokiHori/Collision/actions/workflows/deploy.yml/badge.svg?branch=main
.. _Deploy: https://github.com/NaokiHori/Collision/actions/workflows/deploy.yml

.. image:: https://img.shields.io/badge/youtube-%23EE4831.svg?&style=for-the-badge&logo=youtube&logoColor=white
   :target: https://youtu.be/k8hbpa3CsCg
   :width: 10%

.. image:: https://github.com/NaokiHori/Collision/blob/main/thumbnail.jpg
   :target: https://youtu.be/k8hbpa3CsCg
   :width: 100%

********
Overview
********

Event-driven simulation of many colliding particles.

***********
Quick start
***********

Visit `the main page <https://naokihori.github.io/Collision/index.html>`_.

Several URL parameters are optionally available:

* ``length``: size of the domain

* ``nitems``: number of particles

* ``rate``: draw rate (the smaller the smoother but the more demanding)

The default configuration is equivalent to ``length = 192``, ``nitems = 8192``, and ``rate = 0.1``, namely:

``https://naokihori.github.io/Collision/index.html?length=192&nitems=8192&rate=0.1``.

The particle radii are fixed to ``0.5`` for now, and the restitution coefficient between particles is set to ``0.99``.
I assume the domain is squared-shape and the periodic and wall-bounded conditions are imposed in the horizontal and the vertical directions, respectively.
Also, the number of particles is clipped if the volume fraction exceeds ``40%``.

******
Method
******

In this project, I aim at simulating finite-sized particles following Newtonian mechanics with overlaps prohibited.
To this end, I need to properly detect all inter-particle collisions, which inherently requires O(N_p^2) operations, where N_p is the number of particles.
It is necessary to reduce this cost somehow to treat say millions of particles.

#. Event-driven approach

   One possible way to handle inter-particle interactions is to introduce repulsive forces between particles, such that they repel to each other when too close.
   Each particle motion is given by ordinary-differential equations, which can be solved by an appropriate time marcher (e.g. Runge-Kutta method).
   The repellent force is, however, arbitrarily chosen and to be nicely designed.
   To avoid these problems, I adopt the so-called event-driven approach with the cost of O(N_p) for the collision detection.
   Moreover, this is an ODE-free method, which is advantageous to eliminate the numerical errors of the time-marching schemes.

#. Cell method

   It is desired only to consider particles near-by, which is achieved by the so-called cell method splitting the whole domain into many cells.
   This leads to the cost of O(N_p^2 / N_c) where N_c is the number of cells.
   Since N_c can be changed arbitrarily, the cost to detect collisions results in O(1).
   Instead, cell-particle events (particles passing through the cell boundaries) are to be considered.
   However, the cost to update events for each step remains O(1).

#. Scheduler

   Although the events are separately queued for each cell, it is necessary to fetch the latest event among all cells, which requests O(N_c) operations if implemented naively (iterate over all cells, find the latest one).
   To reduce the cost, a minimum binary heap with the cost of O(log N_c) to be balanced is adopted.

#. Local time

   Updating particle positions and velocities requests O(N_p) operations, and doing this process for each step is verbose.
   This is mitigated by introducing particle-based local time, so that particles are only updated when they are involved.

***************
Acknowledgement
***************

I would like to thank Prof. Stefan Luding for a stimulating lecture in a JMBC course *Particle-based modeling techniques*.

