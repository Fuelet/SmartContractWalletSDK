# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(flutter_fuelet_smart_contract_wallet_library VERSION 0.0.1 LANGUAGES C)

add_library(flutter_fuelet_smart_contract_wallet SHARED
  "flutter_fuelet_smart_contract_wallet.c"
)

set_target_properties(flutter_fuelet_smart_contract_wallet PROPERTIES
  PUBLIC_HEADER flutter_fuelet_smart_contract_wallet.h
  OUTPUT_NAME "flutter_fuelet_smart_contract_wallet"
)

target_compile_definitions(flutter_fuelet_smart_contract_wallet PUBLIC DART_SHARED_LIB)
