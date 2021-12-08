TAP-research
------------

Questions of the week:
- page 7, Yao introduce garbled circuits as the first MPC and it's still the basis, so they must be pretty good? I know we talked about them quite a bit, so are we going to use them eventually?
- page 9, do we need to do function verification?
- splitting the key server into two? do we need this?
- secure chananels between parties? API for action and trigger, but how are they going to access us? API as well?
- page 16, computational and statistical security parameters k and sigma, is it for the future security proofs? our k and our sigma?
- page 16-17, a couple of types of security, are they relevant for us, if so which one? (computational, information-theoretic, statistical)
- use secret sharing?
- do we have to create ideal scheme and then see what's not ideal by the real-ideal paradigm?
- semi-honest security?
- don't really understand security with abort, but if it's irrelevant, you don't have to explain
- composition = protocols in conjunction with each other? page 26
- other details like commitment and zero-knowledge - are they helpful for us here?

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
- what else? 
----------

Notes to Veronika (myself):
- fix bibtex for two articles I am planning to add and make sure that citation key is different for them
- https://ifttt.com/plans. --- cite it for ifttt services
- write about MPC and garbled circuits

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
