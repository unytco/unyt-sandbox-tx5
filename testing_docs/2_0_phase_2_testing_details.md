# Phase 2 Testing Details

#### links to related docs

- [Test Plan](./1_0_testing_plan.md)
- [Unyt Setup](../README.md)
- [Testing Documentation, Phase 4](./4_0_phase_4_testing_details.md)
- [Unyt Dictionary](./4_2_unyt-dictionary.md)
- [Intro to Smart Agreements (Three Layers)](./4_1_intro_to_smart_agreements.md)
- [Templates and Smart Agreements Library Repo](https://github.com/unytco/smart_agreement_library)
- [Feedback](https://github.com/orgs/unytco/projects/5/views/1)


Phase 2 Test Details are Below.

**WARNING: This doc contains details!** 

Feel free to use the outline view available in the upper right to get an overview of the different content covered.

See also, [Phase 1 testing details](https://github.com/unytco/domino-releases/blob/develop/testing_docs/1_1_phase_testing_details.md).

## Phase 2 Testing Orientation

### Overview of context:
Unyt is creating an accounting software system that makes flows of service activity visible as well as the payments for that service through credit creation and credit use.

Domino is focused on accounting that recognizes the utilization of services and not just a demonstration of capacity. 

### Version
In Phase 2, we are testing version 0.8.0 of Domino.

A couple of reminders:
1) This version of the software hasn't had its UX/UI simplified for a particular use case or particular end users. 
2) This version is intended to help partners and their developers gain familiarity with the features and functionality of domino, and 
3) to help those teams begin exploring the ways in which they may want to make use of domino for their use cases.

We will be checking out some new features, seeking guidance on functionality and features and will focus on helping Testing Teams create their own custom RAVEs (Smart Contracts).

### Refresher / Intro 
If you are new to Domino or just need a refresher, check out **The Basics** section towards the end of this document. You can click on it in the outline to get there quickly.

For the rest of you, lets dive straight into the latest changes.

## NEW in Phase 2

We have a full list below, but the two big changes since Phase 1 testing are the addition of support for Bulk Processing (Aggregating actions) and the support for multi-unit accounting.

### Summary:
**New Features**
- Bulk Processing
    - Aggregate Sends
    - Aggregate Receives
- Multi-Unit Accounting
    - Base Units
    - Service Units
    - Pool Definition
- Checking Balance(s)
- Passwords (optional)

**Three Methods to Initiate a RAVE from an Executable Agreement**
1. Parked Invoice
2. Parked Promise
3. Executor Initiated

