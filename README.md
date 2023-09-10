# Bevy Crab Pong

A Bevy Engine port of the [Video Tutorials Rock](http://www.videotutorialsrock.com/index.php) final project [Crab Pong](http://www.videotutorialsrock.com/opengl_tutorial/crab_pong/home.php).

## To Run

To compile and run, use [cargo](https://www.rust-lang.org/learn/get-started):

```shell
cargo run --release
```

## Screenshot

![Bevy Crab Pong](screenshots/screenshot.jpg)

## TODO

- [x] Fix issue with mode switching due to paddles not respawning between rounds.
- [ ] Make walls spawn fully-formed before window opens.
- [ ] Add "reflections" in the water via mirrored geometry and water blending.
- [ ] Switch to RT-based in-world UI.
- [ ] Add a Ferris model (eg. [Ferris the Crab](https://sketchfab.com/3d-models/ferris-the-crab-e9bc16e19d1c4880b30d2aa5fd174887))
- [ ] Adjust model textures to support color tinting.
- [ ] Use updated scene textures rather than the originals?
- [ ] Add water texture scrolling.
- [ ] Try to mimic the lighting of the original.
- [ ] Add shadowed directional light for the sun?
- [ ] Use a Rust gear logo texture?
- [ ] Add proper mesh text for in-world UI? (eg. [Bevy Text Mesh](https://github.com/blaind/bevy_text_mesh))
- [ ] Adjust ai speed to make them tougher.
- [ ] Make balls gradually get faster the longer they are in play?
- [ ] Fix lighting on "reflections".
