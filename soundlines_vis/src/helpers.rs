use rand;
use rand::distributions::Range;
use rand::distributions::IndependentSample;

use noise::Perlin;
use noise::NoiseModule;

pub fn random(min: f32, max: f32) -> f32 {
    if min == max {
	    return min;
    }

    let mut rng = rand::thread_rng();
    let range = Range::new(min, max);
    unsafe {
        range.ind_sample(&mut rng)
    }
}

pub fn noise(x: f32, y: f32) -> f32 {
    let perlin = Perlin::new();
    perlin.get([x, y])
}

pub fn map(val: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (val - start1) / (stop1 - start1) * (stop2 - start2) + start2
}
