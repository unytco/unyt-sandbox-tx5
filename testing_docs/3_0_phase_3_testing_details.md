# Phase 3 Testing Details

#### links to related docs

- [Test Plan](./1_0_testing_plan.md)
- [Unyt Setup](../README.md)
- [Testing Documentation, Phase 4](./4_0_phase_4_testing_details.md)
- [Unyt Dictionary](./4_2_unyt-dictionary.md)
- [Intro to Smart Agreements (Three Layers)](./4_1_intro_to_smart_agreements.md)
- [Templates and Smart Agreements Library Repo](https://github.com/unytco/smart_agreement_library)
- [Feedback](https://github.com/orgs/unytco/projects/5/views/1)

**WARNING: This doc contains details!**

Feel free to use the outline view available in the upper right to get an overview of the different content covered.

For older versions, see [Phase 1 testing details](https://github.com/unytco/hfvz-releases/blob/develop/testing_docs/1_1_phase_testing_details.md) and [Phase 2 Testing Details](https://github.com/unytco/hfvz-releases/blob/develop/testing_docs/1_1_phase_2_testing_details.md).

## Phase 3 Testing Orientation

### Overview of context

Domino, by Unyt Accounting is decentralized software for microtransaction accounting.

Why is it called Domino? It is software for Decentralized Microtransaction Network Orchestration, or DMNO. We pronounce it Domino.

At present, we are primarily focused on supporting Decentralized Physical Infrastructure Networks (DePIN) with transaction processing designed to handle high volumes of activity with very low costs.

Domino Features:

* Lightweight (can run on ordinary consumer devices)
* Low Cost (cheap)
* Support for Microtransactions (economically viable even for transactions worth pennies)
* Parallel Transaction Processing (fast and high volume)
* Customizable Smart Agreements (an agent-centric alternative to blockchain style smart contracts)
* Complements a project's existing blockchain token(s) and can be used alongside them.
* Enables delegation of a project's compliance with financial regulations (Crypto / Anti Money Laundering / Know Your Customer) freeing teams to focus on their core value proposition.

Domino can account for both payments and for services provided in accordance with smart agreements custom designed to meet your communities needs.

There is no one global Domino app. Instead, each community runs its own separate stand-alone software network, operating on the machines of -- and governed by -- its participants, typically with support from (though not control by) Unyt Accounting. We refer to these stand-alone, peer-to-peer applications as Domino Accounting Alliances (or just Alliances for short).

Each Alliance has its own name, its own payment unit of account (the general name we use is Fuel Credits), and its own sets of smart agreements that are used and enforced -- all in a decentralized manner.

For Testing purposes, all testers will be participating in the same Domino Accounting Alliance, which uses Holo Fuel or HF as its Fuel Credits. As a result, all testers are joining a single peer network and will be able to send and receive credits from anyone else in the test network as well as see and interact with any of the smart agreements that other testers create.

All of you will be using the software with the permissions of a super user, with the ability to not only interact with Smart Agreements and send direct transactions, but the ability to create new Smart Agreements as well.

Since this is a shared application and social space, any Agreement Code, Smart Agreement or Smart Agreement Verifiable Execution Doc (SAVED) that you create will be visible to other testers.

Domino is being built by a small, focused team, several of whom helped found Holochain, the framework for building peer-to-peer applications that powers Domino's ability run as a distributed application.

#### Customer Focus #1: DePIN

Our roots and our initial focus is on supporting Decentralized Physical Infrastructure Networks (DePINs) by providing them with superior internal transaction processing that can complement their existing blockchain architectures and tokens.

Domino facilitates accounting that recognizes the provision and utilization of services and not just a demonstration of capacity. It is currently being optimized for contexts where participants (such as a Host in a DePIN Hosting Network) can provide Proofs of Service for services that they (or devices under their control) have provided to others.

To use a simple metaphor, pretend your DePIN project is a ridesharing service.

1) Do you pay drivers just for installing your app and running it?

2) Or is your system structured to recognize when drivers are doing work that is useful for others -- like giving people rides to where they want to go -- and compensating them for that?

If you are in the first group, there may still be ways to use Domino, but those aren't the kinds of projects that we are actively seeking to serve at present. Instead, we are working mostly with DePIN projects that share our focus on recognizing and rewarding real productive contributions.

We see our efforts as a domino. Not the first. Not the biggest. But also, not the only. Maybe with your help, we can start to trigger a chain reaction in the realms of coordination, contribution and compensation.

### A Request

As you test Domino, please try to imagine new ways that you and your team might be able to use this to help recognize real contributions and foster more of them.

We love to imagine and strategize about new possibilities alongside our partners and then work to bring them into being.

If you would like to run something by us, feel free to reach out via whatever channel your team is already in communication with us (Telegram, Signal, etc) or simply by sending an email to:

info@unyt.co

### Version

In Phase 3, we are testing version 0.13.0 of Domino (formerly titled HFvZ).

### Reminders

1) This version of the software hasn't had its UX/UI refined for a particular use case or for a particular set of end users.

2) This version is intended to help partners and their developers gain familiarity with the features and functionality of Domino, and

3) help those teams begin exploring the ways in which they may want to make use of Domino for their use cases.

### Priorities for Phase 3 Testing

1) We are seeking guidance on overall features and functionality and

2) We want to support Testing Teams in designing, creating and testing their own Smart Agreements that deliver on the business logic relevant for their context.

## Overview of what is NEW in Phase 3

The biggest changes since Phase 2 testing are:

1) Some renaming of features, for example:
   - The software itself has been renamed from HFvZ to Domino

   - The action of sending credits has been renamed from PROMISE to SPEND.

   - EXECUTABLE AGREEMENT has been renamed SMART AGREEMENT.

   - The generic term for a community running their own custom instance of Domino has been renamed from POOL to DOMINO ACCOUNTING ALLIANCE or DOMINO ALLIANCE for short.

   - Swim Lane has been renamed Service Network Console

2) The addition of Roles for interacting with an executable agreement and

3) the implementation of example Smart Agreements, including for Sales and Redemption of that Alliance's Fuel Credits for a particular Service Network's external currency (ex: blockchain token).
   - Sales and Redemption refer to the ways that your existing Token holders could use their tokens to purchase Fuel Credits, and the ways that service providers in your network could redeem Fuel Credits for tokens.

## Table of Contents

