import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_fuelet_smart_contract_wallet/wallet/smart_contract_wallet.dart';

const _betaApiUrl = 'https://beta-5.fuel.network';
const _testWalletPrivateKey =
    'e5e05a4ab2919dc01b97c90a48853fd4dfbd204e92e44327375702ab09bb184e';
const _stubR1PublicKey =
    '0xfe1177573a47310756f27ef25c7229b5da8d5109a7258b297f23a6768c78cb40';
const _notGeneratedTitle = 'not generated yet';

void main() {
  runApp(const SmartContractWalletUsageExample());
}

class SmartContractWalletUsageExample extends StatefulWidget {
  const SmartContractWalletUsageExample({super.key});

  @override
  State<SmartContractWalletUsageExample> createState() =>
      _SmartContractWalletUsageExampleState();
}

const _textStyle = TextStyle(fontSize: 25);

List<Widget> _commonText(String text) {
  return [
    SelectableText(
      text,
      style: _textStyle,
      textAlign: TextAlign.left,
    ),
    const SizedBox(height: 8),
  ];
}

List<Widget> _commonButton(String text, VoidCallback? onPressed) {
  return [
    OutlinedButton(
      onPressed: onPressed,
      child: Text(text),
    ),
    const SizedBox(height: 8),
  ];
}

class _SmartContractWalletUsageExampleState
    extends State<SmartContractWalletUsageExample> {
  String _r1PublicKey = _stubR1PublicKey;
  String _recoveryWalletPrivateKey = _testWalletPrivateKey;
  String _predicateAddress = _notGeneratedTitle;

  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    const spacerSmall = SizedBox(height: 10);
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Fuelet smart contract wallet example'),
        ),
        body: SingleChildScrollView(
          child: Container(
            padding: const EdgeInsets.all(10),
            child: Column(
              children: [
                ..._commonText('r1PublicKey: $_r1PublicKey'),
                ..._commonText(
                    'recoveryWalletPrivateKey: $_recoveryWalletPrivateKey'),
                ..._commonText('predicateAddress: $_predicateAddress'),
                ..._commonButton('Connect', _connectToWallet),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Future<void> _connectToWallet() async {
    final wallet = await SmartContractWallet.connect(
        networkUrl: _betaApiUrl,
        r1PublicKey: _r1PublicKey,
        recoveryWalletPrivateKey: _recoveryWalletPrivateKey);
    setState(() {
      _predicateAddress = wallet.predicateBech32Address;
    });
  }
}
