# Bevy Crab Pong

A Bevy Engine port of the [Video Tutorials Rock](http://www.videotutorialsrock.com/index.php) final project [Crab Pong](http://www.videotutorialsrock.com/opengl_tutorial/crab_pong/home.php).

## To Run

To compile and run, use [cargo](https://www.rust-lang.org/learn/get-started):

```shell
cargo run --release
```

## TODO

- [x] Set up a basic Bevy engine project.
- [x] Add config files for all game settings.
- [x] White box the scene with tinted quads & cubes.
- [x] Add repeating slow swaying motion on camera.
- [x] Add placeholder 2D UI elements.
- [x] Add two balls that launch in random directions in succession.
- [ ] Add ball visibility system to fade them into/from view.
- [ ] Get ball return working.
- [ ] Get bounded crab walking working.
- [ ] Get balls bouncing around inside 4 barriers and poles without crabs.
- [ ] Get balls bouncing off each other.
- [ ] Get score decrementing on goal working.
- [x] Add Game Over state that resets the game.
- [x] Add crab visibility system to grow/shrink them into/from view.
- [ ] Add crabs AI.
- [ ] Get win condition working.
- [ ] Switch to Bevy's entity IDs instead of custom solution?
- [ ] After a ball passes a goal fade it out and reset it while decrementing that goal's score.
- [ ] Fade out crabs, remove them, and add barrier on their side when their score reaches zero.
- [ ] Add new game and game over messages UI.
- [ ] Add a Ferris model (eg. [Ferris the Crab](https://cults3d.com/en/3d-model/art/ferris-the-crab))
- [ ] Adjust model textures to support color tinting.
- [ ] Add "reflections" in the water via mirrored geometry and water blending.
- [ ] Add water texture scrolling.
- [ ] Use updated textures rather than the originals?
- [ ] Use a Rust gear logo texture?
- [ ] Add shadowed directional light for the sun?
- [ ] Add 3D text instructions and scores? (eg. [Bevy Text Mesh](https://github.com/blaind/bevy_text_mesh))
