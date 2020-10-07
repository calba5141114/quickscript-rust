# QuickSnippet

Just a quick Rust snippet for compiling a
binary you can use to convert JSON to Hack (HHVM) shapes.
I use it for very specific work if you find it useful thats dope.

## JSON to Shape

Run the code and print out output (this logic applies to other commands)

```bash
cargo run jsontoshape ./testing.json
```

Run the code with a filepath to create a resulting file (this logic applies to other commands)

```bash
cargo run jsontoshape ./testing.json ./result.php
```

## Shape To Json

```bash
cargo run shapetojson ./result.php ./result.json
```

currently the command jsontoshape is supported
but shapetojson, objtojson and objtoshape is in the works.

### Todo List

- [x] jsontoshape
- [x] shapetojson
- [] objtojson
- [] objtoshape
