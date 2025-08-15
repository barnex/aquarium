fn main() {
    diffusion();
}

fn diffusion() {
    let l = 50;
    let mut q = vec![0.0; l];

    let ticks = 1000;
    for t in 0..ticks {
        q[0] = 1.0;
        q[l - 1] = 0.0;

        diffuse(&mut q);

        if t % 20 == 0 {
            q.iter().enumerate().for_each(|(i,v)| println!("{i} {v}"));
            println!();
        }
    }
}

fn diffuse(q: &mut [f32]) {
    let l = q.len();
    let mut q2 = vec![0.0; l];

    for i in 1..(l - 1) {
        let delta_l = 0.5 * (q[i - 1] - q[i]);
        let delta_r = 0.5 * (q[i + 1] - q[i]);

        q2[i] = q[i] + delta_l + delta_r;
    }

    q.clone_from_slice(&q2);
}
