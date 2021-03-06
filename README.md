
<pre>
                 _                          _     _                _        _ 
  _____  ___   _| |__   ___ _ __ __ _ _ __ | |_  | |__   __ _  ___| | _____| |
 / _ \ \/ / | | | '_ \ / _ \ '__/ _` | '_ \| __| | '_ \ / _` |/ __| |/ / __| |
|  __/>  <| |_| | |_) |  __/ | | (_| | | | | |_  | | | | (_| | (__|   <\__ \_|
 \___/_/\_\\__,_|_.__/ \___|_|  \__,_|_| |_|\__| |_| |_|\__,_|\___|_|\_\___(_) </pre>

### Build Dependencies and Installation

It should be possible to build this project for most platforms using cargo
directly. Installation and XScreensaver integration probably only works on UNIX
machines (Linux, \*BSD, etc). It's only been developed and tested on Debian
jessie, using the stable Rust toolchain (version 1.9).

A patched version of the `glutin` OpenGL window generation library is required;
see below.

The `pandoc` tool is required for building manpages.

To just build (`--debug`) and run an indivual "hack", eg `exuberantbovines`:

    cargo run --bin exuberantbovines

To build everything (`--release`) and install, first run:

    make install

Then follow the directions about adding lines like the following to you
`~/.xscreensaver` to have hacks actually show up in, eg, `xscreensaver-demo`:

    GL:               exuberantbovines --root             \n\

### Creating Your Own Hacks

You'll need to create at least three files with the same base name (`$HACK`):

- the rust sourcecode (`src/bin/$HACK.rs`)
- an XML config file (`configs/$HACK.xml`)
- a manpage in Markdown format (`doc/$HACK.6.md`)

Then just add your HACK to the list in Makefile.

See also XScreensaver's 
["Writing new XScreenSaver modules"](https://github.com/Zygo/xscreensaver/blob/master2/README.hacking)

