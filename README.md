TAP-research
------------

Questions of the week:
- do we cite the dataset separately from the paper about the dataset? (I assume so because those two have different links)
- I don't have access to two of the papers in the bibliography:
   * https://dl.acm.org/doi/10.1145/3131365.3131369
   * https://dl.acm.org/doi/10.1145/2858036.2858556

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


Background section plan
- TAP
- talk about different TAPs, like IFTTT, Zappier, etc.
- what else? 


Notes to Veronika (myself):
- fix bibtex for two articles I am planning to add and make sure that citation key is different for them
- https://ifttt.com/plans. --- cite it for ifttt services

**Preliminary Timeline starting week of 11/18/2021:**

  Week 1 (of Mon Nov 22)
    - Background finish what I can
    - High-level design overview

  Week 2 (of Mon Nov 29)
    - Design details:
       * required crypto protocols
       * protocol flow -> UML sequence diagram

  Week 3 (of Mon Dec 6)
  - Security Analysis
  - Specify privacy and functionality goals
  - Start security proofs

  Week 4 (of Mon Dec 13)
  - Implementation details: software