INTRODUCTION

DOMINO DICTIONARY

AGENT MODAL

- Connection Status and Sync
- Your Agent and Identicon
- Your Balance
- Fuel Balance
- Fees Owed

LEDGER DETAILS

- Fuel Balance
- Fees Owed
- Unclaimed
- Credit Limit (as of last check)

TRANSACTIONS

- Activity
  - Create Transaction (button)
  - Action Table
  - Completed Transactions

- Executor
  - Requests to Execute Agreements
  - Executed Agreements

LIBRARY

- How to Create a Smart Agreement
- Add Agreement Code Template
  - Title
  - Input Schema
  - Execution Code
  - Output Schema
- Create a Smart Agreement from an Agreement Code Template
  - Title
  - Execution Rules
    - One Time Run (toggle on / off)
    - Aggregate Execution (toggle on / off)
    - Who can Execute?
      - Anyone
      - Authorized Executor
  - Roles
    - Role Name
    - Description
    - Qualification Type
      - Any
      - Authorized
      - KYC
  - Input Rules
    - Provided by Role
      - Select Role
    - Executor Provided
    - Fixed
    - Query
    - Custom (Preset Query)
- Examples and Guidance for Creating Custom Smart Agreements
- Viewing Smart Agreements
- Interacting with a Smart Agreement, including as an Executor

DATA RECORDS

- Creating New Data Records
  - Creaate a Conversion Table
  - Create a Recipe
  - Create a Code Sheet
  - Add New Data Record (Custom)

- Data Records Table
  - View Data Records

ADMIN
- Global
  - Global Configuration
    - Global Config Window
      - Effective Start Date
      - Expiration Date
    - System Agreements
      - Compute Credit Limit Agreement
      - Compute Transaction Fee Agreement
      - Fee Trigger
      - Fee Percentage
    - Additional Information
      - Additional Special Agents
      - Global Configuration declared Smart Agreements
  - Service Units
    - Fuel Credits
    - Non-Fuel Service Units
  - Service Networks
    - Initialize Service Network
      - Service Network Config
        - Basic Properties
          - Name
          - Abbreviation
          - URL
          - Outside Currency Name
          - Outside Currency Symbol
          - Service Network Admnistrators
        - Service Network Config Window (Previously "Lane Definition". UI update in progress)
          - Effective Start Date
          - Expiration Date
        - Special Agents
          - Ops Account
          - Service Infrastructure Account
          - Sales Agent
        - Smart Agreements
          - Purchase of Units
          - Redemption of Units
          - Proof of Service
    - View Service Network Config
    - Service Network Console

SERVICE NETWORKS (the tab)
  - Smart Agreements
    - Purchase of Fuel Credits
    - Redemption of Fuel Credits
    - Proof of Service
  - Add Conversion Sheet
  - Drop off Proof of Deposit
  - Redemption Agent Redeems
  - Current Redemption Rate

## INTRODUCTION

When you use a Domino application, it is not being served by a webserver. It is instead run locally on your computer and connects with the computers of other participants to form a peer-to-peer application. It is built using Holochain, which is a free, open-source framework for creating apps that run in this distributed manner with no corporate web servers or blockchain tokens needed to facilitate the interactions of the participants.

Also, there is no global Domino application. Domino is not a platform. It is a type of peer-to-peer application, run by the members of that particular community and enforcing the rules that they have opted to create and use.

Each community runs their own version of Domino, customized to meet their needs. We refer to each of these as a Domino Accounting Alliance, or Domino Alliance for short.

Each Domino Alliance defines the Unit of Account that they will use within their community as their primary payment unit. The generic term that we use for a Domino Alliance's payment unit is Fuel Credit. Each Domino Alliance will choose a name and an abbreviated symbol for their Fuel Credit.

Each Domino Alliance will also define for themselves other details such as:
1) the kinds of services that they are choosing to keep track of in addition to payments,

2) The methods by which those services are priced in Fuel Credits.

3) How Fuel Credit Limits are determined in that community. A Fuel Credit Limit determines how far into the negative a particular agent is allowed to spend.

4) Which external currencies (such as Service Network related blockchain tokens) can be used to purchase the Fuel Credits of that Alliance.

5) What specific requirements need to be met for an agent to be eligible to redeem Fuel Credits for a specific external currency.

Next lets walk through the different components of a Domino application, one component at a time.

## Setup and Login
Peers start by installing a Domino Alliance app on their computer and then going through setup.

