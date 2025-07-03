# shipyard-domino

Update the tauri.config.json

- identifier
- version
- product name

run this to setup the android project:
https://darksoil.studio/p2p-shipyard/guides/android/project-setup.html

you need to run `yarn tauri icon <PATH_TO_THE_ICON_PNG_FILE>` before running those commands

## Test development

Enter `nix develop`

`make install`

in one terminal run (This will test the desktop build)
`make launch`

in terminal two run (This will setup the android build)
There are presetup steps for android that you can read about [here](https://darksoil.studio/p2p-shipyard/guides/android/device-setup.html)
enter `nix develop .#androidDev`
then
`make launch-android`

## Prep for release

set the network seed `NETWORK_SEED`
