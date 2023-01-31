#include <stdio.h>
#include <math.h>
#include "particle.h"
#include "boundary.h"
#include "event.h"
#include "config.h"


static double compute_new_position(const double dt, const double pos, const double vel){
  return pos + vel * dt;
}

void integrate_particle_in_time(const double t_new, part_t *particle){
  double *t_old = &(particle->time);
  const double dt = t_new - *t_old;
  double *pos = particle->position;
  double *vel = particle->velocity;
  for(dim_t dim = 0; dim < NDIMS; dim++){
    pos[dim] = compute_new_position(dt, pos[dim], vel[dim]);
  }
  *t_old = t_new;
}

static double compute_dot(const double *vec0, const double *vec1){
  double dot = 0.;
  for(dim_t dim = 0; dim < NDIMS; dim++){
    dot += vec0[dim] * vec1[dim];
  }
  return dot;
}

static double modify_restcoef(const double restcoef, const double norm){
  // enforce coef = 1 when two velocities are almost the same
  //   in order to avoid dt = 0
  const double threshold = 1.e-4;
  return norm < threshold ? 1. : restcoef;
}

static void update_particle_velocities_pp(part_t *p0, part_t *p1){
  // compute new velocities after collision
  const double d0 = p0->density;
  const double d1 = p1->density;
  const double r0 = p0->radius;
  const double r1 = p1->radius;
  const double m0 = compute_p_mass(d0, r0);
  const double m1 = compute_p_mass(d1, r1);
  // inter-particle restitution coefficient
  double restcoef = config.get_double("restcoef_pp");
  // compute normal vector from p0 center to p1 center
  const double *pos0 = p0->position;
  const double *pos1 = p1->position;
  double dpos[NDIMS] = {0.};
  for(dim_t dim = 0; dim < NDIMS; dim++){
    dpos[dim] = pos1[dim] - pos0[dim];
  }
  double normal[NDIMS] = {0.};
  for(dim_t dim = 0; dim < NDIMS; dim++){
    normal[dim] = dpos[dim] / (r0 + r1);
  }
  // manipulate velocity of colliding particles
  double **vel0 = &(p0->velocity);
  double **vel1 = &(p1->velocity);
  // velocity of gravity center of the two particles
  double gvel[NDIMS] = {0.};
  for(dim_t dim = 0; dim < NDIMS; dim++){
    gvel[dim] =
      (m0 * (*vel0)[dim] + m1 * (*vel1)[dim]) / (m0 + m1);
  }
  double dvel[NDIMS] = {0.};
  for(dim_t dim = 0; dim < NDIMS; dim++){
    dvel[dim] = (*vel1)[dim] - (*vel0)[dim];
  }
  const double norm_dvel = sqrt(compute_dot(dvel, dvel));
  restcoef = modify_restcoef(restcoef, norm_dvel);
  //
  double dot = compute_dot(dvel, normal);
  const double factor = - (1. + restcoef) * dot;
  for(dim_t dim = 0; dim < NDIMS; dim++){
    dvel[dim] += factor * normal[dim];
  }
  //
  for(dim_t dim = 0; dim < NDIMS; dim++){
    (*vel0)[dim] = gvel[dim] + (-m1) / (m0 + m1) * dvel[dim];
    (*vel1)[dim] = gvel[dim] + (+m0) / (m0 + m1) * dvel[dim];
  }
  return;
}