See the [README](https://github.com/unytco/domino-releases?tab=readme-ov-file#domino-releases) section of the Domino Releases Repository for Installation and Set up instructions.

At present, this may involve giving the application permissions to run on your machine. See [Setup](https://github.com/unytco/domino-releases?tab=readme-ov-file#setup) for details.

After setting up Domino and signing on (with or without a password), you may get a notification that it has not yet synced. This should not take long, assuming at least one other user of the app is online that has synced with the progenitor that set up this app version. That progenitor has added some Smart Agreements to set some initial groundrules (including a system_credit_limit_computation) that we are going to be using. They (and maybe others) have added other Smart Agreements to the Library as well that we can try out. Once you are synced, you will have a credit limit available and will be able to spend into the negative up to your credit limit.

### Let's test together

Feel free to try out some transactions. If you don't have anyone else to test alongside, you can Send Credits to, or Request Credits from Matt Schutte with a Spend or an Invoice.

Matt's public key is:

```
uhCAkOKFD_M3OuSQ8q-oEMSC-gKOHIJuchdp8eS1W1jnWPnWAW65F
```

And feel free to reach out to us through our shared comms channel on Telegram or Signal or via email at info@unyt.co

We are more than happy to jump on a video conference call to test alongside you for a bit.



### AGENT MODAL

The Agent Modal provides a summary of several key pieces of information about your account in this particular Domino Alliance.

The Agent Modal is moveable. Click and drag vertically to reposition.

#### Connection Status and Sync

When disconnected from other application peers, status is "Disconnected"

When connection to peers is established, status is "Connected"

When synchronization of data with peers has not yet been completed, status is "Waiting for network sync..."

When data has been synchronized with other peers, an alert is temporarily displayed that indicates: "Syncing complete. Ready to use!"

#### Your Agent and Identicon

Other participants will percieve the actions that you take as coming from a particular Agent, your Agent. Domino makes use of public/private key cryptography to enable participants to enable you to choose to show up in the Domino Alliance as the same actor (agent) over and over again. Every action you take makes use of your (current) public key, whether it is being used as an address to send Fuel Credits to, or you are making use of the public key (and its associated private key) to sign some particular action, such as a payment or "spend" transaction, or whether your public key is being assigned a Role and thus granted the authority to interact with a Smart Agreement in a particular way. With your agent, you are taking actions within the shared social space of this particular Domino Accounting Alliance and others are able to recognize that those actions are being taken by the same party.

Next to "Your Agent" is a colorful circle. This is what we refer to as an "identicon." It is an image that is generated based on your agent's public key. The intent with identicons is to give people an easy way to recognize that a particular public key might not be the same as it usually is. People are better at recognizing a picture that looks different than they are at seeing a long string of characters and assessing whether or not there might be one or more characters that have been altered.

Your public key is meant to be shared with others or used to interact with smart agreements etc.

Click the identicon to copy your agent's public key to your clipboard. Then paste it into other areas of this Domino application to interact with Smart Agreements perform direct transactions. You can also share it directly with other people through whatever medium you wish, whether via email, signal, telegram, hand written letter or something else altogether.

They will enter it on their own copy of this Domino Alliance when doing things like sending you an invoice (a request for payment) or a promise (a sending of payment) or when granting you authorization to interact with a Smart Agreement through a particular Role.

#### Your Balance

##### Fuel Balance

We refer to the payment units in a Domino Accounting Alliance as Fuel Credits. 

Your Fuel Balance (or Fuel Credits Balance), is displayed in bold and shows the present Fuel Balance  of your account (positive or negative.

This represents the total number of Fuel Credits that you have accepted from other agents minus those you have spent (including those set aside for payment of service and transaction fees)

Each Domino Alliance will choose its own name and symbol for its Fuel Credits. 

In the current test version, we are using Holo Fuel as the name and HF as the symbol for the Fuel Credits in this particular Domino Alliance. Once your device syncs with other peers, you should see this change from "0 X" to "0 HF". This is one indicator that you have synced with other peers.

Other communities will have their own Fuel Credits. An Electricity Microgrid Alliance, for example, might choose to name their Fuel Credits "Joules" and use "JL" for the Symbol.

That will be an independent accounting alliance and will be run on the devices of those participating in that particular alliance.

##### Fees Owed

The stewards of a particular Domino Alliance can structure that application to charge Transaction Fees when Fuel Credits are spent.

In this test version, a Transaction Fee of 1% is owed when an agent Spends Fuel Credits by sending them to an agent directly or by sending them to a Smart Agreement.

The Transaction Fees that are owed -- but have not yet been paid -- are set aside (earmarked) and will be paid once some accumulation threshold is met.

That Fee Payment threshhold is set by each Domino Alliance.

In this test version, that threshold is set at 1 HF.

Once the threshold is crossed, the next transaction the agent engages in will be preceded by the sending (payment) of fees owed.

### LEDGER DETAILS

The Ledger Details Modal is accessed by clicking on your Fuel Credits Balance in the Agent Modal.

#### Fuel Credits Balance
This section displays the Name and Symbol of this Domino Alliance's Fuel Credits as well as your agents Fuel Credits Balance.

#### Fees Owed Details

Transaction Fees (in Fuel Credits) that have accrued, but have not yet been paid are shown here. This is the same information that was available at the bottom of the Agent Modal.

#### Unclaimed

The sum of all Fuel Credits that have been allocated to you and are available to collect, but that you have not yet accepted.

This is, in some ways, a reminder to go claim the credits that others have already sent to you.

#### Credit Limit

Displays the credit limit that was returned after the last time your software checked your credit limit. Your credit limit is the amount below zero that you can spend to in accordance with the applications rules. EX: in this test version of Domino (v 0.13.0), each participant gets a credit limit of 10,000 HF. As a result, each agent could spend up to 10,000 units below zero (i.e. until their balance is -10,000). After that, attempts to spend additional credits would be fail.

### TRANSACTIONS

#### ACTIVITY

##### Create Transaction (button)

Domino supports the sending or requesting of payments directly. Accordingly there are two Transaction Types available: Spend and Invoice.

1) Spend

Domino supports sending Fuel Credits directly to another agent. Under Transaction Type, select Spend.

In the Amount section, enter the number of Fuel Credits that you want to send.

In the Receiver section, enter the public key of the agent to whom you are wanting to send the credits.

You can also add a note. This is optional.

As soon as you click "Create Spend", your device will check for several potential issues with the transaction - including whether the Receivers Public key is appropriately structured and whether the spend (plus transaction fees) would bring your account balance beyond the credit limit available to you.

Assuming there are no such validation violations, the spend will complete, your device will "sign" the transaction with your private key (which allows other devices to confirm that it was actually you trying to send some of your credits) and your account will be debited the amount of the transaction, plus transaction fees. 

In addition, your device will publish a notification that will alert the receiver that they have credits available to accept.

We will cover Accepting credits in the Action Table section below, but first lets cover the other Transaction Type supported in the Create Transaction Modal.

2) Invoice

Domino also supports requesting Fuel Credits directly from another agent. Under Transaction Type, select Invoice.

In the Amount section, enter the number of Fuel Credits that you want to request.

In the Spender section, enter the public key of the agent that you are requesting to send you Fuel Credits. If you do not have the agents public key yet, you can ask them to send it to you through any medium. See the "Your Agent and Identicon" section, above.

You can also add a note. This is optional.

As soon as you click "Create Invoice", your device will check for several potential issues with the transaction - including whether you have filled out all required fields, and whether the Spenders Public key is appropriately structured.

Assuming there are no such validation violations, your device will "sign" the Invoice with your private key (which helps other devices to confirm that it was actually you trying to request some Fuel Credits) and you should see the Invoice Transaction show up in your Action Table as a "Pending" transaction.

In addition, your device will publish a notification that will alert the Spender that they have an Invoice request that they can take action on. It will show up in their Action Table as an "Actionable" transaction.

##### Action Table
This is where you can view in-process transactions that are either:
a) ready for you to act on (Actionable) or
b) awaiting action by another agent (Pending)

Let's review the different columns in the Action Table.

###### ID

This is a transaction ID that is generated based on the content of the initial action.

###### AMOUNT

