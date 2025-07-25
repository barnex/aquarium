## Arch

### gamecore
game incl Ui
tick(inputs) -> outputs
outputs:
  * scenegraph
  * sounds
  * paths to reflect
queryable state
path-based reflection
serializeable state
like a cpu emulator

### webshell
driver (ticks)
inspects via refl
draws scenegraph

## Run


WARNING: must `cargo check` before `trunk serve` lest it serve an outdated build.
```
cargo lcheck --target=wasm32-unknown-unknown && trunk serve
```

or
```

miniserve --port 8001 webshell/dist
```

### Trunk tweaks

trunk config show
watch = ..


