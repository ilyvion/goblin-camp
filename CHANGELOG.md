## 0.1.0
While porting the game, it has been renamed **Goblin Camp Revival**. This was done both to indicate
its relationship to its predecessor, but also to give it a new name so it won't be confused with the
original game. As such, I'm also starting the versioning over at **0.1**. Once I feel the game is
"as good as" the original, I'm going to release it, and each update from then on will be increasing
the version number.

This is the initial release since porting Goblin Camp from C++. While porting the code, I was also
trying to catch and fix any existing bugs, as well as make obvious improvements where possible. I've
done my best to catalog these changes and improvements below.
### New ✨
* None yet? Probably won't be for 0.1, since it's just trying to be on par with the original,
feature-wise.
### Improvements 🙌
* While not outwardly visible to the end user, the architecture of the game is being changed to be a
lot less rigid and use fewer poor coding practices. This effort will hopefully make it easier to
contribute improvements and bug fixes for outsiders, and make it easier to make improvements and
changes to the game going forward. Among these improvements are:
  * Proper game state management. Hard to explain succinctly, but have a look at the `GameState`
  trait and its implementers, as well as how these are used from the `Game` type.
  * Thanks to the game state management, there is now a global game loop, whereas the original
  had "local game loops" all over the place.
  * Furthermore, since the original had local game loops everywhere, each such loop had its own
  input handling, game logic and screen rendering code. Besides causing unnecessary repetition,
  this code also tended to be "spaghetti code" which intertwined input handling, game logic updates
  and screen rendering. Thanks to the global game loop, there is now one place for input handling,
  which gets passed on to each component, and each component receives a separate method call for
  game logic updates and for rendering, so that code separation is maintained.
* Folder and file paths are now adhering to platform standards. This means you probably won't find
the files where you used to in the original. Run the game with a `-v` parameter to have it print out
(among much other debug information) the paths it uses for various purposes.
### Regressions 💩
* Python support has been removed from the game. I realize that this is a backwards incompatible
change, but there is currently no way to run Python scripts from Rust stable (which is where I want
to be). I am considering replacing Python with Lua, since I find it a much more appropriate language
for scripting, but in the meantime, the game is without a scripting system.
### Bug Fixes 🐛
* Attempting to save invalid resolution settings will not work anymore.
Instead of allowing this, and causing a crash on the next startup, the field will be colored in red,
and a dialog will pop up if you try to save, telling you your values are not valid. 
