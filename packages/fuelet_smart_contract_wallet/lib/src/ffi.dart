import 'bridge_generated.dart';
import 'ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';

FueletSmartContractWallet? _wrapper;

FueletSmartContractWallet createWrapper(ExternalLibrary lib) {
  _wrapper ??= createWrapperImpl(lib);
  return _wrapper!;
}