**Examples and Guidance on Creating Custom RAVEs**
 1. [RAVE Library Repo](https://github.com/unytco/rave_library)
 2. RAVE: Aggregate Sends
 3. RAVE: Conditional Forward (whitelist)
 4. RAVE: Conditional Forward (base check)

### Details of New Changes

**Aggregate Sends**
Domino now includes support for bulk (or aggregate) sends. 
This enables a series of payment promises to be prepared using a particular Executable Agreement and Executor (whether by the Spender or from multiple requests (i.e. invoices) from others). 

If a sender sets up multiple payments, they can pay all of them with a single click.

If another party (or parties) have sent multiple invoices to an agent, (requesting payment) these will show up for the Spender as Requests to Spend. In addition to being able to Send one payment at a time, the new Aggregate Sends functionality supports them being able to Send all of them as Parked Promises (links on the Action of that Executable Agreement) with a single button click.

This functionality is useful in contexts such as DePIN where there will be high volumes of transactions maybe with different service providers, but through the same Executable Agreement and we want the user experience to be fairly simple. 

**Aggregate Execution** 
Domino also now supports Aggregate Executions of Parked Promises on an Executable Agreement. Similar to aggregate sends, this enables multiple payments to be processed with a single RAVE, whether from a single spender or multiple spenders, when all are using the same Executable Agreement and Executor. After the RAVE has been Executed, the Executor will notify the Receiver(s) that they have a Payment to Accept. 

This functionality is useful in contexts such as DePIN and can support things like Service Fee collection or Transaction Fee Collection.

**The technical details (for the geeks among us)**
The mechanism by which notifications related to spends and receives work makes use of [holochain links](https://developer.holochain.org/concepts/5_links_anchors/), whether added as 
- metadata (ex: an Invoice, i.e. Request to Spend) on the Spender agentpubkey, 
- metadata (Request to Execute) on the [Action](https://developer.holochain.org/resources/glossary/#action) of the Executable Agreement. 
- metadata (Payment Available for Collection) on the Receiver agentpubkey, or 


For example: when a Spender has been invoiced for one or more payments, they would look at their copy of Domino and see the list of outstanding Requests to Spend (which are being drawn from the links on their own agentpubkey). 

When they pay those, not only is a new Action and a new Entry on their own source chain being created to make that payment, but they are then also marking the relevant Request to Spend links as deleted, indicating (to themselves and others) that they these are no longer outstanding. 

In addition, in the case of a direct transaction, the Spender would add a link to the Receiver's agentpubkey. 
In the case of a RAVE transaction, the spender would instead add a Parked Promise link to the Action of the Executable Agreement.

Note that these links don't affect account balances. They are being used solely for notification purposes.

**Multi-Unit Accounting**
We have added support for multiple accounting units to be supported in any particular Pool. Each particular community that is running a Pool (i.e. their own white-labeled and customized version of the software) would be making use of a single "Base Unit" for making and receiving payments in their application. In addition, they can define different "Service Units" that can be used in the pool as well for tracking activity. This supports tracking of value contributions in one direction and payments in the other direction, and enables both customers and service providers to have some visibility into the value that they have received or the contributions that they have made, in addition to the payments that have been made in one direction or the other.


**Base Unit**
The Base Unit of a pool is the main Unit of Account that the community using that pool is making and receiving payments in. In the current test version of Domino, the base unit is HF, or HoloFuel.

**Service Units**
In addition to the Base Unit of a pool, Domino supports tracking the sending and receipt of other units. Inspired by the design of domino accounting, this enables different communities to decide for themselves the kinds of contributions (useful services) that they expect different members of their community to offer to one another and to determine how those sorts of contributions will be measured and verified. For example, with the Domino, in addition to HoloFuel, we have created at least two service units - one for Bandwidth (measured in Megabytes) and another for Requests (measured as a count).

Participants can send Service Units as part of a RAVE, or in a direct transaction. 

The most common use would be in a RAVE, where the Service Units are documenting the services that have been provided. These are quantities. A canonical case involves Services being provided at rates determined by a Price Sheet. For instance, 1 HF per 10MB of Bandwidth. When a Service Provider goes to Invoice the customer, they could send an Invoice that showed 50 BW (Megabytes of Bandwidth) and 5 HF. Note that Service Units will show up inversed, meaning that the party that provided the services might see their account go from 0 BW to -50 BW (reflecting that they had provided services to others) while the Base Unit would go from 0 HF to 5 HF (reflecting that they had been paid for the services that they had provided).

Though a less common use case, to send Service Units in a Direct Transaction, click on the Unit (which defaults to the base unit, HF) and change it to service unit that you are wanting to transfer. To be clear, this is an accounting only and is actually intended to make visible services that have been provided. For instance, if you had donated 100 megabytes of bandwidth services to a customer, you could make that visible by sending 100 BW to that customer. 


**Pool Definition**
The Pool Admin(s) (a privileged role) creates and edits the pool definition. This involves defining the Symbols, Names and Descriptions of the Base Unit and Service Units that you would want to be available in your accounting system.

The first Pool Piece that gets set up is the Base Unit. 
Any subsequent pieces are Service Units. 
Any of the Units can be edited by the Pool Admin(s). 
Once set, these different Units would likely change only rarely and usually in situations where new types of services are being offered that require a clear accounting.

By default, Domino can support up to 255 different Service Units in addition to the one Base Unit for that pool.


**Checking Balance(s)**
Clicking on your balance in the Status Window in the upper right will bring up your balance for:
The Base Unit (HF, in the current version),
Any Service Units that you have transacted in, and
Any Outstanding Fees Owed.

If you have not sent or received a particular service unit, the current user interface will not display that balance.

An upcoming version will have Fees Automatically Paid during the transaction immediately following a transaction when the Fees Owed balance crosses a threshold (ex: 10 HF). Those fees are then able to be collected by the Fee Collector using the Aggregate Receives functionality mentioned above.

**Passwords (optional)**
When you open the app for the first time, you have the option to create a password. There is no centralized control over your use. If you forget your password, we can't help you regain access to your app. Uninstall the app and create a fresh install to start from scratch and continue testing. More details are available on the [README](../README.md).


**Three Methods to Initiate a RAVE from an Executable Agreement**
There are now three methods available for Initiating a RAVE from an Executable Agreement:
1. Parked Promise
2. Parked Invoice
4. Executor Initiated

The first thing to note is that even though the current implementation shows every Executable Agreement as being able to have RAVEs initiated in all three of these ways, different RAVEs will more naturally fit with some of these and not others.

**Parked Promise**
One way to think about this is that similar to doing a Direct Transaction Promise, A Parked Promise RAVE first sets up a Spender with a Request to Spend and enables them to Approve that Request to Spend (i.e. Pay). Once they send payment, that notifies the Executor that they have a Request to Execute.

**Parked Invoice**
A Parked Invoice on the other hand starts one step earlier. It first sets up a Receiver with an ability to Submit an Invoice and Request a Payment from the spender. Often, if that invoice involves service provision through one of the networks or members of the Pool Alliance, it may need to include some proof of service. 

**Executor Initiated**
Whereas both The Parked Promise and Parked Invoice RAVE Initiation Methods lend themselves naturally to payment like functionality, the Executor Initiated Method can have a wide range of functionality. For instance the __System_Credit_Limit_Computation RAVE is one that is typically Executor Initiated. Anytime a Spender is going to spend into the negative, they first Execute a __System_Credit_Limit_Computation RAVE and include the result on their source chain as the Action immediately preceding their attempt to do a Spend Transaction that takes their balance below zero. The current version of that Executable Agreement enables anybody to run it (to compute their own credit limit -- 1000 HF in the current version).

### Creating Custom RAVEs
A major focus of phase 2 testing is on having testers work on creating custom RAVEs that they think might be useful in their particular context. To that end, we have created a RAVE Library as a Github Repository. It has some examples and guidance at present and we would love to have you create new RAVEs, try them out in Domino and add them to the [RAVE Library Repo](https://github.com/unytco/rave_library)

For a summary and guidance, start by looking at the comments at the top of the execution_code.rhai file in any particular RAVE.

There are 5 different example RAVEs to start with in the RAVE Library Repo
 1. [System Credit Limit Computation](https://github.com/unytco/rave_library/tree/main/library/__system_credit_limit_computation)
 2. [System Transaction Fee Collection](https://github.com/unytco/rave_library/tree/main/library/__system_transaction_fee_collection)
 3. [Conditional Forward (base check)](https://github.com/unytco/rave_library/tree/main/library/conditional_forward_base_check)
 4. [Conditional Forward whitelisting](https://github.com/unytco/rave_library/tree/main/library/conditional_forward_whitelisting)
 5. [POS Invoice Manager](https://github.com/unytco/rave_library/tree/main/library/pos_invoice_manager)

And that is a wrap for the major new items for Phase 2 testing. Below are the basics (mostly from Phase 1) in case you are looking for some orientation or a refresher.

## The Basics

### Testing together on one p2p network
Domino is software that runs locally on your computer and connects with other instances to form a peer-to-peer network.

In the future, there will likely be many different networks running their own version of this software for their community.

During Phase 2 testing, however, all testers will be joining a single peer network and will be able to send and receive transactions from anyone else in the test network. 

Once you have opened Domino, you can test out different types of transactions. To start with, in this version, each agent will have a credit limit of 1000 units.

All of you will be using the software as a super user with the ability to not only engage in direct transactions, but to create new RAVEs (smart contract like functionality. details are below.). 

Since this is a shared application, any Code Template, Executable Agreement or RAVE that you create will be visible to other testers.

### Getting Started

After setting up domino and signing on (with or without a password), you may get a notification that it has not yet synced. This should not take long, assuming at least one other user of the app is online that has synced with the progenitor that set up this app version. That progenitor has added some RAVEs to set some initial groundrules (with a system_credit_limit_computation) that we are going to be using. They (and maybe others) have added other RAVEs to the RAVE library as well that we can try out. Once you are synced, you will have a credit limit available and will be able to spend into the negative up to your credit limit.

Your Status window should always be visible as an overlay on the upper right.

The Status Window displays:
- Status of your internet connection, 
- your agent identicon. 
- Your balance in HF (the main unit of account - or base unit - for this instantiation of the app)
- Fees owed

Clicking on your identicon (on the right side) will copy your public key (your address) to the clipboard. You can then share that with others through whatever other medium you have available (email, signal, whatsapp, zoom chat, etc). 

They will enter it on their own copy of Domino when doing things like sending you an invoice (a request for payment) or a promise (a sending of payment) or when choosing you to serve as the executor of a RAVE.

Clicking on your account balance (which should initially read "0 HF") will bring up a window that shows your balance in each of the account units that you have a balance in (positive or negative). You will always see the base unit (HF), a any outstanding fees owed (in the base unit). If you have balances of service units, those will be visible too. 

Note that this version of Domino doesn't yet have push notifications set up. So for the time being, you may need to hit reload in order to see that someone has sent you a transaction to Pay or Execute or Accept.

### Let's test together

Feel free to try out some transactions with some of our team. 

Matt's public key is:

```
uhCAk6n2s2zomaV5ZeWD1w9bYmkrLxT1evo3WWkVvix-UxBk8NaPK
```

Jarod's public key is:
```
uhCAkHkYGpJTFDSpRWEbCIYAMdovZMGFF3ISqrDQuqumzPc98iO3k
```

And feel free to reach out to us through our shared comms channel on Telegram or Signal. 
 
We are more than happy to jump on a video conference call to test alongside you for a bit. 

### Transactions
In Domino, there are two main types of action:
1) Direct Mutual Credit Transaction
2) RAVE (Record of Agreement Verifiably Executed)

### Overview of Transaction Types
**A Direct Mutual Credit Transaction** involves the sending of credits from one agent directly to another agent. They involve, sequentially, changes to the [source chain history](https://developer.holochain.org/concepts/3_source_chain/) of the spender and of the receiver. A selection of other agents [independently validate](https://developer.holochain.org/concepts/7_validation/) each step in this process.

**RAVEs (Records of Agreements Verifiably Executed)** are similar to blockchain based smart contracts, but are designed to make use of Holochain's agent-centric architecture. They involve the verifiable execution of an agreement by an executor. RAVEs record changes to the source chain history of an executor, and may include references to changes to the source chain of a Spender, as well as references to other sources of data. If the output of a RAVE sends credits to a Receiver, then subsequent to the execution of the RAVE, that Receiver will have to accept the payment before they credits are credited to their account. 

A selection of other agents in the network independently validate each step in this process ([see holochain validation](https://developer.holochain.org/concepts/7_validation/)). RAVE transactions allow more complicated logic to be made use of in structuring transactions or in creating software agreements that are executed in a verifiable manner.

### Direct Mutual Credit Transactions
A Process Overview

Direct transactions involve a spender and a receiver and can be initiated in two different ways:

1) A Promise
2) An Invoice

You can find each of these as Transaction Type options when Creating a Transaction under Transactions > Direct Tx

#### 1) Promise
A sending of payment by the Spender to a Receiver.

**Summary**

This form of payment is a multi-step process with validation happening behind the scenes for both the making of the promise and the acceptance of the promise. 

Reminder: In Holochain, individual agents each have their own activity history (source chain) and they alone authorize state changes to their own source chain. Each agent is responsible for checking the validity of the action they are taking, and each action is also independently validated by other devices on the the network. Agents are able to validly spend amounts (including fees) either to zero, or if they have a credit limit, going into the negative up to their credit limit.

**Promise Workflow**

**1) Sending with Promise**

Spender promises (sends) a specific amount to a specific receiver. As soon as the Spender clicks "Create Promise" the funds are immediately debited from their account balance along with any associated transaction fees.

**2) The Between Times (Before Acceptance)**

Spender will see the transaction as a "Completed Transaction" of the type "promise". At this point, Receiver will see the transaction as an "Actionable Transaction" of the type "Promise" and will be shown the details and an option to Accept the payment.

**3) Acceptance**

Receiver clicks accept. The funds are immediately credited to their account.


    
#### 2) Invoice
A request for payment, made by the receiver. 

A receiver can request a payment by sending an invoice. They will indicate the Spender and the Amount Requested.

**Summary**

An invoice transaction involves the same underlying payment workflow as a promise transaction, but enables the Receiver to initialize the request.

**Invoice Workflow**

**1) Invoice Request**

An agent that is seeking to receive funds can send a request to pay in the form of an invoice to the party that they want to pay them. They enter the amount and the public key of the agent that they are requesting payment from. They can also add a note.

**2) The Between Times (Before Invoice is Paid)**

After an invoice is sent, the receiver will see it in their Pending Transactions as an Invoice. The spender will see it in their Actionable Transactions as an Invoice and will have a button to Pay the invoice on the right hand side. (Spender may need to refresh first). At this point, only a request has been made and no funds have been debited from or credited to either account. 
    
**3) Paying an Invoice**

The process that follows from this point forward is identical to the Promise workflow, as the Spender is now making a Promise (though the details had initially been proposed by the receiver).
As soon as the Spender clicks "Pay" the funds are immediately debited from their account balance along with any associated transaction fees. 

**4) The Between Times (Before Acceptance)**

Spender will see the transaction as a "Completed Transaction" of the type "promise". At this point, Receiver will see the transaction as an "Actionable Transaction" of the type "Promise" and will be shown the details and an option to Accept the payment.

**5) Acceptance**

