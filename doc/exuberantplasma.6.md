% exuberantplasma(6) | XScreenSaver manual

NAME
====

**exuberantplasma** — classic GLSL screensaver

SYNOPSIS
========

| **exuberantplasma** \[**-h**|**--help**] \[**--window-id** _id_] \[**--root**]
| \[**--wireframe**] \[**--fps**]

DESCRIPTION
===========

This is a "hack" for the XScreensaver screen locker. It shows a wavey color
background.

OPTIONS
=========

_exuberantplasma_ accepts the following options. Note that some long options (like
**root** and **window-id** can be passed with either a single dash (**-root**)
or a double-dash (**--root**) for backwards compatibility with XScreensaver.

-h, --help

:   Prints brief usage information.

--wireframe

:   Render in wireframe instead of solid. (UNIMPLEMENTED)

--fps

:   Display the current frame rate, CPU load, and polygon count. (UNIMPLEMENTED)


FILES
=====

*~/.xscreensaver*

:   Per-user configuration file. If there isn't a line for this hack in the
    file, xscreensaver-demo won't find or run this hack.

*/usr/share/xscreensaver/config/exuberantplasma.xml*

:   Configuration options for this hack

ENVIRONMENT
===========

**XSCREENSAVER_WINDOW**

:   Optional ID number of the X window to draw into.

Note that **XENVIRONMENT** and **DISPLAY** are *not* implemented.

BUGS
====

A lot of features (like fps, root-window-finding, wireframe, etc) aren't
implemented yet.

See GitHub Issues: <https://github.com/bnewbold/exuberant-bovines/issues>

AUTHOR
======

Bryan Newbold <bnewbold@robocracy.org>

SEE ALSO
========

**xscreensaver(1)**, **xscreensaver-demo(1)**
