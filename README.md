TAP-research
------------

Questions of the week:
- security proofs - do we need to do them??? 
- do we need to generate aad? 
- do we need to drop bytes of keys after we've transformed them into key types in Kem? or maybe also clear files once we've retrieved the keys? or other ways to make sure the keys are secure?


- Development of clear performance criteria

- Sufficient preliminary designs

- Well Constructed and tested prototype

- Statistically analyzed and documented results

- Retesting and redesign
---------
- Zapier can read gmail letters

------

IFTTT:
- Wifi password from google assistant https://ifttt.com/applets/EQ6xwrTD-wifi-password?term=password
- Create a customizable form to collect details from paid members into Google Sheets. On a successful enable, a shareable URL to a custom form will be sent to your IFTTT email address. Only members of your channel can use the custom form. If enabled, an email will be sent when a member submits the form to you. https://ifttt.com/connections/DsztxHNf-collect-paid-member-details?term=address
- If someone calls, start maps and show his location, https://ifttt.com/applets/hJMKghVa-if-someone-calls-start-maps-and-show-his-location?term=location
---------

Background section plan
- TAP
- talk about different TAPs, like IFTTT, Zappier, etc. - market analysis
- write about MPC and garbled circuits
- do we write anything about semi-honest security in the background or design, or do we not talk about it explicitly?
----------

Notes to Veronika (myself):
- oauth tokens and API
maybe google will be comfortable with their emails on IFTTT using this new design
-------
On Abdou et al. Digital automation:
- use decoupled ifttt paper fot 
Earlence Fernandes and Amir Rahmati et al. [17] quantify the risk users face in the context of IFTTT that inter- faces with more than 297 online services and provides over 200,000 recipes. They perform the first empirical analysis of the OAuth-based authorization model of IFTTT using semi- automated tools that they built to overcome the challenges of IFTTT’s closed source nature and of online service 

- In addition, digital automation platforms provide their RESTful APIs for integration with other application. They protect the communication through RESTful APIs using OAuth authorization workflow [23]. In [24] Z. Berkay Celik et al. provided IOTGUARD which detects when an application or interactions among applications lead to unsafe and insecure states and block these undesired states.
- paper 23: E. Fernandes, A. Rahmati, J. Jung, and A. Prakash, “Decentralized action integrity for trigger-action iot platforms,” in Proceedings 2018 Network and Distributed System Security Symposium, 2018.
- paper 24: Z. B. Celik, G. Tan, and P. D. McDaniel, “Iotguard: Dynamic enforce- ment of security and safety policy in commodity iot.” in NDSS, 2019.

- papers that talk about unexpected behaviour with security risks:
 [20] C. Nandi and M. D. Ernst, “Automatic trigger generation for rule- based smart homes,” in Proceedings of the 2016 ACM Workshop on Programming Languages and Analysis for Security, 2016, pp. 97–102.
[21] M. Surbatovich, J. Aljuraidan, L. Bauer, A. Das, and L. Jia, “Some recipes can do more than spoil your appetite: Analyzing the security and privacy risks of ifttt recipes,” in Proceedings of the 26th International Conference on World Wide Web, 2017, pp. 1501–1510.
[22] J.-b. Woo and Y.-k. Lim, “User experience in do-it-yourself-style smart homes,” in Proceedings of the 2015 ACM international joint conference on pervasive and ubiquitous computing, 2015, pp. 779–790.

- Zapier is more secure according to https://www.luhhu.com/blog/is-zapier-safe-for-businesses and https://myintelligenthouse.com/is-it-safe-to-use-ifttt-security-concerns-when-using-ifttt/
----------
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

------
January timeline
Week 1 (of Jan sth)
- write networking code
- mpc library setup
- finish design

Week 2 
- android app design
- design a high-level overview diagram
- think of the sequence diagram design

Week 3
- security proofs
- security analysis

Week 4
- start setting up the experiments
- evaluating metrics: latency, how many requests per min, communication cost
