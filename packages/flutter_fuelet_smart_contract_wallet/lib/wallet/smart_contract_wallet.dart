import 'package:flutter_fuelet_smart_contract_wallet/wallet/wrapped_smart_contract_wallet.dart';

import 'platform_impl/stub_wallet.dart'
    if (dart.library.io) 'platform_impl/mobile_wallet.dart'
    if (dart.library.html) 'platform_impl/web_wallet.dart';

class SmartContractWallet {
  final WrappedSmartContractWallet _wrappedWallet;

  SmartContractWallet(this._wrappedWallet);

  static final _platformWallet = PlatformSmartContractWalletImpl();

  static Future<SmartContractWallet> connect(
      {required String networkUrl,
      required String r1PublicKey,
      required String recoveryWalletPrivateKey}) async {
    final wrappedWallet = await _platformWallet.connect(
        networkUrl: networkUrl,
        r1PublicKey: r1PublicKey,
        recoveryWalletPrivateKey: recoveryWalletPrivateKey);
    return SmartContractWallet(wrappedWallet);
  }

  String get predicateBech32Address => _wrappedWallet.predicateBech32Address;

  String get r1PublicKey => _wrappedWallet.r1PublicKey;

  Future<void> deployContract() {
    return _wrappedWallet.deployContract();
  }

  Future<(String, String)> createTransferTxRequest({
    required String destinationB256Address,
    required int fractionalAmount,
    required String assetId,
  }) {
    return _wrappedWallet.createTransferTxRequest(
        destinationB256Address: destinationB256Address,
        fractionalAmount: fractionalAmount,
        assetId: assetId);
  }

  Future<String> sendTransaction(
      {required String transactionRequestHexOrJson,
      required String r1Signature}) {
    return _wrappedWallet.sendTransaction(
        transactionRequestHexOrJson: transactionRequestHexOrJson,
        r1Signature: r1Signature);
  }
}
