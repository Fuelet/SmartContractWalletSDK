import 'package:test/test.dart';

import '../test_utils.dart';

void main() {
  test('predicate address generation', () async {
    final smartContractWallet =
        await getSmartContractWallet(stubR1PublicKey, testWalletPrivateKey);
    expect(smartContractWallet.bech32Address,
        "fuel1x4z06cpw2p4kn73xwh09ck3njz2uctg3n4vv9lk8v4x9slaylqcshceprc");
  });

  test('deploy contract', () async {
    final smartContractWallet =
        await getSmartContractWallet(stubR1PublicKey, testWalletPrivateKey);
    // will fail if the contract is already deployed
    await smartContractWallet.deployContract();
  }, skip: 'Should be run manually');
}
