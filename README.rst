#########
Collision
#########

|License|_ |LastCommit|_ |CI|_

.. |License| image:: https://img.shields.io/github/license/NaokiHori/Collision
.. _License: https://opensource.org/licenses/MIT

.. |LastCommit| image:: https://img.shields.io/github/last-commit/NaokiHori/Collision/main
.. _LastCommit: https://github.com/NaokiHori/Collision/commits/main

.. |CI| image:: https://github.com/NaokiHori/Collision/actions/workflows/ci.yml/badge.svg?branch=main
.. _CI: https://github.com/NaokiHori/Collision/actions/workflows/ci.yml

.. image:: https://img.shields.io/badge/youtube-%23EE4831.svg?&style=for-the-badge&logo=youtube&logoColor=white
   :target: https://youtu.be/k8hbpa3CsCg
   :width: 10%

.. image:: https://github.com/NaokiHori/Collision/blob/main/.github/snapshot2d.png
   :target: https://youtu.be/k8hbpa3CsCg
   :width: 100%

.. image:: https://img.shields.io/badge/youtube-%23EE4831.svg?&style=for-the-badge&logo=youtube&logoColor=white
   :target: https://youtu.be/yQFA8boM4cE
   :width: 10%

.. image:: https://github.com/NaokiHori/Collision/blob/main/.github/snapshot3d.jpg
   :target: https://youtu.be/yQFA8boM4cE
   :width: 100%

********
Overview
********

Event-driven simulation of colliding particles in N-dimensional (N = 2,3,4,...) domains.

**********
Motivation
**********

Just for fun.

**********
Dependency
**********

   * C compiler

   * GNU make (not essential but recommended)

***********
Quick start
***********

#. Initialise particles

   A simple package to initialise particles without overlap is included:

   .. code-block::

      $ cd input/RandomGenerator
      $ make all
      $ sh exec.sh
      $ cd ../..

   Domain size and the number of particles should be specified in ``input/RandomGenerator/exec.sh``
   Do not forget to go back to the root directory after being initialised.

#. Compile and execute

   .. code-block::

      $ make output
      $ make all
      $ sh exec.sh

   * Initial condition
   * Maximum durations (both in terms of simulation time units and real time units in seconds)
   * Restitution coefficients (inter-particle and particle-wall cases)
   * Logging / output schedulers (in terms of simulation time units)

   should be specified in ``exec.sh``.

Results (particle positions, velocities, etc.) are dumped to files under ``output/save/`` in ``NPY`` format, which can be easily opened using `Python <https://www.python.org>`_ with `NumPy <https://numpy.org>`_ package, whose visualisation leads

.. image:: https://github.com/NaokiHori/Collision/blob/main/.github/example.gif
   :width: 100%

Total momentum and energy are monitored and written under ``output/log/``.

*********
Dimension
*********

Please modify ``NDIMS`` given in the ``Makefile`` to a desired (2, 3, 4, ...) value.
For the time being, recompiling the whole source codes is necessary.

********
Examples
********

Several examples can be found in a `YouTube playlist <https://www.youtube.com/playlist?list=PL_FWWE4JjBnyoVE1HbFDqEdsDkQlpzkPq>`_.

******
Method
******

All collision events are considered, which requires essentially O(N_p^2) operations, where N_p is the number of particles.

Event-driven approach combined with the cell method is used, which drops the cost to O(N_p^2 / N_c), where N_c is the number of cells.

Scheduling requests O(N_c) operations if I implement naively, which is reduced by adopting minimum binary heap (O(log N_c) for insertion and deletion, O(1) for fetching the latest event).

Updating particle positions and velocities requests O(N_p) operations, which is mitigated to O(N_p / N_c) by introducing particle-based local time.

Documentation is in preparation.

****
Note
****

Implementation is incomplete, and thus particles can penetrate to each other when the local volume fraction is extremely dense and the restitution coefficients are less than unity.
Program aborts when this kind of inconsistency is detected.

****************
3D Visualisation
****************

`The other library <https://github.com/NaokiHori/Scatter3D>`_ is used to visualise three-dimensional cases easily.

***************
Acknowledgement
***************

I would like to thank Prof. Stefan Luding for his stimulating lecture in a JMBC course *Particle-based modeling techniques*.

