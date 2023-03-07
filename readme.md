# Sway virtual display (output) control
In the fantastic wayland compositor [sway](https://swaywm.org/) there's a command for creating virtual / headless outputs, mostly for development purposes: `swaymsg create_output`.
But in combination with a VNC server (for example [wayvnc](https://github.com/any1/wayvnc)), this allows you to essentially have additional monitors, by connecting to the VNC server with an appropiate client (for example on an tablet or laptop).

Sure, you have some sort of delay, but for a second, third or fourth monitor that probably only holds stuff like spotify, discord and co, it's more than enough.

The only "problem" is, that you cannot name the virtual outputs you create (atleast not to my knowledge). This is important, because you likely want to set a certain resolution, scale factor and more for them.
By default, it's just "HEADLESS-{incrementing number}".
I also wanted to have an easy time starting and stopping the vnc servers and "unplugging" the virtual outputs.

---

Here's what I came up with:

A CLI, acting as a wrapper around sway and [wayvnc](https://github.com/any1/wayvnc), that stores its state (to keep track of active outputs and the "next output name", you know the HEADLESS-number stuff) in a json file, located in `/tmp`, while reading a config file with "presets", which contain the output settings, the vnc server port and more.

How to use it:

Define some nice presets in the config under `~/.config/vdctl/config.json`.

Now comes the most important concept: You are able to start ("create") and stop ("kill") your presets, meaning that a preset can only be started once. If you want to have two, just copy / paste it in the config file and give it a different name.

Next you can start a preset with `vdctl create [PRESET]`, the port will be displayed.


By the way: It goes without saying that you should probably secure the vnc server with some sort of authentication. With wayvnc you can do so [using a config file](https://github.com/any1/wayvnc#encryption--authentication).

---

## Commands
General schema: `vdctl [options] [ACTION] [PRESET / VALUE]`

### Action
- `create [PRESET]`: Creates a headless output, sets its settings & starts a vnc server.
- `kill [ACTIVE-PRESET]`: Terminates the vnc server process & unplugs the virtual display. (The workspace will be persisted tho).
- `list`: Prints all the active outputs.
- `next-number [NUMBER]`: manually set the next output number. Basically only good for debugging.
- `sync-number`: If there already any virtual outputs active, this will parse `swaymsg -t get_outputs` and set the next output number accordingly

### Options
- `novnc`: Also for debugging. Skips starting the VNC server, only creates the output with the settings

