# Duration Parsers
> 4 different ISO 8601 Duration Parsers

This is a series of examples used for a presentation
on Rust parser libraries.

There are 4 different implementations each can be found
in the `./crates` folder.

The main crate is feature gated to perform the same process
for each of the implementations.

To see each parser in action you could run the following
command.

```
cargo run --features <feature> -- [number]
```

Where `<feature>` is one of the following
- `nom` - for the nom implementation
- `combine` - for the combine implementation
- `pest` - for the pest implementation
- `hand` - for the hand rolled implementation
and `[number]` is the number of random durations you
want to generate (default is 1000).

## Crates
### Nom
The most popular parsing rust crate, utilizes macros
extensively.

[repo](https://github.com/Geal/nom)
[docs](https://docs.rs/nom)

### Combine
A function based approach to parsing, utilizes `impl Trait`
extensively

[repo](https://github.com/Marwes/combine)
[docs](https://docs.rs/combine/)

### Pest
A PEG (parsing expression grammar) parser generator, utilizes
an external grammar file to generate a parser for the user.

[website](https://pest.rs/)
[repo](https://github.com/pest-parser/pest)
[docs](https://docs.rs/pest/)

## Performance
> WARNING, NAIVE BENCHMARKS AHEAD

| crate       | build time | build size | bench                     |
| ----------- | ---------- | ---------- | ------------------------- |
| nom         | 9.67s      | 202kb      | 3,279 ns/iter (+/- 3,321) |
| combine     | 26.96s     | 303kb      | 4,172 ns/iter (+/- 944)   |
| pest        | 1m 13s     | 417kb      | 4,556 ns/iter (+/- 3,461) |
| hand rolled | 3.60s      | 125kb      | 2,099 ns/iter (+/- 121)   |
> MacBook Pro (Retina, 13-inch, Late 2013), Processor: 2.4GHz i5, Memory: 8GB
