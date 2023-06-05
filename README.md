Demo of nondeterministic Rust code generation by [`grmtools`](https://github.com/softdevteam/grmtools)

```
cargo test
   Compiling grmtools-nondeterministic-bug v0.1.0 (/home/franklinchen/grmtools-nondeterministic-bug)
    Finished test [unoptimized + debuginfo] target(s) in 0.48s
     Running unittests src/lib.rs (target/debug/deps/grmtools_nondeterministic_bug-18011b81bc01a5ed)

running 1 test
test tests::err_no_number ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests grmtools-nondeterministic-bug

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo test
   Compiling grmtools-nondeterministic-bug v0.1.0 (/home/franklinchen/grmtools-nondeterministic-bug)
    Finished test [unoptimized + debuginfo] target(s) in 1.17s
     Running unittests src/lib.rs (target/debug/deps/grmtools_nondeterministic_bug-18011b81bc01a5ed)

running 1 test
stored new snapshot /home/franklinchen/grmtools-nondeterministic-bug/src/snapshots/grmtools_nondeterministic_bug__tests__err_no_number.snap.new
test tests::err_no_number ... FAILED

failures:

---- tests::err_no_number stdout ----
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Snapshot file: src/snapshots/grmtools_nondeterministic_bug__tests__err_no_number.snap
Snapshot: err_no_number
Source: src/lib.rs:21
────────────────────────────────────────────────────────────────────────────────────────────────────
Expression: term_y::parse(&lexer)
────────────────────────────────────────────────────────────────────────────────────────────────────
-old snapshot
+new results
────────────┬───────────────────────────────────────────────────────────────────────────────────────
    4     4 │                 DefaultLexeme {
    5     5 │                     start: 2,
    6     6 │                     len: 0,
    7     7 │                     faulty: true,
    8       │-                    tok_id: 3,
          8 │+                    tok_id: 0,
    9     9 │                 },
   10    10 │             ),
   11    11 │         ),
   12    12 │     ),
┈┈┈┈┈┈┈┈┈┈┈┈┼┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
   25    25 │                 repairs: [
   26    26 │                     [
   27    27 │                         Insert(
   28    28 │                             TIdx(
         29 │+                                0,
         30 │+                            ),
         31 │+                        ),
         32 │+                    ],
         33 │+                    [
         34 │+                        Insert(
         35 │+                            TIdx(
   29    36 │                                 3,
   30    37 │                             ),
   31    38 │                         ),
   32    39 │                     ],
   33       │-                    [
   34       │-                        Insert(
   35       │-                            TIdx(
   36       │-                                0,
   37       │-                            ),
   38       │-                        ),
   39       │-                    ],
   40    40 │                 ],
   41    41 │             },
   42    42 │         ),
   43    43 │     ],
────────────┴───────────────────────────────────────────────────────────────────────────────────────
To update snapshots run `cargo insta review`
Stopped on the first failure. Run `cargo insta test` to run all snapshots.
thread 'tests::err_no_number' panicked at 'snapshot assertion for 'err_no_number' failed in line 21', /home/franklinchen/.cargo/registry/src/index.crates.io-6f17d22bba15001f/insta-1.29.0/src/runtime.rs:569:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::err_no_number

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

error: test failed, to rerun pass `--lib`
```
