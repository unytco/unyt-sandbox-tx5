# Signing required during setup

## 🔐 Create Apple Certificates for Code Signing

https://hackmd.io/@zo-el/SkARhyw9ke

## 🔐 Generating `TAURI_SIGNING_PRIVATE_KEY` for Tauri

https://hackmd.io/@zo-el/B1sHELKHxe

## 🔐 Android Signing Setup for Tauri

https://hackmd.io/@zo-el/S1vu7IFrxx

## When Going to the Apple App Store

**Important**
We would have to change the certificate we are using.

- Currently we use a `Developer ID Application` (For notrization)
- But when we go to the app store we need to use `Apple Distribution`
- This may need some changes to the tauri-config to know its for the appstore so it wont need to notarize
