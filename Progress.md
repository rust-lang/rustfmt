# Multithreaded rewrite progress

Pretty difficult, high risk of messy code. 

There is only one real hurdle, getting stdout and stderr printing to be synchronized and backwards compatible.

This is non-trivial when internals can run threaded, eagerly printing cannot happen, messages have
to be passed back through buffers. Even that would be non-trivial if there wasn't color-term writing happening
making things complicated.

## Rustc termcolor vendoring

A big problem is the way that `rustc` vendors termcolor. It vendors some types but no printing facilities, 
and the vendored code isn't interoperable with the published `termcolor` library, which means that 
either the printing facilities need to be reinvented (no), or some compatibility-code needs to be introduced (yes).

## Changes

### Output facilities

In practice there are four printing facilities used:

1. Regular `stdout`, pretty easy, replace `println` with printing into a buffer.
2. Regular `stderr`, same as above in all ways that matter.
3. Term stdout, this happens in the `diff`-printing part of the code.
4. Term stderr, this is done by `rustc_error` and the most complex to integrate.

Additionally, these four facilities can't be separated, since they have to preserve order between each other.
