#include <stdio.h>
#include "common.h"
#include "config.h"
#include "particle.h"
#include "event.h"
#include "cell.h"
#include "logging.h"
#include "io.h"
#include "output.h"
#include "debug.h"


int main(void){
  time_t wtimes[2];
  wtimes[0] = common_get_current_time();
  // activate environmental variable loader
  config.load();
  // parameters
  const double tmax = config.get_double("tmax");
  const double wtmax = config.get_double("wtmax");
  size_t iter = 0;
  double time = 0.;
  load_scalar(config.get_string("input_directory"), "iter", sizeof(size_t), dtype_size_t, &iter);
  load_scalar(config.get_string("input_directory"), "time", sizeof(double), dtype_double, &time);
  // particles
  size_t nparticles = 0;
  part_t *particles = NULL;
  init_particles(time, &nparticles, &particles);
  // cells
  size_t ncells = 0;
  cell_t **cells = NULL;
  init_cells(&ncells, &cells, nparticles, particles);
  // events
  init_events(tmax, ncells, cells);
  // initialisation is completed
  printf("initialisation is completed\n");
  // prepare for main update process
  // output initial field
  logging(time, nparticles, particles, ncells, cells);
  output(iter, time, nparticles, particles);
  // schedule next logging and saving
  const double save_rate = config.get_double("save_rate");
  const double log_rate  = config.get_double("log_rate");
  double save_next = time + save_rate;
  double log_next  = time + log_rate;
  // main update process
  while(1){
    // deal with event
    //   1. extract upcoming event
    //   2. update relevant objects
    //   3. reschedule events
    //   4. update time
    time = process_event(tmax, cells);
    iter += 1;
    // terminate
    if(time > tmax){
      printf("time limit exceeded\n");
      break;
    }
    wtimes[1] = common_get_current_time();
    if(wtimes[1] - wtimes[0] > wtmax){
      printf("wall time limit exceeded\n");
      break;
    }
    // post-processing
    if(time > log_next){
      printf("time % .3e iter %16zu\n", time, iter);
      for(size_t n = 0; n < nparticles; n++){
        integrate_particle_in_time(time, particles + n);
      }
      logging(time, nparticles, particles, ncells, cells);
      log_next += log_rate;
    }
    if(time > save_next){
      for(size_t n = 0; n < nparticles; n++){
        integrate_particle_in_time(time, particles + n);
      }
      output(iter, time, nparticles, particles);
      save_next += save_rate;
    }
  }
  // clean-up
  finalise_cells(ncells, cells);
  finalise_particles(nparticles, particles);
  // clean-up env vars
  config.unload();
  return 0;
}

