# restart-plugin

A skyline plugin for allowing `cargo-skyline` (or other tools) to restart your game without you 
having to touch your controller.

### Install

Copy into `sd:/atmosphere/contents/[title id]/romfs/skyline/plugins`, where `[title id]` is the 
title id of the game you want to enable remote restarting for.

### Usage

* Launch your game with the plugin installed
* Run the following `cargo-skyline` command:

```
cargo skyline run --restart
```

this will build your plugin, FTP it to your switch, and start up logging before restarting your 
game so your changes take effect.

Also available is the following command for restarting without any of the rest of it:

```
cargo skyline restart-game
```

### Technical Details

To restart a game from your own tool, connect to the Switch on port 45423 and send an 8 bytes containing a big endian title id for the game you're expecting to restart.
