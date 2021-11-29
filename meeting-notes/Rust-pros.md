Rust vs. Go:

Decided to use Rust because:
  - _very_ fast = if you code it really well, you can even reach almost full potential of the hardware you are running on
  - many more features available
  - extensive support system with guides, videos, users...
  - you can build concurrency, though manually
  - ensures high safety, the compiler pings every possible minute bug (which is what we are working on here)
    * Rust gives us the ability to say that we own a specific piece of data; it's not possible for anything else to claim ownership, so we know nothing else will be able to modify it.
  - efficient code = you don't write much to do much
  - Rust is more suitable for research, and Go is a better go-to for companies and bigger teams
  - Go's garbage collection is questionable, and we don't need more unclarity
  - might as well learn to code well
  - anyways, people say they hugely learned Go in two days, so if something goes very wrong, we can switch in no time