Receiver clicks accept. The funds are immediately credited to their account. Receiver will now see the transaction as a Completed Transaction of the type "Accepts".




### Making a Direct Transaction 
A step-by-step walk through.

The below Direct Transactions will only work if the sender has available credit in their account. For testing with Domino 0.8.0 we have set it up so that, once synced with the network, any new agent starts with a credit limit of 1000. 


#### Send direct transactions via Promise

Who: Any member of your community in good standing with available credit in their account.

* Transaction Type
    * select Promise
* Amount
  * (pick an amount that is less than that agents remaining credit limit)
* Receiver
  * Copy receivers public key (ask that agent for it)
  * Paste receivers public key into Receiver field
* Note
  * Add a note if you want (optional)
* Click create Promise
  * As soon as you click, the software will check that you are not attempting to spend beyond your credit limit. 
  * If that is fine, it immediately deducts the amount from your balance and makes it available to the receiver
  * It also deducts any associated transaction fees from your account.



#### Receive direct transaction via Promise

Who: any member of your community in good standing.

In Agent UI for Receiver, go to Transactions > Actionable Transactions

* Find Promise, check details and click the accept button (on the right)
  * As soon as you click, the software will check that the spender was not attempting to spend beyond their credit limit. 
  * If that is fine, it immediately credits the amount to your account.





