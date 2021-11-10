Meeting notes 11/10
-------------------

Quick hits:
  - Will we use symmetric key crypto?
  - Design goals?
  - Functionality goals?
  - What SMC protocols to use? I believe they don't use them in the paper but use modified garbled circuits


Will we use symmetric key crypto?
---------------------------------

YES! Probably.

PKC: Gen, Enc, Dec

Kevin                 Veronika
pk_K, sk_K            pk_V, sk_V

I want to send you a message m, and I know your public key pk_V.

c <- Enc(pk_V, m)

I send you the ciphertext c.

You decrypt:

m <- Dec(sk_V, c)

End-to-end encryption

Fitbit                     TAP                Google Calendar

c <- Enc(pk_F, m)    m <- Dec(sk_F, c)
                     c <- Enc(pk_V, m)        m <- Dec(sk_V, c)

Not E2E

c <- Enc(pk_V, m)    c                        m <- Dec(sk_V, c)

E2E


Design goals?
-------------

1. Privacy - What does exactly mean? To be determined. Ideally, the TAP
   shouldn't learn any information.
2. Usability (client should be offline) - This is a cloud service, so the client
   should be able to be fully offline after setup.
3. Performance - It should go fast. MPC is a heavy, general-purpose tool, but
   can be very, very slow. Can we achieve good privacy without MPC?

Functionality goals?
--------------------

1. Should mimic most of the functionality of IFTTT. What are the most popular
   applets people use today? Are they privacy invasive?
2. Look into IFTTT, Zapier, Microsoft Power Automate, SmartThings?

What SMC protocols to use? I believe they don't use them in the paper but use
modified garbled circuits
-----------------------------------------------------------------------------

1. Both papers use garbled circuits (2-party computation or 2PC).
   In garbled circuits, there are two kinds of security: semi-honest and
   malicious.

  Semi-honest security: We assume that all parties will follow the protocol, but
  they are curious (i.e., they can eavesdrop and passively learn information).

  Malicious security: Parties can arbitrarily deviate, but we should still
  retain the security properties we're after.

  The first TAP paper: We can use a semi-honest secure garbled circuit scheme,
  and still get good privacy.

  Walnut paper also uses 2PC, but it leakes some more information for the sake
  of performance.

Garbler                            Evaluator
-------                            ---------
Has input x                        Has input y

Both want to compute z <- f(x, y), but leak only the output z

We represent this function as a binary circuit C (circuit with AND gates, XOR
gates, etc.)

Create GC     ---------------->     GC

                                    Use oblivious transfer to get its inputs to
                                    the circuit for y, without garbler learning
                                    anything.

                                    Evaluates the garbled circuit on the garbled
                                    inputs, and gets the output z.

Requires a lot of symmetric crypto! Also, requires a lot of communication
between the garbler and evaluator.

We want to reduce computation cost, and also communication cost (ideally without
MPC).

Here's a naive idea for hiding the services
-------------------------------------------

TAP knows my identity.

Services are FitBit and Google.

Today:
FitBit <---------------> TAP <--------------> Google

A better idea?:
Assume TAP1 and TAP2 don't collude.

FitBit <---------------> TAP1 <----> TAP2 <--------------> Google

TAP1 learns you interact with FitBit.
TAP2 learns you interact with Google.
We assume they don't collude, so at least they don't learn you interact with
both FitBit and Google.

A better better idea?:
Maybe there are two proxies?

FitBit <---------------> Proxy1 <----> TAP <----> Proxy2 <--------------> Google

Now TAP maybe doesn't learn that you interact with either FitBit or Google.


Another (bad?) idea for hiding applet semantics
-----------------------------------------------

Example rule: "If email header contains 'IMPORTANT', sound the alarm."

Idea: Encrypt 'IMPORTANT.'

The TAP sees: Example rule: "If email header contains 'ashdfkdhka', sound the
alarm."

Gmail sends encrypted email header, and then we check if encrypted email header
has encryption of "IMPORTANT."

Cryptographic tool: Encrypted search. Possible, but can be leaky.

Pro: Definitely better privacy than we have today!

Con: Only works for a very specific kind of rule.

What we want: a general-purpose solution.

There is one called (cryptographic) program obfuscation. Totally not practical
(but very cool theoretical result)!

Another idea: Change the model. Maybe we don't need to outsource the branch
condition computation.

We ask FitBit to compute "sleep_duration < target_duration" and simply send the
result.

Suppose FitBit sends sleep duration and also number of footsteps.

If I send you 8, what can you infer?

Another idea
------------
Ideally, the client is completely offline after setup. Maybe the client can come
online periodically (very infrequently) to do some useful work that can help
with privacy (this is what the first TAP paper does).

Other things to think about
---------------------------
Heartbeat messages (e.g., for polling services) between service and TAP. Do we
break this functionality?


For crypto coding/systems building
-----------------------------------
- Rust for crypto coding
- Golang for building distributed systems