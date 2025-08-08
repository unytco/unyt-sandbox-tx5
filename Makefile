install:
	( [ -d  node_modules ] || yarn install )
	cd domino && ( [ -d  node_modules ] || yarn install )

setup: 
	git submodule update --init
	cd domino && git submodule update --init

launch:
	cd domino && yarn build:happ
	mkdir -p workdir
	cp -r domino/workdir/domino.happ workdir/domino.happ
	yarn network:tauri

launch-android: install
	yarn launch:android

package:
	cd domino && APP_VERSION=$$(jq -r '.version' ./src-tauri/tauri.conf.json) make package

build-android: install
	yarn tauri android build