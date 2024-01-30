#!/bin/bash

set -e

CURR_VERSION=fuelet_smart_contract_wallet-v`awk '/^version: /{print $2}' packages/fuelet_smart_contract_wallet/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_fuelet_smart_contract_wallet/ios/flutter_fuelet_smart_contract_wallet.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_fuelet_smart_contract_wallet/macos/flutter_fuelet_smart_contract_wallet.podspec
rm packages/flutter_fuelet_smart_contract_wallet/macos/*.bak packages/flutter_fuelet_smart_contract_wallet/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/flutter_fuelet_smart_contract_wallet/$CMAKE_PLATFORM/CMakeLists.txt
    rm packages/flutter_fuelet_smart_contract_wallet/$CMAKE_PLATFORM/*.bak
done

git add packages/flutter_fuelet_smart_contract_wallet/