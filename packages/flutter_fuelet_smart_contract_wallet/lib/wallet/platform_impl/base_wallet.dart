import 'package:flutter_fuelet_smart_contract_wallet/wallet/wrapped_smart_contract_wallet.dart';

abstract class BaseWallet {
  Future<WrappedSmartContractWallet> connect(
      {required String networkUrl,
      required String r1PublicKey,
      required String recoveryWalletPrivateKey});
}