#### Request direct transaction via Invoice
Who: Any member of your community in good standing

* Transaction Type
  * select Invoice
* Amount
  * (pick an amount that is less than that agents remaining credit limit)
* Spender
  * Copy Spender’s public key (ask that agent for it, or if in dev mode, click on the receiver’s icon in that agents window to copy it to clipboard)
  * Paste Spenders public key into Spender field
* Note
  * Add a note if you want (optional)
* Click create Invoice
  * As soon as you click, the software will send an Actionable Transaction to the party being invoiced (the Spender).



#### Pay an Invoice
Who: any member of your community in good standing with adequate credit available.
    * After an invoice has been sent to you, go to the Actionable Transactions section of your agent’s UI. 
    * Find the invoice
    * Check the details of the invoice
    * Click button to pay the invoice (Spend?)
      * As soon as you click, the software will check that you are not attempting to spend beyond your credit limit. 
      * If that is fine, it immediately deducts the amount from your balance and makes it available to the receiver
      * It also deducts any associated transaction fees from your account.



#### Receive Payment of an Invoice
Who: Any member of your community in good standing.

* After reloading, the Receiving Agent will find the transaction in the Actionable Transactions section of their Agent UI
* Check the details and click accept
  * As soon as you click, the software will check that the spender was not attempting to spend beyond their credit limit. 
  * If that is fine, it immediately credits the amount to your account.



