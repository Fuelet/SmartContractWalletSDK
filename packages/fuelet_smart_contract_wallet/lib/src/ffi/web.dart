import 'package:fuelet_smart_contract_wallet/src/bridge_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

typedef ExternalLibrary = WasmModule;

FueletSmartContractWallet createWrapperImpl(ExternalLibrary module) =>
    FueletSmartContractWalletImpl.wasm(module);
