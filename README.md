# README

## Run

WARNING: must `cargo check` before `trunk serve` lest it serve an outdated build.
```
cargo lcheck --target=wasm32-unknown-unknown && trunk serve
```

or
```
miniserve --port 8001 webshell/dist
```

## Test


 cargo ltest -p gamecore -- --nocapture
 cargo +nightly miri test -p memkeep

### Trunk tweaks

```
trunk config show
watch = ..
```

## Git
```
git push origin HEAD:main
```

## Gameplay

  - [x] God/sandbox mode
  - [ ] Crop dynamics
    - [x] Canal block
    - [ ] Canal dynamics
      - [ ] Water source
      - [x] Water level
      - [ ] Land Irrigation
    - [ ] Farm

## Architecture

### gamecore

- [x] game incl Ui
- [x] tick(inputs) -> outputs
- [ ] outputs:
    - [x] scenegraph
    - [ ] sounds
    - [ ] paths to reflect

- [x] serializeable state like a cpu emulator
- [ ] queryable state / path-based reflection

### webshell

- [x] driver (ticks)
- [ ] inspects via refl
- [x] draws scenegraph