The number of Fuel Credits involved in the transaction. These are also color coded as Green (Incoming) or Red (Outgoing) to give an easy visual indicator of whether the transaction will result in an increase in your Fuel Credit Balance (Incoming) or a decrease in your Fuel Credit Balance (Outgoing).

###### STATUS

Transactions will either be Actionable or Pending. Once all required actions on your part have been completed, the transaction will be moved to the Completed Transactions table. This is true even if one or more other agents still have other actions to take, such as accepting Fuel Credits that would only affect their balance(s). If you have already "spent" the credits, from your perspective the transaction is complete. The credits have been debited from your account. This is true regardless of whether or not the receiver has accepted them yet.

Actionable
Indicates that it is possible for you to take some action on that item at present.

Pending
Indicates that there are no actions for you to take, but that you are waiting on another agent to take some action, at present. This might happen when you have invoiced another agent, but they have not yet sent the Fuel Credits you requested.

###### DIRECTION

Incoming
The transaction, if and when completed, will result in an increase in your Fuel Credit Balance (Incoming) or .

Outgoing
The transaction, if and when completed, will result in a decrease in your Fuel Credit Balance.

###### TYPE

Spend
A sending of credits by the Spender to a Receiver. 

Invoice
A request for credits from a Receiver to a Spender.

Parked
A submission of content to a Smart Agreement, whether an invoice, an attestation, a notarization, or some other content. A variety of custom Smart Agreements will likely make use of the "parked" type.

ParkedSpend
A sending of credits by the Spender to a Smart Agreement.

SAVED
The Document Recording a specific Verifiable Execution of a Smart Agreement. SAVED stands for Smart Agreement Verifiable Execution Doc. In earlier versions, this was referred to as a RAVE.

Accept
An acceptance of a transaction.

###### SPENDER

The identicon of the agent whose Fuel Credit Balance will be debited (decreased) as a result of the transaction, if it is completed.

Hovering over the identicon will display the public key of the Spender.

Clicking on the identicon will copy the Spender's public key to your clipboard for easy re-use elsewhere.

###### RECEIVER

The identicon of the agent whose Fuel Credit Balance will be credited (increased) as a result of the transaction, if it is completed.

Hovering over the identicon will display the public key of the Receiver.

Clicking on the identicon will copy the Receiver's public key to your clipboard for easy re-use elsewhere.

###### DETAILS

This provides a summary of the transaction information, including details such as the participants and the payload, if any. A payload might be as simple as a note that was added by the Spender when they created the transaction.

###### ELAPSED (Previously TIMESTAMP. UI is being updated.)

Timestamp shows an approximation of the amount of time that has passed since the transaction had its previous action taken.

###### ACTION

If there is an action that you can take at present on a particular transaction, the action button, whether a "Pay", an "Accept" or something custom will be presented here. If you are presented with a Pay button, once you click Pay (assuming you have the required credits etc) your account will be debited the requested amount, plus any associated fees. If you are presented with an Accept button, once you click "Accept" assuming all of the content is valid, you will be credited the amount presented. If there is no action available for you to take at present, for instance, when the transaction is listed as "pending", there will be no action button available.

##### Completed Transactions

We have moved completed transactions to their own section at the bottom titled, creatively: "Completed Transactions."

As discussed in the Action Table section, a transaction with a status of "Completed" indicates that there are no additional actions for you to take. From your perspective the transaction is complete.

Most columns are similar to their counterparts in the Action Table. The status of all transactions should be "Completed." There are no actions to be taken in the Completed Transactions table. Think of this as a historical log that is useful as a reference.

That concludes the walk through of the Activity tab. Next up: Execution!

#### EXECUTOR

In Domino, Direct Transactions are executed by just the Spender and the Receiver themselves through their individual actions. However, for more complicated transaction logic, Domino supports Smart Agreements that make use of a particular party as the Executor. A Smart Agreement can be configured to support either anyone acting as the Executor, or a specific party serving as the Executor.

##### Requests to Execute Agreements

When you have been assigned to Execute one or more Smart Agreements, each Agreement that has been interacted with will show up in the Requests to Execute Agreements table.

There are three columns: EA ID, EA NAME and ACTIONS.

###### SA ID (Previously EA ID

Displays the identicon for the Smart Agreement. Hovering over the identicon will display the ID (the Hash) of the Smart Agreement. Clicking on the identicon will copy the ID to the clipboard.

###### SA TITLE (previously EA NAME. UI is being updated)

Displays the Smart Agreement Title.

###### ACTIONS

Button to Execute the Agreement. When clicked, an Execute Agreement modal gets displayed. This modal will include the Executor Inputs that are required, per the Smart Agreement as well as the option to Execute the Agreement, or simply close the modal.

At present, there is not an automatic check to make sure that all required parties have interacted with the Smart Agreement. If you attempt to Execute the agreement and not all conditions are satisfied, Execution should fail. You can try to execute again at a later date.

##### Executed SAVEDs (previously named Executed RAVEs. UI is being Updated)

This table shows completed Executions of Agreements (SAVEDs) that you have Executed.

The columns include: ID, EA, EA TITLE, AMOUNT, SPENDER, RECEIVER, DETAILS, TIMESTAMP.

###### SAVED ID (previously ID. UI is being updated)

Displays the Identicon of the SAVED. Hovering displays the Address of the Execution Document and clicking the Identicon will copy the address to the clipboard. This address allows peers to refer to or retreive the SAVED from this Domino Accounting Alliance's distributed storage.

###### SA ID (previously EA. UI is being updated)

Displays the Identicon of the Smart Agreement that was executed to create this RAVE/SAVED. Again, hovering will display the address of the agreement. Clicking will copy that address to the clipboard.

###### SA TITLE (Previously EA TITLE. UI is being Updtated)

Displays the title of the Smart Agreement that was executed to create this SAVED (the verifiable execution doc).

###### AMOUNT

Total number of Fuel Credits transferred in the execution of this transaction.

###### SPENDER

Displays the Identicon of the Spender, if there is a Spender that interacted in the Smart Agreement. Hover to display the spender's public key (which is also their address). Click to copy to the clipboard.

###### RECEIVER

Displays the Identicon of the Receiver, if there is a Receiver that Fuel Credits were made available to as a result of the execution of the Smart Agreement. Hover to display the Receiver's public key (their address). Click to copy to the clipboard.

