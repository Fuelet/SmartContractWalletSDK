import 'dart:ffi';
import 'dart:io';

import 'package:flutter_fuelet_smart_contract_wallet/flutter_fuelet_smart_contract_wallet_bindings_generated.dart';
import 'package:fuelet_smart_contract_wallet/fuelet_smart_contract_wallet.dart';

const String _libName = 'flutter_fuelet_smart_contract_wallet';

/// The dynamic library in which the symbols for [FlutterFueletSmartContractWalletBindings] can be found.
final DynamicLibrary _dylib = () {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.open('$_libName.framework/$_libName');
  }
  if (Platform.isAndroid || Platform.isLinux) {
    return DynamicLibrary.open('lib$_libName.so');
  }
  if (Platform.isWindows) {
    return DynamicLibrary.open('$_libName.dll');
  }
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

FueletSmartContractWallet createLib() => createWrapper(_dylib);
