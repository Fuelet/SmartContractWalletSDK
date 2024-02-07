import 'dart:ffi';
import 'dart:io';

import 'package:flutter_fuelet_smart_contract_wallet/flutter_fuelet_smart_contract_wallet_bindings_generated.dart';
import 'package:fuelet_smart_contract_wallet/fuelet_smart_contract_wallet.dart';

const String _base = 'fuelet_smart_contract_wallet';
const String _flutterLibName = 'flutter_$_base';

/// The dynamic library in which the symbols for [FlutterFueletSmartContractWalletBindings] can be found.
final DynamicLibrary _dylib = () {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.open('$_flutterLibName.framework/$_flutterLibName');
  }
  if (Platform.isAndroid || Platform.isLinux) {
    return DynamicLibrary.open('lib$_base.so');
  }
  if (Platform.isWindows) {
    return DynamicLibrary.open('$_base.dll');
  }
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

FueletSmartContractWallet createLib() => createWrapper(_dylib);
