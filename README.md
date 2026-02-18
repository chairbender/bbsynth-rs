# bbsynth-rs
Fun with virtual analog - now in Rust + Faust

Using portlight + coupler.

# Build

```
cargo run -p cargo-coupler -- coupler bundle -p bbsynth-vst3 
cargo run -p cargo-coupler -- coupler bundle -p bbsynth-clap
```

Add `--release` for release build 

# Attribution

# License

All code licensed under GPLv3

(I believe Faust has some GPLv3 code so that's why I have it as GPLv3 for now).
