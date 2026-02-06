MAC_HOST = rileyseaburg@192.168.50.248
PROJECT_DIR = ~/actions-runner-ssh/_work/ssh-terminal/ssh-terminal
TEAM_ID = J9YRM3U37D

KEYCHAIN_PASS ?= $(error Set KEYCHAIN_PASS, e.g.: make ios-dev KEYCHAIN_PASS=mypass)

.PHONY: ios-dev ios-build ios-install mac-pull mac-logs clean

## Run tauri iOS dev on device (live reload)
ios-dev:
	ssh -t $(MAC_HOST) "security unlock-keychain -p '$(KEYCHAIN_PASS)' ~/Library/Keychains/login.keychain-db; cd $(PROJECT_DIR) && APPLE_DEVELOPMENT_TEAM=$(TEAM_ID) cargo tauri ios dev"

## Build iOS release IPA
ios-build:
	ssh -t $(MAC_HOST) "cd $(PROJECT_DIR) && APPLE_DEVELOPMENT_TEAM=$(TEAM_ID) cargo tauri ios build --target aarch64"

## Install latest IPA to device
ios-install:
	ssh $(MAC_HOST) "xcrun devicectl device install app --device 4745B27B-2B68-5164-A053-B869B07EE2CA '$(PROJECT_DIR)/src-tauri/gen/apple/build/arm64/SSH Terminal.ipa'"

## Pull latest code on Mac
mac-pull:
	ssh $(MAC_HOST) "cd $(PROJECT_DIR) && git pull"

## Tail Xcode device logs
mac-logs:
	ssh -t $(MAC_HOST) "xcrun devicectl device process logstream --device 4745B27B-2B68-5164-A053-B869B07EE2CA --filter 'subsystem == \"com.sshterminal.app\"'"

## Clean build artifacts on Mac
clean:
	ssh $(MAC_HOST) "cd $(PROJECT_DIR)/src-tauri && cargo clean"
