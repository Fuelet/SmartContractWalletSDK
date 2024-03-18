import 'package:test/test.dart';

import '../test_utils.dart';

void main() {
  test('predicate address generation', () async {
    final smartContractWallet =
        await getSmartContractWallet(stubR1PublicKey, testWalletPrivateKey);
    expect(smartContractWallet.bech32Address,
        "fuel1ec0d4dgpp8clz3pvpglas8q9sh5z9aflrgcjx3pd6xldvae2nvcsm99p8t");
  });

  test('tx request creation', () async {
    final smartContractWallet =
        await getSmartContractWallet(stubR1PublicKey, testWalletPrivateKey);
    final (tx, txId) = await smartContractWallet.genTransferTxRequest(
        toB256: testDestinationB256, amount: 1, asset: ethAsset);
    expect(txId.isEmpty, false);
    expect(tx.isEmpty, false);
  }, skip: 'Should be run manually (need coins)');

  test('deploy contract', () async {
    final smartContractWallet =
        await getSmartContractWallet(stubR1PublicKey, testWalletPrivateKey);
    // will fail if the contract is already deployed
    await smartContractWallet.deployContract();
  }, skip: 'Should be run manually');
}
