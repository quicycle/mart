// use std::f64::consts::TAU;

// pub const H: f64 = 6.626_07e-34;
// pub const H_BAR: f64 = H / TAU;
// pub const EP_0: f64 = 8.854_188e-12;
// pub const MU_0: f64 = 1.256_637e-6;
// pub const E: f64 = 1.602_176_6e-19;
use plotters::prelude::*;

const N_STEPS: usize = 1000;
const K_TOTAL: usize = 500;
const K_MID: usize = K_TOTAL / 2;
const SPREAD: f64 = 12.0;
const T0: f64 = 40.0;

trait Boundary {
    fn apply(&mut self, ex: &mut [f64], hy: &mut [f64]);
}

#[derive(Debug, Default)]
struct ReflectingBoundary;
impl Boundary for ReflectingBoundary {
    fn apply(&mut self, _: &mut [f64], _: &mut [f64]) {}
}

#[derive(Debug, Default)]
struct AbsorbingBoundary {
    l1: f64,
    l2: f64,
    h1: f64,
    h2: f64,
}
impl Boundary for AbsorbingBoundary {
    fn apply(&mut self, ex: &mut [f64], _: &mut [f64]) {
        (ex[0], self.l2, self.l1) = (self.l2, self.l1, ex[1]);
        (ex[K_TOTAL - 1], self.h2, self.h1) = (self.h2, self.h1, ex[K_TOTAL - 2]);
    }
}

#[derive(Debug)]
struct Source {
    k: usize,
    f: fn(f64) -> f64,
}

#[derive(Debug)]
struct Simulation<const K: usize, const NS: usize, B: Boundary> {
    ex: [f64; K],
    hy: [f64; K],
    sources: [Source; NS],
    boundary: B,
}

impl<const K: usize, const NS: usize, B: Boundary> Simulation<K, NS, B> {
    #[inline]
    fn update_ex(&mut self, k: usize) {
        self.ex[k] += 0.5 * (self.hy[k - 1] - self.hy[k]);
    }

    #[inline]
    fn update_hy(&mut self, k: usize) {
        self.hy[k] += 0.5 * (self.ex[k] - self.ex[k + 1]);
    }

    fn step(&mut self, t: f64) {
        for k in 1..K_TOTAL {
            self.update_ex(k);
        }

        for s in self.sources.iter() {
            self.ex[s.k] += (s.f)(t);
        }

        self.boundary.apply(&mut self.ex, &mut self.hy);

        for k in 0..(K_TOTAL - 1) {
            self.update_hy(k);
        }
    }
}

fn main() -> anyhow::Result<()> {
    // third arg is frame duration
    let area = BitMapBackend::gif("animated.gif", (640, 480), 4)?.into_drawing_area();
    let mut ctx = ChartBuilder::on(&area).build_cartesian_2d(0..K_TOTAL, -1.0..1.0)?;

    let mut s = Simulation {
        ex: [0.0; K_TOTAL],
        hy: [0.0; K_TOTAL],
        sources: [
            Source {
                k: K_MID,
                f: |t| (-0.5 * ((T0 - t) / SPREAD).powi(2)).exp(),
            },
            Source {
                k: K_MID / 2,
                f: |t| -(-0.5 * ((T0 - t) / SPREAD).powi(2)).exp(),
            },
        ],
        boundary: ReflectingBoundary,
    };

    let mut t: f64 = 0.0;

    for i in 0..N_STEPS {
        t += 1.0;
        s.step(t);

        println!("Step {i}");
        if i % 10 == 0 {
            println!("outputting frame");
            area.fill(&WHITE)?;
            ctx.configure_mesh().draw()?;
            let ex = LineSeries::new((0..K_TOTAL).map(|k| (k, s.ex[k])), &GREEN);
            let hy = LineSeries::new((0..K_TOTAL).map(|k| (k, s.hy[k])), &BLUE);
            ctx.draw_series(ex)?;
            ctx.draw_series(hy)?;
            area.present()?;
        }
    }

    Ok(())
}
