#############
3D visualiser
#############

.. image:: https://github.com/NaokiHori/Collision/blob/main/.github/snapshot3d.jpg
   :width: 100%

********
Overview
********

Simple, library-free and CPU-based renderer of spherical point cloud.

**********
Motivation
**********

To visualise point clouds (more than 10^6 particles) like above and create bunch of images easily and quickly.
ParaView, PyVista were not suitable because of some reasons.

**********
Dependency
**********

   * C compiler

   * GNU make (not essential but recommended)

***********
Quick start
***********

.. code-block::

   $ make all
   $ ./a.out

which creates ``image.ppm``.

*********
Reference
*********

`Ray Tracing in One Weekend <https://raytracing.github.io/books/RayTracingInOneWeekend.html>`_

`PyVista - Cameras <https://docs.pyvista.org/api/core/camera.html>`_

