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
- [x] Add repeating slow swaying motion on camera.
- [x] Add config files for all game settings.
- [x] Add 2D UI elements for now.
- [x] Get two balls to launch in random directions in succession.
- [ ] Get ball return working.
- [ ] Get bounded crab walking working.
- [ ] Get balls bouncing around inside 4 barriers and poles without crabs.
- [ ] Get balls bouncing off each other.
- [ ] Get player crab control and bouncing working.
- [ ] Get score decrementing on goal working.
- [ ] Get game over state working.
- [ ] Get AI crabs working.
- [ ] Get win condition working.
- [ ] Switch to Bevy's entity IDs instead of custom solution?
- [ ] After a ball passes a goal fade it out and reset it while decrementing that goal's score.
- [ ] Fade out crabs, remove them, and add barrier on their side when their score reaches zero.
- [ ] Handle Game Over state, and resetting game.
- [ ] Add a Ferris model (eg. [Ferris the Crab](https://cults3d.com/en/3d-model/art/ferris-the-crab))
- [ ] Adjust model textures to support color tinting.
- [ ] Add "reflections" in the water via mirrored geometry and water blending.
- [ ] Add water texture scrolling.
- [ ] Use updated textures rather than the originals?
- [ ] Use a Rust gear logo texture?
- [ ] Add shadowed directional light for the sun?
- [ ] Add 3D text instructions and scores? (eg. [Bevy Text Mesh](https://github.com/blaind/bevy_text_mesh))
