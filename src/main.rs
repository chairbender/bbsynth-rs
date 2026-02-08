use nih_plug::prelude::*;
use bbsynth_rs::Gain;

fn main() {
    nih_export_standalone::<Gain>();
}