###### DETAILS

Click on the Details document to display a modal with the Transaction Details of the SAVED, including information about Participants such as Spenders, Receivers or other Roles, Transaction Information such as the Total Fuel Credits Transferred In/Out, and the Definition (the set of Global Governing Rules that were in place for this Domino Alliance at the time of the previous Action.)

###### ELAPSED (Previously TIMESTAMP. UI is being updated)

The Timestamp of the Execution of the Smart Agreement that resulted in the creation of the SAVED. This timestamp is from the perspective of the Executor themselves - i.e. yours.

### LIBRARY

The Library is where participants can interact with Smart Agreements and where those with admin access can create the different components required to create and distribute new Smart Agreements.

For an introduction, see [Intro to Smart Agreements]().

Assuming you have read through that, or already have an idea of what Smart Agreements are, lets explore how they are made.

#### How to Create a Smart Agreement

The creation of a Smart Agreement involves a few steps:

First, you need to create an Agreement Code Template (or "Agreement Template" or simply "Template" for short). This will provide the core logic of your Smart Agreement.

Once an Agreement Code Template has been created, a number of different Smart Agreements can make use of that Template but each can be configured differently, whether that involves different sources for inputs, different roles created and authorized to take particular actions, or authorizing different agents to perform some particular action.

In other words, Agreement Code Templates make it easy to re-use the core logic in multiple contexts, which supports both clarity about which particular ruleset is being made use of in a specific context, as well as community learning with regard to which patterns work well, and where.

Lets now dive into the details of the three main elements of the Library:

- Adding an Agreement Code Template,
- creating a Smart Agreement, and
- interacting with a Smart Agreement, including as an Executor

#### Add Agreement Code Template

If you click the "Add Agreement Code Template" button on the upper left of the Library screen, it will bring up the Create Agreement Code Template modal.

An Agreement Code Template enables a particular set of basic rules to be re-used by multiple Smart Agreements. Those rules include:
  
- the required structure of the inputs (the input schema),
- the core logic of how those inputs will be transformed to create outputs (the execution code),
- and the required structure of the outputs (the output schema).

To create one, give your Agreement Code Template a Title, define those three sections, and then click Create Template.

For inspiration, check out other Templates in the Library in Domino itself or view examples in the [Agreement Code Templates Library](https://github.com/unytco/rave_library)

Once an Agreement Code Template has been created by any agent (with admin privileges) in the Domino Alliance, it will be made available to everyone running the software and will be published as a new row in the Agreement Code Library.

From the Agreement Code Library, on that Template's row, clicking on the ">" symbol on the left side of the row will display any Smart Agreements, if any have been created.

To the right of that, an edit icon is available which will enable participants with admin privileges to edit that Agreement Code Template. Note that all testers have admin privileges in this current test version of the software.

Next, the Title of the template is shown.

Then the different components of the Agreement Code Template are each made available separately, as:
CODE, IP SCHEMA, AND OP SCHEMA.

CODE displays the Execution Code - i.e. the ways that Inputs will be Transformed into Outputs.

IP SCHEMA displays the required structure of the inputs. (Previously IP Signature. UI update in progress.)

OP SCHEMA displays the required structure of the outputs. (Previously OP Signature. UI update in progress.)

#### Create a Smart Agreement from an Agreement Code Template

A Smart Agreement is a specific implementation of a code template with additional definitions of:

- a Title,
- Execution Rules, including who is authorized to serve as Executor of this specific Smart Agreement,
- Roles, and
- the required source of each Input.

We'll go through each of the items in the Create Smart Agreement Modal, one-by-one.

##### TITLE

Give this particular Smart Agreement a human readable Title for easy reference and differentiation from others.

##### EXECUTION RULES

###### One Time Run

To create a Smart Agreement which can be Executed only once, select One Time Run / Single Execution.

To create a Smart Agreement which can be Executed multiple times, the Executor will gather the inputs that are available at that time. Note that inputs that have been processed in a prior execution would no longer be available, though outputs from a prior execution (in the SAVED that documents that execution) might be available as an input for a subsequent execution.

###### Aggregate Execution

Aggregate Execution enables the Executor to gather multiple Inputs that have been parked on the Smart Agreement and process them all as part of a single Execution. This supports bulk processing of inputs.

###### Who can Execute

There are two options regarding who can Execute a Smart Agreement: Anyone or only an Authorize Executor.

1) Anyone

Any agent can execute the agreement.

An example:
Under the __system_credit_limit_computation Template, you can find the "credit check v0.1.0" Smart Agreement. This is an agreement that any agent can execute. 

Anytime an agent attempts to spend into the negative, Domino first has your agent automatically execute this credit check v0.1.0 agreement, which creates an output with your agent's credit limit and publishes that to your source chain. If the spend transaction you are attempting will not result in your Fuel Credit Balance going below that credit limit, the transaction will proceed. This also makes it easy for others to later validate what your credit limit was just prior to your attempted spend. It is always provided by the prior action, which they can independently verify was validly executed.

2) Authorized Executor

The alternative to allowing anyone to execute a Smart Agreement is to allow only a specific, pre-determined "someone" to serve as the executor. The public key of the Authorized Executor must be entered as part of the definition of the Smart Agreement. If another agent attempts to Execute the agreement, their attempt would be recognized as invalid by any other peer.

Domino does not currently support having two or more specific Authorized Executors.

##### ROLES

Roles are one of the main changes to Domino since Phase 2 testing.

Roles enable you to define classes of agents and grant authorizations based on membership in the class.

"+ Add Role"
Adding a Role inlvolves creating a Role Name, a Description, and setting the Qualification Type.

There are three options for Qualification Type:

1) Any
Anyone can interact with the Smart Agreement through this Role

2) Authorized
Only specific Authorized Agents can interact with the Smart Agreement through this Role, and

3) KYC
A specific third party can authorize an agent to interact with the Smart Agreement through this Role. This "delegation of authorization" feature is planned, but has not yet been built.

Multiple Roles can be added to a Smart Agreement.

##### INPUT RULES

For each input defined in the Agreement Code Template, you will need to select a Source where the Executor will obtain that input when they are assembling content to Execute the Smart Agreement and produce a SAVED.

There are 5 different types of Sources that you can require an Input to come from:

1) Provided by Role

After you have added 1 or more Roles, you can require that a particular input be provided by an agent interacting with the Smart Agreement through a particular Role.