int check_next_pp_event(const double tmax, double *time, const part_t *p0, const part_t *p1, part_t **new_p0, part_t **new_p1){
  // particle positions at t lead
  //   x_i^0 + u_i^0 * dt,
  //   x_i^1 + u_i^1 * dt,
  //   where dt = t - fmax(t^0, t^1)
  // L^2 norm between them is
  //   || dx_i + du_i * dt ||^2
  // I am interested in dt
  //   when the norm is equal to (r_0 + r_1)^2,
  // This yields a quadratic equation with respect to dt:
  //   + 1 dot(du_i, du_i) dt^2
  //   + 2 dot(dx_i, du_i) dt
  //   + 1 dot(dx_i, dx_i) - (r_0 + r_1)^2
  //   = 0
  // Instead of solving this equation w.r.t. dt naively,
  //   which suffers from numerical errors and might allow overlap,
  //   I try to minimise the residual of
  //   f(dt) =
  //     + 1 dot(du_i, du_i) dt^2
  //     + 2 dot(dx_i, du_i) dt
  //     + 1 dot(dx_i, dx_i) - (r_0 + r_1)^2,
  //   whose derivative is given as
  //   f'(dt) =
  //     + 2 dot(du_i, du_i) dt
  //     + 2 dot(dx_i, du_i)
  //   by means of Newton-Raphson method
  // Before entering the iterative process,
  //   there are several special cases when we should return earlier
  //   1. coef2 = 0: no solution
  //     indicating two objects are moving with the same speed
  //   2. coef1 >= 0: negative solution
  //     indicating they collided in the past
  // dx_i
  double dpos[NDIMS] = {0.};
  for(dim_t dim = 0; dim < NDIMS; dim++){
    dpos[dim] = p1->position[dim] - p0->position[dim];
  }
  // du_i
  double dvel[NDIMS] = {0.};
  for(dim_t dim = 0; dim < NDIMS; dim++){
    dvel[dim] = p1->velocity[dim] - p0->velocity[dim];
  }
  // coef in front of dt^2
  const double coef2 = compute_dot(dvel, dvel);
  if(coef2 == 0.){
    return 1;
  }
  // coef in front of dt^1
  const double coef1 = 2. * compute_dot(dpos, dvel);
  if(coef1 >= 0.){
    return 1;
  }
  // coef in front of dt^0
  const double coef0 =
    + compute_dot(dpos, dpos)
    - pow(p0->radius + p1->radius, 2.);
  // overlap check
  if(coef0 < 0.){
    printf("%s:%d ", __FILE__, __LINE__);
    printf("%p vs %p, coef0 = % .1e < 0.\n", p0, p1, coef0);
    exit(1);
  }
  // discriminant b * b - 4 a c
  const double dis = coef1 * coef1 - 4. * coef2 * coef0;
  if(dis < 0.){
    // imaginary solution (do not collide)
    return 1;
  }
  // first compute time when two particles collide
  // their new positions are obtained at the same time
  double dt = 0.;
  double pos0[NDIMS] = {0.};
  double pos1[NDIMS] = {0.};
  for(size_t iter = 0; iter < 10; iter++){
    // function
    const double f0 =
      + coef2 * pow(dt, 2.)
      + coef1 * pow(dt, 1.)
      + coef0 * pow(dt, 0.);
    // derivative
    const double f1 =
      + 2. * coef2 * pow(dt, 1.)
      + 1. * coef1 * pow(dt, 0.);
    // tentative dt
    const double dt_ = dt - f0 / f1;
    // tentative positions, check overlap
    double pos0_[NDIMS] = {0.};
    double pos1_[NDIMS] = {0.};
    double dpos_[NDIMS] = {0.};
    for(dim_t dim = 0; dim < NDIMS; dim++){
      pos0_[dim] = compute_new_position(dt_, p0->position[dim], p0->velocity[dim]);
      pos1_[dim] = compute_new_position(dt_, p1->position[dim], p1->velocity[dim]);
      dpos_[dim] = pos1_[dim] - pos0_[dim];
    }
    // distance between particle surfaces
    const double dist =
      sqrt(compute_dot(dpos_, dpos_))
      - p0->radius
      - p1->radius;
    /* printf("NR iter %3zu dist % .7e\n", iter, dist); */
    if(dist < 0.){
      // (almost) overlapped
      // get rid of tentative values
      //   and abort iteration
      break;
    }
    // these values are OK (no overlap yet)
    // accept them as tentative values
    dt = dt_;
    for(dim_t dim = 0; dim < NDIMS; dim++){
      pos0[dim] = pos0_[dim];
      pos1[dim] = pos1_[dim];
    }
  }
  *time = p0->time + dt;
  if(*time > tmax){
    return 1;
  }
  // now I know a solution exists
  // prepare buffers to store new positions and velocities
  *new_p0 = common_calloc(1, sizeof(part_t));
  *new_p1 = common_calloc(1, sizeof(part_t));
  (*new_p0)->density = p0->density;
  (*new_p1)->density = p1->density;
  (*new_p0)->radius  = p0->radius;
  (*new_p1)->radius  = p1->radius;
  (*new_p0)->position = common_calloc(NDIMS, sizeof(double));
  (*new_p0)->velocity = common_calloc(NDIMS, sizeof(double));
  (*new_p1)->position = common_calloc(NDIMS, sizeof(double));
  (*new_p1)->velocity = common_calloc(NDIMS, sizeof(double));
  for(dim_t dim = 0; dim < NDIMS; dim++){
    (*new_p0)->position[dim] = pos0[dim];
    (*new_p1)->position[dim] = pos1[dim];
  }
  for(dim_t dim = 0; dim < NDIMS; dim++){
    (*new_p0)->velocity[dim] = p0->velocity[dim];
    (*new_p1)->velocity[dim] = p1->velocity[dim];
  }
  update_particle_velocities_pp(*new_p0, *new_p1);
  return 0;
}

int check_next_pb_event(const double tmax, double *time, const part_t *p, const bund_t *b, part_t **new_p){
  // events in which the particle is getting out of this cell
  //   are delayed slightly
  // this treatment is to avoid particles being removed from one cell
  //   while not being registered to the neighbouring cell
  //   when crossing the cell boundary
  // this is useful
  //   when the cell size coincides with the diameter of the particle
  const double offset = b->is_outer ? 1.01 * p->radius : p->radius;
  const double pos = p->position[b->dim];
  const double vel = p->velocity[b->dim];
  if(b->dir == DIR_NEG && vel >= 0.){
    // consider negative direction,
    //   while the particle is moving toward positive direction
    return 1;
  }
  if(b->dir == DIR_POS && vel <= 0.){
    // consider positive direction,
    //   while the particle is moving toward negative direction
    return 1;
  }
  double bnd = b->position;
  if(b->sft == DIR_NEG){
    bnd -= offset;
  }else{ // sft == DIR_POS
    bnd += offset;
  }
  double dt = (bnd - pos) / vel;
  if(dt <= 0.){
    // past event
    return 1;
  }
  *time = p->time + dt;
  if(*time > tmax){
    return 1;
  }
  *new_p = common_calloc(1, sizeof(part_t));
  (*new_p)->position = common_calloc(NDIMS, sizeof(double));
  (*new_p)->velocity = common_calloc(NDIMS, sizeof(double));
  (*new_p)->density = p->density;
  (*new_p)->radius  = p->radius;
  const double restcoef = modify_restcoef(config.get_double("restcoef_pw"), fabs(vel));
  for(dim_t dim = 0; dim < NDIMS; dim++){
    const double new_pos = b->dim == dim ? bnd : compute_new_position(dt, p->position[dim], p->velocity[dim]);
    const double new_vel = b->dim == dim ? restcoef * (-1. * vel) : p->velocity[dim];
    (*new_p)->position[dim] = new_pos;
    (*new_p)->velocity[dim] = new_vel;
  }
  return 0;
}

