import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:fuelet_smart_contract_wallet/src/bridge_generated.dart';

const testnetApiUrl = 'https://testnet.fuel.network';
const testDestinationB256 =
    '0xfe1177573a47310756f27ef25c7229b5da8d5109a7258b297f23a6768c78cb40';
const testWalletPrivateKey =
    'e5e05a4ab2919dc01b97c90a48853fd4dfbd204e92e44327375702ab09bb184e';
const stubR1PublicKey =
    '0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000';
const ethAsset =
    '0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07';

String projectPath = Directory.current.parent.parent.path;
var rustSdk = FueletSmartContractWalletImpl(loadLibForFlutter(
    '$projectPath/target/debug/libfuelet_smart_contract_wallet.dylib'));

Future<SmartContractWallet> getSmartContractWallet(
    String r1PublicKey, String recoveryPrivateKey) {
  return SmartContractWallet.connect(
      bridge: rustSdk,
      r1PublicKey: r1PublicKey,
      recoveryPrivateKey: recoveryPrivateKey,
      nodeUrl: testnetApiUrl);
}
