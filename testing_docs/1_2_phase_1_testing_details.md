
# Phase 1 Testing Details

#### links to related docs

- [Test Plan](./testing_docs/1_0_testing_plan.md)
- [Unyt Setup](../README.md)
- [Testing Documentation, Phase 4](./4_0_phase_4_testing_details.md)
- [Unyt Dictionary](./4_2_unyt-dictionary.md)
- [Intro to Smart Agreements (Three Layers)](./4_1_intro_to_smart_agreements.md)
- [Templates and Smart Agreements Library Repo](https://github.com/unytco/smart_agreement_library)
- [Feedback](https://github.com/orgs/unytco/projects/5/views/1)


**WARNING: This doc contains details!** 

Feel free to use the outline view available in the upper right to get an overview of the different content covered.

### Phase 1 Testing Orientation

In Phase 1, we are testing version 0.2.4 of Domino

Domino is software that runs locally on your computer and connects with other instances to form a peer-to-peer network.

In the future, there will likely be many different networks running their own version of this software for their community.

During Phase 1 Testing, however, all testers will be joining a single peer network and will be able to send and receive transactions from anyone else in the test network.


For this Phase of testing, you will not need to create a password to log in to your copy of the Domino. If you lose your device or delete the software (and your private key), you will lose the ability act as that agent (including the ability to control any test currency that has been sent to you account). (Local) Password login is a feature that we can incorporate in a later release.

Once you have opened Domino, you can test out different types of transactions. To start with, in this version, each agent will have a credit limit of 100 units.

All of you will be using the software as a super user with the ability to not only engage in transactions, but to create new RAVEs (smart contract like functionality). Any Code Template, Executable Agreement or RAVE that you create will be visible to other users. 

Clicking on your identicon (on the right side) will copy your public key to the clipboard. You can then share that with others through whatever other medium you have available. 

They will enter it on their own copy of Domino when doing things like sending you an invoice (a request for payment) or a promise (a sending of payment) or when choosing you to serve as the executor of a RAVE.

Note that this version of Domino doesn't yet have push notifications set up. So for the time being, you may need to hit reload in order to see that someone has sent you a transaction to Pay or Execute or Accept.

If nobody else on your team is a available to test alongside you yet, feel free to try out some transactions with some of our team. 

Matt's public key is:

```
uhCAk6pXOgrY4sP8-VEzkGKb3NpiKyLSJwmZktYrTrwnqC1OPM0Po
```

Jarod's public key is:
```
uhCAk13kC865NTyZvRfRwLoyfucZpCCZkc3My8qwNXvBuXUYIiPRm
```
 
And feel free to reach out to use through our shared comms channel on Telegram or Signal. 
 
We are more than happy to jump on a video conference call to play alongside you for a bit.
 
On to the details!

### Transactions
In Domino, there are two main categories of transactions:
1) Direct Mutual Credit Transactions
2) RAVE Transactions

### Overview of Transaction Types
**Direct Mutual Credit Transactions** involve the sending of credits from one agent directly to another agent. They involve changes to the source chain history of the spender and the receiver. A selection of other agents independently validate each step in this process.

**RAVE (Recorded Agreement of Verifiable Execution) Transactions** are similar to blockchain based smart contracts, but are designed to make use of Holochain's agent centric architecture. They involve the verifiable execution of an agreement by an executor. RAVEs make changes to the source chain history of an executor, and potentially of a sender and a receiver. A selection of other agents independently validate each step in this process. RAVE transactions allow more complicated logic to be made use of in structuring transactions and the conditions under which those transactions will be processed.

#### Direct Mutual Credit Transactions
A Process Overview

Direct transactions involve a spender and a receiver and can be initiated in two different ways:

1) A Promise
2) An Invoice

You can find each of these as Transaction Type options when Creating a Transaction under Transactions > MC Tx

##### 1) Promise
A sending of payment by the spender to a receiver.
    
The workflow is as follows:

**1) Sending with Promise**

Spender promises (sends) a specific amount to a specific receiver. As soon as the Spender clicks "Create a Promise" the funds are immediately debited from their account balance along with any associated transaction fees.

**2) The Between Times (Before Acceptance)**

Spender will see the transaction as a "Completed Transaction" of the type "promise". At this point, Receiver will see the transaction as an "Actionable Transaction" of the type "Promise" and will be shown the details and an option to Accept the payment.

**3) Acceptance**

Receiver clicks accept. The funds are immediately credited to their account.

**Summary**

This form of payment is a multi-step process with validation happening behind the scenes for both the making of the promise and the acceptance of the promise. 

Reminder: In Holochain, individual agents each have their own activity history (source chain) and they alone authorize state changes to their own source chain. Each agent is responsible for checking the validity of the action they are taking, and each action is also independently validated by other devices on the the network. Agents are able to validly spend amounts (including fees) either to zero, or if they have a credit limit, going into the negative up to their credit limit.
    