2) Executor Provided

The input is provided by the Executor themselves.

3) Fixed

The input is defined (hard coded) into the agreement itself at the time the agreement is created.

4) Query

You can define a query that the executor will perform when gathering inputs for execution and will include the results of the query as the input.

5) Custom (going to be deprecated)

The name is misleading. This option was less about custom inputs and more about the use of pre-set Queries to retrieve inputs. Currently, "Get from Executor's Parked Spend" and "Get" are supported.

"Get from Executor's Parked Spend" is intended to be used in Smart Agreements where during an execution, a leftover spend amount that is not yet allocated to other receivers gets Parked back on the same Smart Agreement by the Executor. During the next execution, that Parked Spend would be pulled in by the Executor as an input during the subsequent execution. This process could repeat so long as there are Fuel Credits that have not yet been allocated.

"Get" was a similar timesaver for our backend dev team to test functionality quickly without having to having to specify all options.

We are currently considering getting rid of Custom, and creating a more rich set of options and editable templates within Query. That will provide greater guidance on how to structure a query.

Let us know if you are interested in that.

Once a Smart Agreement has been created, it appears in the Agreement Code Library under the Agreement Code Template from which it is derived.

##### Examples and Guidance for Creating Custom Smart Agreements

We want to help you create custom Agreement Code Templates and Smart Agreements that you believe might be useful in your context. To that end, we have created a Templates and Agreements Library as a Github Repository. It has some additional examples and guidance and we would love to have you create new Templates and Agreements, try them out in Domino and add them to the [Templates and Agreements Library](https://github.com/unytco/rave_library). This is a Repo that will enable sharing across Domino Accounting Alliances enabling a wider community of projects to view, play with, and gain inspiration from one anothers' Smart Agreement efforts.

For a summary and guidance, start by looking at the comments at the top of the execution_code.rhai file in any particular Template. Any example Smart Agreements should be stored in an Agreements directory within each particular Agreement Code Template directory.

#### Viewing Smart Agreements

If you have clicked the drop down arrow on the left side of an Agreement Code Template (or any white space on that row), it will show you any Smart Agreements that have been created from that template.

On the left side of the Smart Agreement, there is a CLONE icon available. This enables the Smart Agreement content to serve as a starting point, and can be edited to produce a new, different Smart Agreement.

Next is the SA ID, or Smart Agreement ID.

Then the TITLE.

The document under INPUT RULES will display each input and its required source (where the input should come from).

The document under EXECUTION RULES will pull up the details concerning who can Execute the Smart Agreement, as well as whether a) it is an agreement that can only be executed once or can be executed multiple times and b) whether Aggregate (or bulk) Execution of the agreement is allowed.

The document under ROLES will show the name, description and qualification requirements for any ROLES that have been defined in the Smart Agreement.

On the right side of each Smart Agreement in the Library is the "Interact As" button.

#### Interacting with a Smart Agreement, including as an Executor

When you click on the Interact As button, you will be presented with options to interact with the Smart Agreement in one or more ways, including as an Executor, and (if other Roles have been defined), as each Role that was defined in that Smart Agreement.

When you click to interact with a Smart Agreement through a particular Role, if there are Inputs that are to be contributed from that Role, you will be presented with the relevant fields where you can enter the data for each of those inputs.

After any optional and all required fields are filled out, click Create to publish the interaction with the Smart Agreement.

This will publish your action on your own Source Chain, and depending on what sort of action you took, will also publish links that are used as indicators that others can find regarding your actions and their consequences. Those links may be attached to the Smart Agreement itself or may be attached to the AGENT ID of a specific participant. For instance, if you executed the agreement and one of the results was to allocate 5 Fuel Credits to an agent, Alice, then you would also publish a link to Alice's AGENT ID that would let Alice know that she has 5 Credits available to be Accepted.

In such a case, Alice would see a notification in the Action Table in the Transactions section of her instance of the Domino Alliance software and she would be able to Accept the Credits by clicking the Accept button.

If Service Networks have been added to a Domino Accounting Alliance, then there will be a Service Networks Tab visible next. We cover Service Networks first in the ADMIN section and then have a SERVICE NETWORKS Section after ADMIN to cover additional details.

### DATA RECORDS

The Data Records Tab enables agents to store larger files and pieces of content in a way that can be referenced from elsewhere within the software, for instance as an input to a Smart Agreement.

There are a few different types of Data Records that can be created at present:
a Conversion Table, a Recipe, and a Code Sheet. Each of these could be used to define service pricing. The Code Sheet could also be used for other more general purposes.

#### Creating New Data Records

##### Create a Conversion Table

Creating a Conversion Table involves defining the number of Service Units required to equate to 1 Fuel Credit. This can then be referred to from within a Smart Agreement, resulting in a fixed price in Fuel Credits for a specific service. 

##### Create a Recipe

On the other hand, a Recipe enables one or more Service Units to be defined in relationship to 1 Fuel Credit. In some ways, this results in a basket of goods pricing structure. For instance, 1 HF might buy you 3 gigabytes of Bandwidth and 4 Requests.

##### Create a Code Sheet

A code sheet allows you to define custom logic for a price sheet.

##### Create a New Data Record

Price sheets aren't the only kind of Data Record that you can create. The "Add New Data Record" button allows you to create any sort of Data Record that you want, in JSON format. The button is on the right side. Just below Code Sheet. Sometimes you need to move the Your Agent Modal to see it.

#### DATA RECORDS TABLE

In the DATA RECORDS TABLE, each row will display the following details for a specific Data Record:
ID, PREVIEW, DATA, and TIMESTAMP.

##### DR ID (Previously ID. UI update in progress.)

DR ID displays the Identifier for the Data Record. Hovering over it will display the DR ID. Clicking it will copy the DR ID to the clipboard.

##### PREVIEW

PREVIEW shows a short snippet of text from the Data Record.

##### DATA

Clicking on the document in the DATA column will bring up a modal with the full text of the Data Record.

##### TIMESTAMP

TIMESTAMP shows the creation date and time of the Data Record.

### ADMIN

The Admin tab is focused on enabling the administration of the Configuration (settings) for your particular Domino Accounting Alliance as a whole, as well as the Configuration portions of the system that will be stewarded by particular Service Networks in that Alliance.

#### GLOBAL

