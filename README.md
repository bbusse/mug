# mug
Dynamic tray icons: Fill your mug with life

Create instant functional tray icons -  
Pick a png and an action, ready 🚀

# Usage

```
# Show PNG icon (monochrome by default)
mug --on-left-click "osascript -e 'display notification \"Full steam ahead\" with title \"mug\"'"

# Show PNG icon in color
mug --use-colour --on-left-click "afplay /System/Library/Sounds/Ping.aiff"

# Show emoji or text as tray icon (always in color)
mug --text "🚀" --on-left-click "afplay /System/Library/Sounds/Ping.aiff"

# Clone repo
git clone https://github.com/bbusse/mug
```
# Options

- `--on-left-click <command>`: Run a shell command when the tray icon is left-clicked
- `--use-colour`: Show the PNG icon in color (default is monochrome)
- `--text <emoji or string>`: Show an emoji or text as the tray icon instead of a PNG

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
https://github.com/twitter/twemoji)  
