import 'dart:typed_data';

import 'package:convert/convert.dart';
import 'package:flutter_fuelet_smart_contract_wallet/ffi/mobile_wrapper.dart';
import 'package:flutter_fuelet_smart_contract_wallet/wallet/smart_contract_wallet.dart';
import 'package:flutter_fuelet_smart_contract_wallet/wallet/wrapped_smart_contract_wallet.dart';
import 'package:fuelet_smart_contract_wallet/fuelet_smart_contract_wallet.dart';

import 'base_wallet.dart';

class PlatformSmartContractWalletImpl extends BaseWallet {
  static final FueletSmartContractWallet _bridge = createLib();

  @override
  Future<WrappedSmartContractWallet> connect(
      {required String networkUrl,
      required String r1PublicKey,
      required String recoveryWalletPrivateKey}) async {
    final rustSmartContractWallet = await SmartContractWallet.connect(
        bridge: _bridge,
        r1PublicKey: r1PublicKey,
        recoveryPrivateKey: recoveryWalletPrivateKey,
        nodeUrl: networkUrl);
    return MobileSmartContractWallet(
        rustSmartContractWallet: rustSmartContractWallet,
        predicateBech32Address: rustSmartContractWallet.bech32_address,
        r1PublicKey: rustSmartContractWallet.r1_public_key);
  }
}

class MobileSmartContractWallet extends WrappedSmartContractWallet {
  final SmartContractWallet _rustSmartContractWallet;
  @override
  final String predicateBech32Address;
  @override
  final String r1PublicKey;

  MobileSmartContractWallet(
      {required SmartContractWallet rustSmartContractWallet,
      required this.predicateBech32Address,
      required this.r1PublicKey})
      : _rustSmartContractWallet = rustSmartContractWallet;

  @override
  Future<void> deployContract() {
    return _rustSmartContractWallet.deployContract();
  }

  @override
  Future<(String, String)> createTransferTxRequest(
      {required String destinationB256Address,
      required int fractionalAmount,
      required String assetId}) async {
    final (txBytes, txIdBytes) =
        await _rustSmartContractWallet.gen_transfer_tx_request(
            destinationB256Address, fractionalAmount, assetId);
    return (hex.encode(txBytes), hex.encode(txIdBytes));
  }

  @override
  Future<String> sendTransaction(
      {required String transactionRequestHexOrJson,
      required String r1Signature}) {
    final txBytes = hex.decode(transactionRequestHexOrJson);
    final signatureBytes = hex.decode(r1Signature);
    return _rustSmartContractWallet.sendTransaction(
        encodedTx: Uint8List.fromList(txBytes), signature: signatureBytes);
  }
}
