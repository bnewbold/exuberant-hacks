
                 _                          _     _                _                 _ 
  _____  ___   _| |__   ___ _ __ __ _ _ __ | |_  | |__   _____   _(_)_ __   ___  ___| |
 / _ \ \/ / | | | '_ \ / _ \ '__/ _` | '_ \| __| | '_ \ / _ \ \ / / | '_ \ / _ \/ __| |
|  __/>  <| |_| | |_) |  __/ | | (_| | | | | |_  | |_) | (_) \ V /| | | | |  __/\__ \_|
 \___/_/\_\\__,_|_.__/ \___|_|  \__,_|_| |_|\__| |_.__/ \___/ \_/ |_|_| |_|\___||___(_)


### Dependencies

Circa June 2016, this requires a patched version of the `glutin` window
creation library to allow re-using an existing X Window. This is only necessary
for integration with X Windows, but the project won't build without it.

As a workaround until there is a solution in upstream `glutin`, use the "dependency override" feature of the cargo build tool:

  http://doc.crates.io/specifying-dependencies.html#overriding-dependencies

Checkout the `feature-existing` branch from
`https://github.com/bnewbold/glutin`, then, under this directory
(exuberant-bovines), create a `.cargo/config` file with a path like:

  paths = ["/home/bnewbold/src/glutin"]