By default, a progenitor agent plays the role of setting up and updating these basic ground rules for this particular Domino Alliance.

#### GLOBAL CONFIGURATION

At any one point in time, there is a single ruleset that governs this Domino Alliance as a whole. We refer to to the ruleset as the Global Config. Config is short for Configuration. We refer to the particular period of time that a particular Global Config ruleset is in effect as the Global Config Window.

A Global Configuration can have a handful of components:

- Global Config Window
  - Effective Start Date
  - Expiration Date
- System Agreements
  - Compute Credit Limit Agreement
  - Compute Transaction Fee Agreement
  - Fee Trigger
  - Fee Percentage
- Additional Information
  - Additional Special Agents
  - Global Configuration declared Smart Agreements

Lets walk through each of them 

##### Global Configuration Window

A Global Config Window has an Effective Start Date as well as an Expiration Date.

Between the Effective Start Date and the Expiration Date, any action that is attempted by any participant must not violate any of the rules established in the Global Configuration. After the Expiration Date, interactions must comply with whatever Global Configuration Ruleset is in effect for that period of time.

##### SYSTEM AGREEMENTS

###### Compute Credit Limit Agreement

This agreement determines how the Credit Limit check will be performed, including which Smart Agreement will need to be run when performing a check of a credit limit.

###### Compute Transaction Fee Agreement

This determines how Transaction Fees are calculated. Since Transaction Fees can accumulate before being submitted, the Compute Transaction Fee Agreement determines the threshold at which transactions fees must be paid before an agent can continue to transact. This is typically handled in an automated manner.

###### Fee Trigger

Displays the Fee Trigger Threshold. No transaction that makes fees accumulate beyond this threshold should process successfully until the outstanding fees have been paid.

###### Fee Percentage

Displays the Transaction Fee Percentage that is charged on all Spend Transactions.

##### ADDITIONAL INFORMATION

We have created a couple of additional experimental features to make it easier for Service Networks to create neutral territory for Smart Agreements and manage their interactions with other Service Networks within that Domino Alliance.

###### ADDITIONAL SPECIAL AGENTS

Special Agents are agents given some particular permissions or rights, similar to Roles in a Smart Agreement, but declared at the level of the Domino Alliance for the duration of that Global Config Window.

###### ADDITIONAL SPECIAL SMART AGREEMENTS (Not yet implemented in UI)

In case it may prove fruitful for one or more Service Networks to set up Smart Agreement defined at the Alliance level, that functionality can be supported here.

#### SERVICE UNITS

This is where we can create the different Units that this particular Domino Alliance will use. Each type of Unit will have a Symbol, and Title, and a Description. Each Unit is able to represented by a floating point number (i.e something like 3.14).

##### FUEL CREDITS

The first Service Unit that is declared serves as the FUEL CREDIT UNIT for this Domino Alliance. It can be edited by the Global Admin. This will serve as the main payment unit.

##### NON-FUEL SERVICE UNITS

Subsequent Service Units are quantifiable (i.e. countable) units related to different forms of service provision. These should be defined in ways that fit with the kinds of services being provided (and tracked) within the Service Networks participating in this Domino Alliance. A cloud computing Service Network might track Bandwidth, Processing, and Requests. A storage network would certainly track Storage. A wifi hotspot Service Network might track connections and authorization for access to wifi service, perhaps on a time basis (5 minute intervals) with additional tracking of Bandwidth provided.

In general, when a customer pays for Services from a Service Network, they will pay with FUEL CREDITS, and will receive Services. When they do, the accounting alliance generally accounts for the specific services that have been provided on their behalf by showing that they have received those Service Units.

#### Service Networks

Service Networks are the different participating member projects that form the backbone of a particular Domino Accounting Alliance. At present, many of these will be Decentralized Physical Infrastructure projects like Holo Hosting that are creating a two-sided marketplace with Service Providers and purchasers of those services and ways to track that provision of services. 

##### INITIALIZE SERVICE NETWORK

The Global Config Admin is able to set up a new Service Network. Initializing a Service Network involves creating the initial Service Network Configuration.  

##### SERVICE NETWORK CONFIG

Similar to the Global Config, a Service Network Config is the ruleset that governs the operation of a particular Service Network within a Domino Alliance. This includes the definitions, properties, smart agreements and authorizations that govern that Service Network's operations within Domino. The Service Network Config includes the following:

##### BASIC PROPERTIES

###### NAME

The Name of the Service Network

###### ABBREVIATION

The Abbreviation of the Service Network Name

###### URL

URL for the Service Network

###### OUTSIDE CURRENCY NAME

Name of the Service Networks Outside Currency (if any).

###### OUTSIDE CURRENCY SYMBOL

Symbol of the Service Network's Outside Currency.

###### SERVICE NETWORK ADMINS (Previously "Lane Editors". UI update in progress)

Agents authorized to serve as Service Network Admins can make changes to this Service Network Configuration.

##### SERVICE NETWORK CONFIG WINDOW

Similar to the Global Configuration, a Service Network Config is effective for a specific period of time and will be replaced at the end of that period with either a new Service Network Configuration (i.e. one or more changes have been made) or a continuation of that same configuration for an addition period of time.

###### EFFECTIVE START DATE

Date and Time where this Service Network Configuration Window becomes effective. During the period it is in effect, any interactions with Service Network Agreements must adhere to this configuration.

###### EXPIRATION DATE

Date and Time that this particular Service Network Configuration Window ceases to be in force.

##### SPECIAL AGENTS

A Service Network can define a number of special agents. These can be added to people's address book so that they can confirm that they are interacting with the officially identified parties playing those particular roles.

###### OPS ACCOUNTS

An operations account that can take actions on behalf of that Service Network. For instance, might pay employees a portion of their salary in Fuel Credits.

###### SERVICE INFRASTRUCTURE ACCOUNT

Each Domino Accounting Alliance sets its own policies regarding how participating Service Networks are able to establish credit within the Alliance. Access to an interest-free credit line can be a significant benefit for participating Service Networks.

This access to credit may enable participating Service Networks to capitalize infrastructure or other investments using the internal credit system of the Domino Alliance rather than having to resort to outside sources such as obtaining a bank loan (where they would have to pay principle plus interest) or selling a portion of the Service Network's equity (which would dilute existing shareholders, and depending on circumstances can ultimately result in a loss of control of their company).

