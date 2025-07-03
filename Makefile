install:
	( [ -d  node_modules ] || yarn install )
	cd domino && ( [ -d  node_modules ] || yarn install )

setup: 
	git sudmodule update --init

launch:
	cd domino && make start

launch-android: install
	yarn launch:android

package:
	cd domino && make package

build-android: install
	yarn tauri android build