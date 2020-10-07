# QuickSnippet

Just a quick Rust snippet for compiling a
binary you can use to convert JSON to Hack (HHVM) shapes.
I use it for very specific work if you find it useful thats dope.

Run the code and print out output

```bash
cargo run jsontoshape ./testing.json
```

Run the code with a filepath to create a resulting file

```bash
cargo run jsontoshape ./testing.json ./result.php
```

currently the command jsontoshape is supported
but shapetojson, objtojson and objtoshape is in the works.

## Todo List

- [x] jsontoshape
- [x] shapetojson
- [] objtojson
- [] objtoshape
