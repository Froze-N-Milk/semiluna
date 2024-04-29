# Semiluna: A script system configured in Lua, and backed by Rust

ideas rq:

Lua script builds lists of Plugins, and Scripts

Scripts:
- Unique events that only run once in the project build lifecycle
- can be called from the command line, or only exposed internally
- have a "run" or "exec" function
- have a list of other Scripts that need to successfully execute first, this can be modified externally
-

Plugins:
- extensibles
- must expose a basic set of functions that are called by the rust runtime
- define Scripts and functions for other Scripts and functions to use
- configurable, configuration options make it easy to configure supplied Scripts, and which are supplied

PackageManager:
- an inbuilt pm for plugins ?


Rust executable runtime

runs lua buildscript to determine what can be done, then takes user input and runs task via Lua Scripts

stores information about what needs to be done?

has some globals and functions that are exposed that help plugins and scripts with making decisions, and performing file IO ops efficiently
