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

The default configuration is equivalent to ``length = 192``, ``nitems = 8192``, and ``rate = 0.25``, namely:

``https://naokihori.github.io/Collision/index.html?length=192&nitems=8192&rate=0.25``.

Note that the particle radii are fixed to ``0.5`` for now.
Also, the number of particles is clipped if the volume fraction exceeds ``40%``.

******
Method
******

#. Cell method

   All inter-particle collision events are considered, which requires essentially O(N_p^2) operations, where N_p is the number of particles.
   To drop the cost, the so-called event-driven approach combined with the cell method is used, which leads to the cost of O(N_p^2 / N_c) where N_c is the number of cells.

#. Scheduler

   Scheduling requests O(N_c) operations if implemented naively.
   To reduce the cost, a minimum binary heap with the cost of O(log N_c) for insertions and deletions is adopted.

#. Local time

   Updating particle positions and velocities requests O(N_p) operations.
   This is mitigated by introducing particle-based local time, so that particles are only synchronised when drawn.

***************
Acknowledgement
***************

I would like to thank Prof. Stefan Luding for his stimulating lecture in a JMBC course *Particle-based modeling techniques*.

