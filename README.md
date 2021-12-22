TAP-research
------------

Questions of the week:

- should I motivate not hiding c1 from the trigger? in the design section?
- do we write anything about semi-honest security in the background or design, or do we not talk about it explicitly?
- crypto book, page 32-33, all of MPC protocols discussed build on oblivious transfer, what do they mean by it?
- do we use purely Yao's GC or sth else?
- page 33, "whille not having the best known communication complexity" - what does it mean, do we want the communication to be complex?
- same place refers to other protocols that get more complicated as the communication rounds increase, so why are Yao's circuits different? Do they simply not offer such complexity and that's it?

------
IFTTT:
- Wifi password from google assistant https://ifttt.com/applets/EQ6xwrTD-wifi-password?term=password
- For new account notifications sent to your email. Toodledo will store basic information. Then later add login and password to Toodledo, or your password manager https://ifttt.com/applets/HnDGSTkE-new-accounts?term=password
- Create a customizable form to collect details from paid members into Google Sheets. On a successful enable, a shareable URL to a custom form will be sent to your IFTTT email address. Only members of your channel can use the custom form. If enabled, an email will be sent when a member submits the form to you. https://ifttt.com/connections/DsztxHNf-collect-paid-member-details?term=address
- If someone calls, start maps and show his location, https://ifttt.com/applets/hJMKghVa-if-someone-calls-start-maps-and-show-his-location?term=location
---------

Background section plan
- TAP
- talk about different TAPs, like IFTTT, Zappier, etc.
- write about MPC and garbled circuits
----------

Notes to Veronika (myself):
- fix bibtex for two articles I am planning to add and make sure that citation key is different for them
- https://ifttt.com/plans. --- cite it for ifttt services
- oaf tokens and API

----------
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
