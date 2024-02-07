abstract class WrappedSmartContractWallet {
  String get predicateBech32Address;

  String get r1PublicKey;

  Future<void> deployContract();

  Future<(String, String)> createTransferTxRequest({
    required String destinationB256Address,
    required int fractionalAmount,
    required String assetId,
  });

  Future<String> sendTransaction(
      {required String transactionRequestHexOrJson,
      required String r1Signature});
}
