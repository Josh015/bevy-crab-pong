# Bevy Crab Pong

A Bevy Engine port of the [Video Tutorials Rock](http://www.videotutorialsrock.com/index.php) final project [Crab Pong](http://www.videotutorialsrock.com/opengl_tutorial/crab_pong/home.php).

## To Run

To compile and run, use [cargo](https://www.rust-lang.org/learn/get-started):

```shell
cargo run --release
```

## TODO

- [x] Set up a basic Bevy engine project.
- [x] White box the scene with tinted quads & cubes.
- [ ] Port the core game logic to the white box.
- [ ] Add config files for all game settings.
- [ ] Add 3D text instructions and scores (eg. [Bevy Text Mesh](https://github.com/blaind/bevy_text_mesh)).
- [ ] After a ball passes a goal fade it out and reset it while decrementing that goal's score.
- [ ] Fade out crabs, remove them, and add barrier on their side when their score reaches zero.
- [ ] Handle Game Over state, and resetting game.
- [ ] Add a Ferris model (eg. [Ferris the Crab](https://cults3d.com/en/3d-model/art/ferris-the-crab))
- [ ] Adjust model textures to support color tinting.
- [ ] Add "reflections" in the water via mirrored and blended geometry.
- [ ] Add water texture scrolling.
- [ ] Add camera swaying.
- [ ] Use updated textures rather than the originals?
- [ ] Use a Rust gear logo texture?
- [ ] Add shadowed directional light for the sun?