### RAVEs
#### About RAVEs
RAVEs, or Records of Agreements, Verifiably Executed are similar to Blockchain based Smart Contracts, but are designed to enable automated agreement execution in agent-based Holochain applications.

Like computer programs more generally, a RAVE can be created to automate the execution of a wide range of activities. 

A RAVE is executed by an Executor. Select other agents in the network validate that the execution was done properly and are relied upon by other agents for their assessment of validity. However, any other agent can independently validate that the execution was done properly. 

A RAVE is an Instantiation of an Executable Agreement. An Executable Agreement must conform to the Code Template that it is based on.

A RAVE includes 

1) a set of Inputs - from the source(s) specified in the Executable Agreement and that match the Input Signature of the relevant Code Template. 

2) execution code

3) a set of Outputs, that match the Output Signature of the relevant Code Template.

In addition, the specific agent that Executes the RAVE must satisfy the Executor criteria from the revelant Executable Agreement, whether that:
  a) requires a specific agent to serve as Executor, 
  b) requires the Executor to be one of set of specific agents or
  c) enables any agent to have been selected as Executor.

Much is outlined in the [Three Layers of RAVEs document](https://github.com/unytco/domino-releases/blob/develop/testing_docs/1_2_three_layers_of_raves.md), but here is a bit more detail about how RAVEs work.

Whether they are instantiated by a Spender, a Receiver, an Executor or another party altogether, the portion that we can describe as the RAVE itself is a specific Record of Agreement, Verifiably Executed, meaning it is the Executor's action that we consider the RAVE. That specific RAVE however, will typically include references to other prior actions which can also be checked and should be valid. Those could include 
1) a reference to an Executable Agreement, of which the RAVE was an instantiation;
2) a reference in that Executable Agreement to the Code Template that the Executable Agreement conforms to;
3) a reference to a Parked Promise by a spender. That is an action that the spender took on their own source chain, and it must be valid and be signed by them.
4) a reference to a Parked Invoice from a party requesting to receive a payment. That is similarly an action taken, possibly by the receiver or possibly by another agent in the network, and it must be signed by them and might need to meet certain requirements, such as a proof of service provision.

