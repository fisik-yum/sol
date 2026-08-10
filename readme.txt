Sol
===
Sol is a very experimental system to programmatically
define phrases in south indian percussion.

Syntax is largely based off my own extended 
notation system, derived from the system used by 
one of my teachers. This results in a concise 
numbering of pieces, which can then be modified 
through some code if someone dares to.

At the moment, it is largely unoptimized and lacks a 
REPL, which I intend to add. I have also thought 
about adding exploration routines, possibly via a 
WASM plugin system to enable assisted korvai-search.

A particular challenge of notation is the arbitrary
timescale transitions. It is for this reason that I 
implemented this as a declarative system. I suppose 
we cannot correctly describe everything concisely. 
This will probably be solved with a custom dimensional 
analysis library.
