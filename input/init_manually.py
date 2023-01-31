import numpy as np


dtype_size_t = "<u8"
dtype_double = "<f8"

np.save("iter.npy", np.array(0, dtype=dtype_size_t))
np.save("time.npy", np.array(0., dtype=dtype_double))
np.save("lengths.npy", np.array([4., 4.], dtype=dtype_double))
np.save("nparticles.npy", np.array(1, dtype=dtype_size_t))
np.save("densities.npy", np.array([1.], dtype=dtype_double))
np.save("radii.npy", np.array([1.], dtype=dtype_double))
np.save("positions_0.npy", np.array([2.2], dtype=dtype_double))
np.save("positions_1.npy", np.array([2.3], dtype=dtype_double))
np.save("velocities_0.npy", np.array([0.5], dtype=dtype_double))
np.save("velocities_1.npy", np.array([0.5], dtype=dtype_double))