##### 2) Invoice
A request for payment, made by the receiver. 

A receiver can request a payment by sending an invoice. They will indicate the Spender and the Amount Requested.

The workflow is as follows:

**1) Invoice Request**

An agent that is seeking to receive funds can send a request to pay in the form of an invoice to the party that they want to pay them. They enter the amount and the public key of the agent that they are requesting payment from. They can also add a note.

**2) The Between Times (Before Invoice is Paid)**

After an invoice is sent, the receiver will see it in their Pending Transactions as an Invoice. The spender will see it in their Actionable Transactions as an Invoice and will have a button to Pay the invoice on the right hand side. (Spender may need to refresh first). At this point, only a request has been made and no funds have been debited or credited to either account. 
    
**3) Paying an Invoice**

The process that follows from this point forward is identical to the Promise workflow, as the Spender is now making a Promise (though the details had initially been proposed by the receiver).
As soon as the Spender clicks "Pay" the funds are immediately debited from their account balance along with any associated transaction fees. 

**4) The Between Times (Before Acceptance)**

Spender will see the transaction as a "Completed Transaction" of the type "promise". At this point, Receiver will see the transaction as an "Actionable Transaction" of the type "Promise" and will be shown the details and an option to Accept the payment.


**5) Acceptance**

Receiver clicks accept. The funds are immediately credited to their account. Receiver will now see the transaction as a Completed Transaction of the type "Accepts".

**Summary**

An invoice transaction involves the same underlying payment workflow as a promise transaction, but enables the Receiver to initialize the request.


### Making a Direct Transaction 
A step-by-step walk through.

The below Direct Transactions will only work if the sender has available credit in their account. For testing with Domino 0.2.0. we have set it up so that any new agent starts with a credit limit of 100. 


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
RAVEs, or Recorded Agreements of Verifiable Execution are similar to Blockchain based Smart Contracts, but are designed to enable automated agreement execution in agent-based Holochain applications.

Like computer programs more generally, a RAVE can be created to automate the execution of a wide range of activities. 

A RAVE is executed by an Executor. Select other agents in the network validate that the execution was done properly and are relied upon by other agents for their assessment of validity. However, any other agent can independently validate that the execution was done properly. 

A RAVE is an Instantiation of an Executable Agreement. An Executable Agreement must conform to the Code Template that it is based on.

A RAVE includes 
1) a set of Inputs - from the source(s) specified in the Executable Agreement 
2) execution code
3) a set of Outputs
In addition, the specific agent that Executes the RAVE must satisfy the Executor criteria from the revelant Executable Agreement, whether that:
  a) requires a specific agent to serve as Executor, 
  b) requires the Executor to be one of set of specific agents or
  c) enables any agent to have been selected as Executor.

#### The existing Executable Agreements

*UI Bug note: In the RAVE Templates tab, there are missing dropdown icons.  Clicking on the whitespace in an existing Code Template row will show Executable Agreements that have already been created using that code template. These can be used to initiate a RAVE.*


There are a couple of RAVEs that have been set up already:
1) __system_credit_limit_computation
 
2) conditional_forward

System_credit_limit_computation is a system wide RAVE that sets initial credit limits for different agents:
    - a credit limit of 10,000 for an infrastructure agent
    - a credit limit of 100 for all other agents
    
The conditional_forward enables as spender to park credits on that RAVE. In this simple version, there is no way for those credits to be returned to the spender. However, the credits are also not in the custody of the Executor. 

The Executor has the ability to 

a) execute the transaction and release the funds for the Receiver to Accept, or 

b) not execute the transaction (presumably because some condition has not yet been met). This keeps the credits in limbo. The executor does not have an ability to use them or direct them elsewhere.

A more sophisticated conditional forward could have logic for returning the credits to the spender and possibly other input sources, for instance data from a pre-determined oracle indicating that some specific condition has in fact been met.

#### Create a RAVE using an Executable Agreement

Feel free to initalize a conditional_forward RAVE. 

Under RAVE Library > Code Templates Library, click on the row for conditional_forward to see the Executable Agreement(s) that have been created from that template. Pick one and click Initialize to begin structuring a transaction using the agents and amounts of your choice.

#### Create an Executable Agreement from a Code Template
If you haven't already, you can create you own Executable Agreement from one of the Code Templates. Maybe create an alternate version of the conditional_forward. (it's pretty minimal). 


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

Once you have created an Executable Agreement, click Initialize to start creating a RAVE that conforms to that Agreement. Once initialized, the agents listed in the agreement will get notified that they have Actionable Transactions when it is their turn to take some action. (again, this may require a reload to show up.)

If you are ready, head next to [Phase 2 Testing Details](./1_1_phase_2_testing_details.md).

If you have any thoughts, questions or criticisms, please feel free to add items to our [feedback board](https://github.com/orgs/unytco/projects/5/views/1), and don't hesitate to reach out to our team.