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

For the most part, the entire engine has been written 
generally to be used as a library. There is a functi-
onal composable pipeline system, but the API is neither
 robust nor well-defined.

This project aims to only provide a REPL and an interpreter 
for composition and archival purposes.

Originally, an LLM was used to refactor and do some 
grunt work; As of 11-12-2026, all LLM-assisted code 
has been removed and replaced with human-written code. 
From now on, all contributions (if any) must be only 
written by a human and not an agent or large language 
model.
