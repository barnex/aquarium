use crate::prelude::*;

pub fn inception() -> G {
    let size = 256;
    let mut g = G::new(vec2(size, size));
    g.camera_pos = (vec2(size, size) / 2).as_i16().pos();

	noise(&mut g.tilemap, 456, 0.08, Tile::Snow);
	noise(&mut g.tilemap, 456, 0.03, Tile::Block);
	noise(&mut g.tilemap, 789, 0.0, Tile::Farmland);
	noise(&mut g.tilemap, 123, 0.1, Tile::Water);

    g
}

fn noise(tiles: &mut Tilemap, seed: u64, fill: f32, tile: Tile) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let n = 30;
    let scale = 5.0..22.0;
    let params = (0..n)
        .map(|_| {
            let dir = vec2f(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalized() / rng.gen_range(scale.clone());
            let phase = rng.gen_range(-PI..PI);
            (dir, phase)
        })
        .collect_vec();

	
        let (w, h) = tiles.size().into();
        for (x,y) in cross(0..w, 0..h){

			let mut v = 0.0;

			for (dir, phase) in params.iter().copied(){
				let r = vec2(x,y).as_f32();
				let arg = r.dot(dir) + phase;
				v += f32::sin(arg);
			}
			v /= f32::sqrt(n.as_());

			if v > 1.0 - fill{
				tiles.set(vec2(x,y).as_i16(), tile);
			}

		}
		

}
