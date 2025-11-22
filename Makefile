APP_NAME = mug
BUNDLE_NAME = $(APP_NAME).app
TARGET_DIR = target/release
BINARY_PATH = $(TARGET_DIR)/$(APP_NAME)
BUNDLE_DIR = $(BUNDLE_NAME)/Contents
MACOS_DIR = $(BUNDLE_DIR)/MacOS
RESOURCES_DIR = $(BUNDLE_DIR)/Resources
INFO_PLIST = $(BUNDLE_DIR)/Info.plist


.PHONY: all macos-bundle install clean enable
enable:
	@mkdir -p ~/Library/LaunchAgents
	cp src/eu.baerlin.mug.plist ~/Library/LaunchAgents/eu.baerlin.mug.plist
	launchctl load ~/Library/LaunchAgents/eu.baerlin.mug.plist

all: macos-bundle

macos-bundle: $(BINARY_PATH)
	@echo "Creating macOS app bundle..."
	@rm -rf $(BUNDLE_NAME)
	@mkdir -p $(MACOS_DIR)
	@mkdir -p $(RESOURCES_DIR)
	@cp $(BINARY_PATH) $(MACOS_DIR)/$(APP_NAME)
	@cp assets/rocket.png $(RESOURCES_DIR)/rocket.png
	@echo '<?xml version="1.0" encoding="UTF-8"?>\
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">\
<plist version="1.0">\
<dict>\
	<key>CFBundleName</key>\
	<string>$(APP_NAME)</string>\
	<key>CFBundleExecutable</key>\
	<string>$(APP_NAME)</string>\
	<key>CFBundleIdentifier</key>\
	<string>com.example.$(APP_NAME)</string>\
	<key>CFBundleVersion</key>\
	<string>1.0</string>\
	<key>CFBundlePackageType</key>\
	<string>APPL</string>\
	<key>CFBundleSignature</key>\
	<string>????</string>\
	<key>CFBundleInfoDictionaryVersion</key>\
	<string>6.0</string>\
	<key>LSUIElement</key>\
	<true/>\
</dict>\
</plist>' > $(INFO_PLIST)
	@echo "Bundle created at $(BUNDLE_NAME)"

install: macos-bundle
	@echo "Installing $(BUNDLE_NAME) to ~/Applications..."
	@mkdir -p ~/Applications
	@cp -R $(BUNDLE_NAME) ~/Applications/
	@echo "Installed $(BUNDLE_NAME) to ~/Applications."

$(BINARY_PATH):
	cargo build --release

clean:
	@rm -rf $(BUNDLE_NAME)
	cargo clean
