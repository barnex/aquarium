use crate::prelude::*;

pub struct State {
    pub inputs: Inputs,

    pub frame: u64,
    pub score: u64,
    pub x: f64,

    pub kits: Vec<(Sprite, vec2i, vec2i)>,
}

impl State {
    pub fn new() -> Self {
        let sprites = [
            sprite!("kit0"),
            sprite!("kit1"),
            sprite!("kit2"),
            sprite!("kit3"),
            sprite!("kit4"),
            sprite!("kit5"),
            sprite!("kit6"),
            sprite!("kit7"),
            sprite!("kit8"),
            sprite!("kit9"),
            //sprite!("blabla"),
        ];

        let N = sprites.len();

        let mut rng = ChaCha8Rng::from_seed([42; 32]);

        let (w, h) = (480, 320);
        let kits = (0..1000).map(|i| (sprites[i % N], vec2i(rng.gen_range(0..w), rng.gen_range(0..h)), vec2i(rng.gen_range(-3..=3), rng.gen_range(1..3)))).collect();

        Self {
            inputs: default(),
            frame: 0,
            x: 0.0,
            score: default(),
            kits,
        }
    }

    pub fn tick(&mut self) {
        let size = [480, 320];
        for (_, pos, vel) in &mut self.kits {
            *pos += *vel;
            for i in 0..2 {
                if pos[i] > size[i] {
                    pos[i] = size[i];
                    vel[i] = -vel[i]
                }
                if pos[i] < 0 {
                    pos[i] = 0;
                    vel[i] = -vel[i]
                }
            }
        }

        if self.inputs.just_pressed(Button(str16!("b"))) {
            self.score += 1
        }

        self.frame += 1;
        self.x += 0.5;
        if self.x > 100.0 {
            self.x = 0.0
        }
    }

    pub fn render(&self, out: &mut Output) {
        //let x = self.x.as_();
        //out.sprites.push((sprite!("kit3"), vec2i(x, x)));

        out.sprites.extend(self.kits.iter().map(|(sprite, pos, _)| (*sprite, *pos)));

        writeln!(&mut out.debug, "frame {}", self.frame).unwrap();
        writeln!(&mut out.debug, "sprites: {}", out.sprites.len()).unwrap();
        writeln!(&mut out.debug, "score {}", self.score).unwrap();
        writeln!(&mut out.debug, "inputs {:?}", self.inputs).unwrap();
    }
}
