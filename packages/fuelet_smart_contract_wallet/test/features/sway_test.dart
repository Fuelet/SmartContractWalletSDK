import 'package:test/test.dart';

import '../test_utils.dart';

void main() {
  test('predicate address generation', () async {
    final smartContractWallet =
        await getSmartContractWallet(stubR1PublicKey, testWalletPrivateKey);
    expect(smartContractWallet.bech32Address,
        "fuel1nnh77tj74f0sp6e39q7mmkpjwhnsqedmwq2ld2afmn4atf676wmq8wl7qy");
  });

  test('tx request creation', () async {
    final smartContractWallet =
        await getSmartContractWallet(stubR1PublicKey, testWalletPrivateKey);
    final (tx, txId) = await smartContractWallet.genTransferTxRequest(
        toB256: testDestinationB256, amount: 1, asset: ethAsset);
    expect(txId.isEmpty, false);
    expect(tx.isEmpty, false);
  });

  test('deploy contract', () async {
    final smartContractWallet =
        await getSmartContractWallet(stubR1PublicKey, testWalletPrivateKey);
    // will fail if the contract is already deployed
    await smartContractWallet.deployContract();
  }, skip: 'Should be run manually');
}
