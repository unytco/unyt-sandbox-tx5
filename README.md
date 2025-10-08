# Unyt Tx5 Releases

![GitHub release (latest by date)](https://img.shields.io/github/v/release/unytco/unyt-sandbox-tx5?style=for-the-badge)
![GitHub All Releases](https://img.shields.io/github/downloads/unytco/unyt-sandbox-tx5/total?style=for-the-badge)

## Related docs

- [Invite to Play](./testing_docs/1_0_invite.md)
- [Unyt Setup](./README.md)
- [Detailed Documentation](./testing_docs/5_0_phase_5_testing_details.md)
- [Unyt Dictionary](./testing_docs/4_2_unyt-dictionary.md)
- [The Smart Agreement Overview](./testing_docs/5_0_Smart_Agreement_Release.md)
- [Intro to Smart Agreements (Three Layers)](./testing_docs/4_1_intro_to_smart_agreements.md)
- [Templates and Smart Agreements Library Repo](https://github.com/unytco/smart_agreement_library)
- [Feedback](https://github.com/orgs/unytco/projects/5/views/1)


## Intro

Unyt is a Holochain based application for creating agent-centric, peer-to-peer, Mutual Credit accounting systems with smart contract like functionality.

We are working with potential partner projects like yours as we build out this software to ensure that it meets the needs of your team as well as your community of users.

## Invite to Play

We [invite you to play](./testing_docs/1_0_testing_plan.md) alongside others in kicking the tires on Unyt and playing with Smart Agreements.

Details on how to join the conversation about Unyt in the Holochain DEV.HC Discord is also in that invite page.

## Downloads Zero Arc releases

<div align="center">

<table>
<tr>
<td width="25%" align="center">

### **Windows**

---

[MSI Installer (x64)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_zero-arc_0.40.0_x64_windows.msi)

[EXE Setup (x64)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_zero-arc_0.40.0_x64_windows.exe)

</td>
<td width="25%" align="center">

### **MacOS**

---

[Silicon (arm64)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_zero-arc_0.40.0_aarch64_darwin.dmg)

[Intel (x64)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_zero-arc_0.40.0_x64_darwin.dmg)

</td>
<td width="25%" align="center">

### **Linux**

---

[AppImage](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_zero-arc_0.40.0_amd64_linux.AppImage)

[Debian (.deb)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_zero-arc_0.40.0_amd64_linux.deb)

</td>
<!-- 
<td width="25%" align="center">

### **Android**

---

[APK](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.0.1/app-universal-release.apk)

[AAB Bundle](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.0.1/app-universal-release.aab)

</td> 
-->
</tr>
</table>

</div>

## Download Full Arc releases

<div align="center">

<table>
<tr>
<td width="25%" align="center">

### **Windows**

---

[MSI Installer (x64)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_0.40.0_x64_windows.msi)

[EXE Setup (x64)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_0.40.0_x64_windows.exe)

</td>
<td width="25%" align="center">

### **MacOS**

---

[Silicon (arm64)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_0.40.0_aarch64_darwin.dmg)

[Intel (x64)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_0.40.0_x64_darwin.dmg)

</td>
<td width="25%" align="center">

### **Linux**

---

[AppImage](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_0.40.0_amd64_linux.AppImage)

[Debian (.deb)](https://github.com/unytco/unyt-sandbox-tx5/releases/download/v0.40.0/Unyt-tx5_0.40.0_amd64_linux.deb)

</tr>
</table>
</div>

All available versions can be found in the [Releases](https://github.com/unytco/unyt-sandbox-tx5/releases/)

Once installed, the Unyt software will run locally on your device and connect with others also running the software to operate as a peer-to-peer application.

## Setup

Note: In Mac, because you downloaded the software directly and not through Apple's App Store, you may need to open the System Settings and go to Privacy and Security, scroll down to Security and give Unyt permission to run.

If you want to delete everything and start over with a new account, check out [Starting Fresh](./testing_docs/starting_fresh.md)


## Intro
Unyt is a Holochain based application for creating p2p credit and payment systems with Smart Agreement functionality.

This release is focused on highlighting Unyt's approach to Smart Agreements.

Though they sound similar to Blockchain Smart Contracts, Unyt's Smart Agreements work a little differently and open up some new possibilities.

Smart Agreements are flexible, lightweight, and are anchored in the agency of each participant.

To dive into more details on Smart Agreements, check out:
- [Arthur Brock's Blog Post]()
- [The Smart Agreement Overview](./testing_docs/5_0_Smart_Agreement_Release.md)
- [Intro to Creating Smart Agreements](./testing_docs/4_1_intro_to_smart_agreements.md)
- [Smart Agreements Library](https://github.com/unytco/smart_agreement_library)
- [Unyt Dictionary](./testing_docs/4_2_unyt-dictionary.md)

When you open Unyt on your operating system for the first time, it will create a set of public and private keys for you that you can use to interact with others. These are stored in a private keystore (Lair) on your own machine and are used during future uses. In Unyt we often refer to this public key as "your address" as it is how others can refer to you when sending, receiving or authorizing you to perform particular roles.

[Share your address](https://forms.gle/sbCFUuv8sGyYhnc97) with others and find theirs as well in the [Address Spreadsheet](https://docs.google.com/spreadsheets/d/1gusOPtLVpT2RCDP7DRhVX39OEE-XAv26pnkPrwZbqzM/edit?gid=2043153663#gid=2043153663).

We [invite you to play](./testing_docs/1_0_testing_plan.md) alongside others.

## License

This project is licensed under the terms specified in the [LICENSE](LICENSE) file.

Copyright (C) 2024 - 2025, unyt.co
