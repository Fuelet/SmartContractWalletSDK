// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.6.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';

class FueletSmartContractWalletPlatform extends FlutterRustBridgeBase<FueletSmartContractWalletWire> with FlutterRustBridgeSetupMixin {
  FueletSmartContractWalletPlatform(FutureOr<WasmModule> dylib) : super(FueletSmartContractWalletWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  Object api2wire_u64(int raw) {
    return castNativeBigInt(raw);
  }

  @protected
  Uint8List api2wire_u8_array_32(U8Array32 raw) {
    return Uint8List.fromList(raw);
  }

  @protected
  Uint8List api2wire_uint_8_list(Uint8List raw) {
    return raw;
  }
// Section: finalizer
}

// Section: WASM wire module

@JS('wasm_bindgen')
external FueletSmartContractWalletWasmModule get wasmModule;

@JS()
@anonymous
class FueletSmartContractWalletWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external FueletSmartContractWalletWasmModule bind(dynamic thisArg, String moduleName);
  external dynamic /* void */ wire_deploy_contract(NativePortType port_, String private_key, String node_url);

  external dynamic /* void */ wire_get_script(NativePortType port_, String private_key, String node_url, String contract_id_str);

  external dynamic /* void */ wire_get_predicate_address(NativePortType port_, String wallet_public_key, Uint8List script_hash);

  external dynamic /* void */ wire_gen_transfer_tx_request(NativePortType port_, String node_url, String wallet_public_key, Uint8List script_hash, String to, Object amount, String asset);

  external dynamic /* void */ wire_send_tx(NativePortType port_, String node_url, Uint8List encoded_tx, Uint8List signature);
}

// Section: WASM wire connector

class FueletSmartContractWalletWire extends FlutterRustBridgeWasmWireBase<FueletSmartContractWalletWasmModule> {
  FueletSmartContractWalletWire(FutureOr<WasmModule> module) : super(WasmModule.cast<FueletSmartContractWalletWasmModule>(module));

  void wire_deploy_contract(NativePortType port_, String private_key, String node_url) => wasmModule.wire_deploy_contract(port_, private_key, node_url);

  void wire_get_script(NativePortType port_, String private_key, String node_url, String contract_id_str) => wasmModule.wire_get_script(port_, private_key, node_url, contract_id_str);

  void wire_get_predicate_address(NativePortType port_, String wallet_public_key, Uint8List script_hash) => wasmModule.wire_get_predicate_address(port_, wallet_public_key, script_hash);

  void wire_gen_transfer_tx_request(NativePortType port_, String node_url, String wallet_public_key, Uint8List script_hash, String to, Object amount, String asset) => wasmModule.wire_gen_transfer_tx_request(port_, node_url, wallet_public_key, script_hash, to, amount, asset);

  void wire_send_tx(NativePortType port_, String node_url, Uint8List encoded_tx, Uint8List signature) => wasmModule.wire_send_tx(port_, node_url, encoded_tx, signature);
}
