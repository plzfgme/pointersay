# pointersay

Display text in a floating window under your mouse pointer. Supports X11 and Wayland.

## Examples

Basic usage:

```sh
# Display "Hello, world!" under your mouse pointer
echo "Hello, world\!" | pointersay
```

A simple translation tool with [wl-clipboard](https://github.com/bugaevc/wl-clipboard) and [translate-shell](https://github.com/soimort/translate-shell): 

```sh
trans -brief "$(wl-paste)" | pointersay
```

