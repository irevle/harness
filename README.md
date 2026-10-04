# harness

A small JSON-driven test runner. It reads test cases from a file, runs a
port-to-service lookup on each input, times every run, and prints pass or fail.

> [!NOTE]
> This is a project for the purposes of learning, not production tooling.

## Run

```sh
cargo run -- cases.json
```

Output:

```text
http PASS! 0us elapsed.
ssh PASS! 0us elapsed.
whatsthis PASS! 0us elapsed.
```

The harness exits with an error if the file cannot be read or the JSON is
invalid. It reads no other path than the one you pass.

## Case file format

`cases.json` is an array of objects:

```json
[
  { "name": "http", "input": "80", "expected": "http" },
  { "name": "ssh", "input": "22", "expected": "ssh" },
  { "name": "whatsthis", "input": "", "expected": "unknown" }
]
```

- `name` - label printed in the result line
- `input` - text passed to `solve`
- `expected` - text `solve` must return for the case to pass

`solve` maps `80` to `http` and `22` to `ssh`. Any other input returns
`unknown`, so a case with an empty input expects `unknown`.

Add a case, rerun, and the new line appears. No recompile needed.

## Verify it detects failures

Put this case in the file. `solve` has no mapping for `443`, so the case fails.

```json
{ "name": "https", "input": "443", "expected": "https" }
```

```text
FAIL! unknown != https. 0us elapsed.
```

This proves the comparison runs, rather than the harness always printing PASS.

## Layout

```text
harness/
├── cases.json      # test cases
└── src/main.rs     # runner and solve
```

## What this exercises

- `std::env::args` for the file path
- `serde` with the `derive` feature to map JSON keys onto `Case` fields
- `serde_json::from_str` to build a `Vec<Case>`
- `std::time::Instant` to time each run
- `Result` with `?` propagation through `main`

## License

MIT
