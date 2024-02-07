import 'package:flutter_fuelet_smart_contract_wallet/wallet/wrapped_smart_contract_wallet.dart';

import 'base_wallet.dart';

class PlatformSmartContractWalletImpl extends BaseWallet {
  @override
  Future<WrappedSmartContractWallet> connect(
      {required String networkUrl,
      required String r1PublicKey,
      required String recoveryWalletPrivateKey}) {
    throw Exception('Not implemented yet');
  }
}
