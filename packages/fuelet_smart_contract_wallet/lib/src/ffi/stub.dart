import 'package:fuelet_smart_contract_wallet/src/bridge_generated.dart';

/// Represents the external library for fuelet_smart_contract_wallet
///
/// Will be a DynamicLibrary for dart:io or WasmModule for dart:html
typedef ExternalLibrary = Object;

FueletSmartContractWallet createWrapperImpl(ExternalLibrary lib) =>
    throw UnimplementedError();
