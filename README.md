# mug
Dynamic tray icons: Fill your mug with life

Create instant functional tray icons -  
Pick a png or emoji and an action, ready 🚀

# Usage

```
# Show PNG icon (monochrome by default)
mug --on-left-click "osascript -e 'display notification \"Full steam ahead\" with title \"mug\"'"

# Show PNG icon in colour
mug --use-colour --on-left-click "afplay /System/Library/Sounds/Ping.aiff"

# Show emoji or text as tray icon (always in colour)
mug --text "🚀" --on-left-click "afplay /System/Library/Sounds/Ping.aiff"

# Run different commands for left and right click
mug --on-left-click "osascript -e 'display notification \"Left\" with title \"mug\"'" \
    --on-right-click "osascript -e 'display notification \"Right\" with title \"mug\"'"
```
# Options

- `--on-left-click <command>`: Run a shell command when the tray icon is left-clicked
- `--on-right-click <command>`: Run a shell command when the tray icon is right-clicked
- `--use-colour`: Show the PNG icon in colour (default is monochrome)
- `--text <emoji or string>`: Show an emoji or text as the tray icon instead of a PNG (always in colour)
- `--tooltip <text>`: Show a custom tooltip when hovering over the tray icon

# Install
```
$ git clone https://github.com/bbusse/mug
$ make && make install
```
# Launch on system start
```
$ make enable
```
# Attribution

Rocket emoji graphic © 2014–2025 Twitter, Inc.  
Licensed under [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/).  
Source: [Twemoji GitHub Repository](https://github.com/twitter/twemoji)  
This project uses Twemoji graphics (resized to 32×32) under the CC-BY 4.0 license.

# Resources
https://github.com/twitter/twemoji
