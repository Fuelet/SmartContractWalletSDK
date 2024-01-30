import 'dart:ffi';

import 'package:fuelet_smart_contract_wallet/src/bridge_generated.dart';

typedef ExternalLibrary = DynamicLibrary;

FueletSmartContractWallet createWrapperImpl(ExternalLibrary dylib) =>
    FueletSmartContractWalletImpl(dylib);
