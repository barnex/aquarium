use std::iter::zip;

fn main() {
    //run(diffuse_nonlinear)
    //run(diffuse_inertia);
    //run(lwave);
    //run(borewave);
    run(borewave2);
}

fn run(f: impl Fn(&mut [f32], &mut [f32], f32)) {
    let l = 100;
    let mut q = vec![0.0; l];
    let mut ve = vec![0.0; l];

    for i in 0..5 {
        q[i] = 0.5;
        ve[i] = 1.0;
    }

    for i in 45..50 {
        q[i] = 0.5;
        ve[i] = -0.5;
    }

    // ⚠️ velocities and momenta must be < 1.0
    let mut p = zip(&q, ve).map(|(&q, ve)| (q * ve)).collect::<Vec<_>>();

    let ticks = 100;
    let output_every = 1;
    let dt = 0.1;

    let ticks = (ticks as f32 / dt) as usize;
    for t in 0..ticks {
        let time = (t as f32) * dt;
        let want_output = t % output_every as usize == 0;
        if want_output {
            zip(&q, &p).enumerate().for_each(|(x, (q, v))| println!("{x} {time} {q} {v}"));
            println!();
        }

        f(&mut q, &mut p, dt);
    }
}

fn borewave2(h: &mut [f32], p: &mut [f32], dt: f32) {
    let n = h.len();
    let mut delta_h = vec![0.0; n];
    let mut delta_p = vec![0.0; n];

    for i in 0..n {
        debug_assert!(h[i] >= 0.0);
        debug_assert!(p[i].abs() <= 1.0);
        //debug_assert!(p[i].abs() <= h[i].abs());

        // water flows into cell index `sink` (left or right neighbor)
        // cannot flow out of bounds
        let sink = if p[i] > 0.0 && i < (n - 1) {
            i + 1 // right
        } else if p[i] < 0.0 && i > 0 {
            i - 1 //left
        } else {
            //delta_p[i] -= p[i] * 0.1 * dt; // dampen out impulse at boundary
            continue;
        };

        // transfer mass
        let dh = (p[i].abs() * dt).clamp(0.0, h[i]);
        delta_h[i] -= dh;
        delta_h[sink] += dh;

        // transfer momentum
        let dp = if h[i] != 0.0 { p[i].signum() * p[i] * p[i] * dt / h[i] } else { 0.0 };
        delta_p[i] -= dp;
        delta_p[sink] += dp;
    }

    for i in 0..n {
        h[i] = (h[i] + delta_h[i]);
        p[i] = (p[i] + delta_p[i]);
    }
}

fn borewave(h: &mut [f32], v: &mut [f32], dt: f32) {
    let n = h.len();
    let mut delta_h = vec![0.0; n];
    let mut delta_p = vec![0.0; n];

    // propagate velocity
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

        let sink = if v[i] > 0.0 && i < (n - 1) {
            i + 1 // right
        } else if v[i] < 0.0 && i > 0 {
            i - 1 //left
        } else {
            //v2[i] = 0.0; // stop velocity at boundary
            continue;
        };

        // amount of water that flows into neighbor
        // cannot be more than all water in cell.
        let bucket = (dt * h[i] * v[i].abs()).clamp(0.0, h[i]);

        // propagate matter
        delta_h[i] -= bucket;
        delta_h[sink] += bucket;

        // propagate momentum
        // source cell has already lost momentum: velocity stays constant while mass has moved out

        // add momentum to sink cell.
        delta_p[sink] += bucket * v[i];
    }

    for ((h, v), (delta_h, delta_p)) in zip(zip(h, v), zip(delta_h, delta_p)) {
        let new_momentum = (*h) * (*v) + delta_p;

        *h += delta_h;
        *v = if (*h) != 0.0 { new_momentum / (*h) } else { 0.0 };
    }
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