If a RAVE results in Credits being made available for a Receiver, the Receiver's collection of the credits requires a separate action by the Receiver and happens AFTER the RAVE that makes those credits available to the Receiver has been executed.

That is an odd little quirk, but it is worth noting.


#### Using an Existing Executable Agreement

*UI Bug note: Though there isn't a drop down icon, in the RAVE Templates tab, clicking on the whitespace between icons on an existing Code Template row will show Executable Agreements that have already been created using that code template. These Executable Agreements can be used to instantiate a RAVE.*

#### Create a RAVE using an Executable Agreement

Feel free to initalize a POS Invoice Manager RAVE. 

Under RAVE Library > Code Templates Library, click on the row for pos_invoice_manager to see the Executable Agreement(s) that have been created from that template. Pick one and click Initialize to begin structuring a transaction using the agents and amounts of your choice.

#### Create an Executable Agreement from a Code Template
If you haven't already, you can create you own Executable Agreement from one of the Code Templates. Maybe create an alternate version of the conditional_forward_whitelabeling. Details on how it currently works are [available here](https://github.com/unytco/rave_library/blob/main/library/conditional_forward_whitelisting/execution_code.rhai). 


#### Create a Code Template
We also have included the code for creating a couple of additional Code Templates. 

You can find these in the [RAVE Templates](./rave_templates) folder.

You can get started by clicking Create Code Template on the RAVE Library page.

And if you feel like writing a custom one, go ahead and take a swipe at it.* 

**Note that since this is a single shared testing environment, all RAVEs and Executable Agreements will be visible to all other testers.*

For a reminder on this layered structure [Intro to RAVEs (Three Layers)](./1_2_three_layers_of_raves.md) has a decent overview.


#### Components of a Code Template 
Each code template has a few components:
* Template Title
* Input Signature (JSON Schema)
* Execution Code
* Output Signature (JSON Schema)

Once you have created a Code Template, you can use that Code Template to create an Executable Agreement by clicking Create Agreement.

#### Create Executable Agreement
When creating an Executable Agreement, there are a few ways in which a particular bit of data can get input, such as the amount of the transaction, or which agent will be the spender.

* RAVEFixed
    * A fixed input (used in RAVEs)
* ExecutorProvided
    * Provided by the Executor
* Fixed
    * A fixed input (used in Direct Transactions)
* Query 
    * Allows you to query the [HDK](https://docs.rs/hdk/latest/hdk/), 80% of the time this will be a get_links call. See this [example code template](https://github.com/unytco/domino-releases/blob/develop/testing_docs/rave_templates/test_query_rave_template.md)). Or [learn more about links and get_links](https://developer.holochain.org/build/links-paths-and-anchors/).

Once you have created an Executable Agreement, click one of the Initialization Methods (PI, PP, or Ex) to start creating a RAVE that conforms to that Agreement. Once initialized, the agents listed in the agreement will get notified that they have Actionable Transactions when it is their turn to take some action. (again, this may require a reload to show up.)

If you have any thoughts, questions or criticisms, please feel free to add items to our [feedback board](https://github.com/orgs/unytco/projects/5/views/1), and don't hesitate to reach out to our team.
