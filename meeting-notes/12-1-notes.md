12/1 meeting notes
------------------

if f1(x, c1), then send f2(x,c2) to action service

f1(x,c1) -> "Email contains 'confidential'"
- x is the email, c1 is 'confidential'

f2(x,c2) -> "Text the email to my number"
- x is the email, c2 is the number

Old properties:
- TAP: shouldn't learn x, (c1, c2)
- TS: shouldn't learn (c1, c2)
- AS: shouldn't learn x, (c1, c2)

New properties:
- TAP: shouldn't learn x, (c1, c2)
- TS: shouldn't learn c2
- AS: shouldn't learn x, c1

Costs:
- TAP:
  - Stores a bunch of circuits (for f2) generated by client (rules)
  - Stores for each service, set of f1s or f2s
  - Compute f2(x, c2) using MPC
- TS:
  - Stores all the inputs
  - Given c1, computes f1(x, c1) in plaintext
- AS:
  - Gets the output of f2(x, c2)
  - Participate in the MPC (not that expensive)

What are the metrics we care about?
- Client (your mobile phone): Computation, communication to TAP
- TS: Computation and communication to TAP
   - Computation of f1(x, c1) in plaintext vs generating GC input for x
   - Communication hopefully decreases, because now for f1, just sends
     encryption of a bit
- AS: Decrypt result of f2(x, c2)

TODO:
- When do API calls happen?
- Motivate that c1 doesn't need to be hidden from TS.
- Kevin: Send resources on garbled circuits (lots of them)
- Kevin and Veronika: Look into rules

Types of MPC:
- We'll probably use 2-PC with garbled circuits
- Other: Secret sharing MPC