In order for a Service Network to receive a line of credit, they need to assign an Agent (a public key) as the steward of their credit line. We call this their Service Infrastructure Account. As the Service Network establishes a credit limit in accordance with that Domino Alliance's policies, the Service Infrastructure Account is the agent to whom that credit limit gets allocated. This agent can then spend up to that credit limit.

###### SALES AGENT

Service Networks that have an Outside Currency and want to offer a way for holders of that Outside Currency to purchase Fuel Credits, need to assign an agent that is authorized to perform such sales on behalf of the Service Network. The Sales Agent is responsible for obtaining the proof of deposit in the Outside Currency, whether that proof is through a Blockchain Transaction, a Bank Transaction or in some other manner, and serve as Executor of the Transaction where that External Proof is being provided and Fuel Credits (initially owned by that Service Network) are being transferred to the Purchaser.

By default, this same Sales Agent plays the same role when the reverse course of action is being followed. A Service Provider who has earned Fuel Credits through work in that Service Network can Redeem those Fuel Credits for that Service Network's Outside Currency. During this Redemption process, the Sales Agent similarly Executes the transaction where the Fuel Credits are being received by the Service Network and authorizes the payment out to the Redeemer through the appropriate method in the relevant Outside Currency.

##### SMART AGREEMENTS

Service Networks will generally make use of 3 different primary forms of Smart Agreements:
a Purchase Agreement, a Redemption Agreement and a Proof of Service Agreement.

To add any of these Smart Agreements, first create the specific Smart Agreement in the Library and then copy the Agreement ID and paste it into the field for the relevant agreement.  There are some examples already published in the library that you can Clone and modify to meet your Service Network's needs.

###### PURCHASE OF UNITS

This is the agreement governing the Purchase of Fuel Credits by Purchasers from the Service Network. They pay with a pre-defined Outside Currency according to the pricing algorithm defined by the Domino Alliance. The purchase must be Executed by the Service Network's Sales Agent.

###### REDEMPTION OF UNITS

This is the agreement governing the Redemption of Fuel Credits by Service Providers to the Service Network. Sales and Redemptions are really a means of enabling parties to "Purchase From" and "Redeem To" that particular Service Network's Fuel Credit Treasury within the Domino Alliance. Redeemers receive from the Service Network the pre-defined Outside Currency according to the pricing at which previous Sales of Fuel Credits had occurred. The Redemption must be Executed by the Service Network's Sales Agent.

After each Sale or Redemption transaction (which the Sales Agent is always participating in), the Current Redemption Rate gets recalculated - based on a simple Redemption Rate algorithm that takes in the number and price of all Fuel Credits sold and subtracts the number and price of all Fuel Credits already redeemed. Thus the Current Redemption Rate is simply the Average Sales Price of Fuel Credits that have been sold by the Service Network, but not yet Redeemed.

###### PROOF OF SERVICE

This is the agreement governing the methods by which Service Providers (or something like a Service Network Invoice Gateway) proves that particular agents have actually Provided Services as part of that Service Network. It also supports agents that want to Pay for Services to do so through a Spender Interaction with this agreement.

In addition to being useful as a part of the integrity process that ensures that those paying for the services are paying for actual value creation, Proofs of Service are helpful in establishing the credibility, and consequent credit limits of particular Service Providers.

Finally, each Service Network will themselves support Redemptions to an Outside Currency only to Service Providers, and only up to a quantity of Fuel Credits equivalent to what has been earned through Service Provision as established through Proofs of Service.  

This principle of Redemption support only for Service Providers helps ensure that any arbitrage type activity within the Domino Alliance itself is mediated by actual Service Provision, and thus is tied to real Value Creation, as defined by the Alliance's governing rules. This helps ensure that the Fuel Credits are backed by the demonstrated capacity to perform valued work on behalf of others who are willing to pay for it.

##### VIEW SERVICE NETWORK CONFIG

In the Service Networks section of the Admin tab, the View Config button (Previously "Details" button. UI update in progress.) will open the Service Network Config modal.

##### SERVICE NETWORK CONSOLE

Once a Service Network has been initialized by the Global Progenitor, the Authorized Service Network Admin(s) are able to update that Service Network Configuration using the Service Network Console. This can be accessed by clicking on the Update Properties button with the Service Networks section of the Admin tab.

### SERVICE NETWORKS (the tab)

The Service Networks Tab only appears once one or more Service Networks have been added by the Global Admin. It is there as a convenience to give a single place to find all of the relevant Smart Agreements, Data Records, and Calculated Redemption Rates for each Service Network.

Each Service Network that is participating as a member in the Domino Alliance has their own area within the Service Networks tab.

For any particular Service Network, the following pieces of content would typically be made available for interaction:

- Smart Agreement for Purchase of the Fuel Credit
- Smart Agreement for Redemption of Fuel Credits
- Smart Agreement for Proof of Service

Each of those Smart Agreements is also available in the Library, but this page assembles them alongside the below bits of information for a better user experience.

Means for Sales Agent to Add a conversion sheet that governs the pricing of the Sale of Fuel Credits for Outside Currency.

Means to Drop off Proofs of Deposit for Payments of Outside Currency used by others to purchase Fuel Credits.

Means for Sales Agent (currently read Redemption Agent) to Redeem Fuel Credits for Outside Currency.

In addition there is an informative "Current Redemption Rate" which is calculated based on the Average Sales Price (in Outside Currency) of all unredeemed Fuel Credits that had previously been sold by the Service Network for Outside Currency.

We have populated the Service Networks tab with example Agreements and other content for Holo Hosting related to purchasing, redeeming Fuel Units (Holo Fuel) using a particular external Blockchain token: HOT (an ERC20 token); an example Proof of Service agreement, Sales Agent Depositing Purchases of Holo (with HOT), which includes an ability to create a conversion sheet - which determines the price at which Holo Fuel (the internal Fuel Unit) is being offered for HOT (the external token), A means by with the Sales and Redemption Agent can Execute  Redeemed for Holo Token, the Current Redemption Rate (which is based on the average purchase conversion rate of the available external tokens (HOT) available for redemption.

And that is a wrap of the walk through of Domino.

If you have any thoughts, questions or criticisms, please feel free to add items to our [feedback board](https://github.com/orgs/unytco/projects/5/views/1), and don't hesitate to reach out to our team.
