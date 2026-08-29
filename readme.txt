Sol
===
Sol is a very experimental system to programmatically
define phrases in south indian percussion.

I was originally planning to keep this closed source 
with binaries available for use, but I figured that 
it would be more beneficial given that a lot of 
carnatic/ICM tools are proprietary.

Syntax is largely based off my own extended 
notation system, derived from the system used by 
one of my teachers. This results in a concise 
numbering of pieces.

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

Notes
======
- I will replace the handwritten parser with something 
  more robust in the future
- For now, gaps are syntactical sugar that are not 
  transformed at the time of AST generation. This will 
  definitely change.
- A LLM was used to assist in code refactorization. Any 
  implemented logic was of my own design.
