## expected behavior

### Currently implemented
[x] `splice <name> <filename>`  -> print region(s) between `spliceSTART <name>`
    and `spliceSTOP <name>`, aka the intron(s)
[x] `splice -e <name> <filename>`  -> print region(s) outside of
    `spliceSTART <name>` and `spliceSTOP <name>`, aka the exons(s)
    [x] `--exons`
[x] `splice -i <text> <name> <filename>`  -> prints file, but with `<text>`
    inserted for intron
    [x] `--insert`
    [x] cannot be combined with -e flag
    [ ] can I reprogram this into eliminating the need for `<name>` but taking
        two arguments?
    - if found multiple times -> replace all (not official behavior,
        but the way it is currently implemented)
[x] `splice -m <name> <filename>`  -> do splicing in-place (mutate file)
    [x] `--mutate`
    [] cannot be combined with using STDIN
[ ] writing tests
    [ ] failures
        [x] fails for bad flag combination/config
        [ ] correct exit codes
    [x] correct inserts
    [x] correct intron prints
    [x] correct exon prints

### Roadmap
#### High priority
[x] graceful error handling
    [] -q flag
    [x] using exit codes
[] `cat <filename> | splice <text>`  -> prints file, but with `<text>`
    inserted for intron
    [] cannot be combined with -i/-e flag
    [] fails if the intron is not unique
    [] `--substitute`
[] `splice <filename> <name> -c <control>`  -> selects introns only when
    somewhere inside the intron `<control>` is matched
    [] `--control`
#### Medium priority
[] get introns from multiple files, new usage options:
    [] `splice <name> <filename> ...`
        - this could work with the m flag, but needs a bit of rewrite
[] `splice <filename> -s <start>:<stop>`  -> splice between line that matches
    `<start>` and line that matches `<stop>`
    [] compatible with -e flag
    [] can be used multiple times
    [] `--selector`
[] regex support
#### Low priority
[] read a config file that associates file extensions with standard matching
    sequences, e.g. `/* -------- spliceSTART <name> -------- */` for .c and
    .h files
[] allow for multiple introns
[] allow for multiple introns with multiple susbstitutions
[] does clap respect the `--` and `-` conventions?
