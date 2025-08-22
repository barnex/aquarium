use std::iter::zip;

fn main() {
    //run(diffuse_nonlinear)
    //run(diffuse_inertia);
    //run(lwave);
    run(borewave);
}

fn run(f: impl Fn(&mut [f32], &mut [f32], f32)) {
    let l = 20;
    let mut q = vec![0.0; l];
    let mut v = vec![0.0; l];

    for i in 2..3 {
        q[i] = 1.0;
        v[i] = 1.0;
    }

    let ticks = 10;
    let output_every = 1;
    let dt = 0.1;

    let ticks = (ticks as f32 / dt) as usize;
    for t in 0..ticks {
        let want_output = t % output_every as usize == 0;
        //let want_output = t == 30;
        if want_output {
            zip(&q, &v).enumerate().for_each(|(i, (q, v))| println!("{i} {t} {q} {v}"));
            println!();
        }

        f(&mut q, &mut v, dt);
    }
}

fn lwave(u: &mut [f32], v: &mut [f32], dt: f32) {
    assert_eq!(u.len(), v.len());
    let n = u.len();
    let mut u2 = vec![0.0; u.len()];
    let c = 1.0;
    let dx = 1.0;

    // Update velocity (leapfrog)
    for i in 1..(n - 1) {
        let laplacian = (u[i - 1] - 2.0 * u[i] + u[i + 1]) / (dx * dx);
        v[i] += dt * c * c * laplacian;
    }

    // Free-end boundary conditions (Neumann: zero derivative)
    v[0] += dt * c * c * (u[1] - u[0]) / (dx * dx);
    v[n - 1] += dt * c * c * (u[n - 2] - u[n - 1]) / (dx * dx);

    // Update displacement
    for i in 0..n {
        u2[i] = u[i] + dt * v[i];
    }
    u.clone_from_slice(&u2);
}

fn borewave(h: &mut [f32], f: &mut [f32], dt: f32) {
    let n = h.len();
    let mut h2 = vec![0.0; h.len()];
    let mut f2 = vec![0.0; f.len()];

    // propagate flux
    //
    //          h
    //        +----+
    //        |    |
    //        |    |
    //        | -> |
    //        | v  |
    //--------+----+-------------
    for i in 0..n {
        debug_assert!(h[i] >= 0.0);
        //debug_assert!(h[i] <= 1.0);

        // water flows into cell index `sink` (left or right neighbor)
        // cannot flow out of bounds
        // velocity stops dead at boundaries.
        let sink = if f[i] > 0.0 && i < (n - 1) {
            i + 1 // right
        } else if f[i] < 0.0 && i > 0 {
            i - 1 //left
        } else {
            f[i] = 0.0; // stop velocity at boundary
            continue;
        };

        // amount of water that flows into neighbor
        // cannot be more than all water in cell.
        let bucket = (dt * h[i] * f[i].abs()).clamp(0.0, h[i]);

        h2[i] = h[i] - bucket;
        h2[sink] = h[sink] + bucket;
    }

    f2.clone_from_slice(&f);

    h.clone_from_slice(&h2);
    f.clone_from_slice(&f2);
}

fn diffuse_inertia(q: &mut [f32], v: &mut [f32], dt: f32) {
    let l = q.len();
    let mut q2 = q.to_vec();
    let mut v2 = v.to_vec();

    let g = 0.25;
    let arne = 0.1;

    //       +----+
    //       |    |
    //  +----+    |
    //  |    |    |
    //  |Lj  |Lj+1|

    for j in 0..(l - 1) {
        let left = j;
        let rght = j + 1;
        let delta_l = -g * dt * (q[rght] - q[left]);

        if delta_l > 0.0 {
            // left to right
            let delta_q = delta_l.abs().clamp(0.0, q[left]);

            let delta_v = arne * delta_q;
            let delta_momentum = delta_q * delta_v;

            let mut momentum_rght = q[rght] * v[rght];
            momentum_rght += delta_momentum;

            q2[left] -= delta_q;
            q2[rght] += delta_q;

            if q2[rght] != 0.0 {
                v[rght] = momentum_rght / q2[rght];
            } else {
                v[rght] = 0.0
            }
        } else {
            // right to left
            let delta_q = delta_l.abs().clamp(0.0, q[rght]);

            let delta_v = arne * delta_q;
            let delta_momentum = -delta_q * delta_v; // 

            let mut momentum_left = q[left] * v[left];
            momentum_left += delta_momentum;

            q2[rght] -= delta_q;
            q2[left] += delta_q;

            if q2[left] != 0.0 { v[left] = momentum_left / q2[left] } else { v[left] = 0.0 }
        }
    }

    q.clone_from_slice(&q2);
}

fn diffuse_nonlinear(q: &mut [f32], _: &mut [f32], h: f32) {
    let l = q.len();
    let mut q2 = vec![0.0; l];

    const L0: f32 = 0.1;

    for i in 1..(l - 1) {
        let delta_l = 0.5 * h * (q[i - 1] - q[i]).abs().powf(2.0) * (q[i - 1] - q[i]).signum() / L0;
        let delta_r = 0.5 * h * (q[i + 1] - q[i]).abs().powf(2.0) * (q[i + 1] - q[i]).signum() / L0;

        q2[i] = q[i] + delta_l + delta_r;
    }

    q.clone_from_slice(&q2);
}

fn diffuse(q: &mut [f32], _: &mut [f32], h: f32) {
    let l = q.len();
    let mut q2 = vec![0.0; l];

    for i in 1..(l - 1) {
        let delta_l = 0.5 * h * (q[i - 1] - q[i]);
        let delta_r = 0.5 * h * (q[i + 1] - q[i]);

        q2[i] = q[i] + delta_l + delta_r;
    }

    q.clone_from_slice(&q2);
}
