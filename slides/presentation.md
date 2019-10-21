---
theme: metropolis
title: CMDR - interactive command line interfaces in Rust
author: Mendelt Siebenga
toc: false
slide_level: 2
header-includes: \metroset{progressbar=frametitle,sectionpage=progressbar}
---

## CMDR
Interactive command line interfaces in rust

## About me
- Recovering Python programmer
- Programs Scala for money
- Plays with Rust for fun, embedded hardware and synthesizers the rest of the time

## This presentation
- Part 1: Show off CMDR
- Part 2: Show some inner workings
  - Procedural macros

## Some Python (Cmd)

```python
import cmd

class HelloWorld(cmd.Cmd):
    """Simple command processor example."""
    
    def do_greet(self, line):
        print "hello"
    
    def do_EOF(self, line):
        return True

if __name__ == '__main__':
    HelloWorld().cmdloop()
```

## Rust problems
- No class based inheritance
  - But we do have traits
- No introspection/reflection
  - But we do have macros

## Stop! Demo time
- Simple commands
- Docstring help
- Prompt
- Before command hook

## More functions
- Subcontexts
- More hooks
- Command history

## Planned functionality before 1.0
- More examples
- Tab completion
  - Commands
  - Parameters
- Typed argument parsing 

## Some usage suggestions
- Login shell for applicances
- Management application for
- API demo
- Old school dungeon crawler (example coming up)

## More information 
- crates.io
- docs.rs
- Examples: github.com/mendelt/cmdr
- cargo gen template: github.com/mendelt/cmdr-template
- This presentation: github.com/mendelt/cmdr-presentation

## Macro's
Three types
- derive
-
- procedural macros

## Procedural macro code example

## When to use procedural macros
- Introspection/reflection, rust does not have this but macros are way cooler
- Domain Specific Languages or code generation

