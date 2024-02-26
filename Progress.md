# Multithreaded rewrite progress

Pretty difficult, high risk of messy code. 

There is only one real hurdle, getting stdout and stderr printing to be synchronized and backwards compatible.

This is non-trivial when internals can run threaded, eagerly printing cannot happen, messages have
to be passed back through buffers. Even that would be non-trivial if there wasn't color-term writing happening
making things complicated.

The change became pretty big, even though the only substantive change 
is in `print.rs` and `bin/main.rs`, because the printer has to be propagated 
deep into the library, about half of the changes are adding a function argument, and reformatting 
caused by those lines becoming too long.

## Changes

### Output facilities

In practice there are four printing facilities used:

1. Regular `stdout`, pretty easy, replace `println` with printing into a buffer.
2. Regular `stderr`, same as above in all ways that matter.
3. Term stdout, this happens in the `diff`-printing part of the code.
4. Term stderr, this is done by `rustc_error` and the most complex to integrate.

Additionally, these four facilities can't be separated, since they have to preserve order between each other.

### Rename StdoutEmitter

Confusing naming, it doesn't output into `stdout`, but into the 
provided buffer.

## Pros

This change brings a substantial speedup, especially for large projects, the speedup
becoming less and less but not none or negative for smaller projects.  

If instant measures as the average reaction time of 250ms, this brings 
down tokio from the not-instant 271ms, to the instant 86ms. 

Giving space for projects almost 3x tokio's size to have an "instant" formatting experience.

## Drawbacks

There are some drawbacks to consider

### Late printing memory increase

All messages that would have gone into `stdout` or `stderr` now go
into an in-mem buffer. This causes a memory-overhead which scales with project size
(number of files). For a large project, this is a difference of
`117M` on the master version, and `170M` on this version.

### Rustc termcolor vendoring

A problem is the way that `rustc` vendors termcolor. It vendors some types but no printing facilities,
and the vendored code isn't interoperable with the published `termcolor` library, which means that
either the printing facilities need to be reinvented (no), or some compatibility-code needs to be introduced (yes).
Hopefully there's some better solution to this, but it's liveable at the moment.

### Accidentally messing with outputs will cause an output jumble

This change would make any direct interaction with `stdout` and `stderr` in the 
formatting codepath a bug. Generally, printing deep inside a library is considered bad for 
imo, but now that would be upgraded to a bug.
