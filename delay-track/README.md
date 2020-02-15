# What should this do?
ping a given host using the system's ping utility and plots in a live plot using the pure Rust `plotters` library.

# Running it



# Dependencies

Windows: should work out of the box (yay!)

Ubuntu / Linux: `piston_window` requires `libfreetype6-dev` `libfontconfig-dev`  otherwise you'll get an empty window or 
```
...
Fontconfig error: "/etc/fonts/conf.d/90-synthetic.conf", line 5: invalid attribute 'translate'
Fontconfig error: "/etc/fonts/conf.d/90-synthetic.conf", line 5: invalid attribute 'selector'
Fontconfig error: "/etc/fonts/conf.d/90-synthetic.conf", line 6: invalid attribute 'xmlns:its'
Fontconfig error: "/etc/fonts/conf.d/90-synthetic.conf", line 6: invalid attribute 'version'
Fontconfig error: Cannot load default config file
```
