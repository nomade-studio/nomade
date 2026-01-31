/// Support for Nomade Native integration
library;

export 'src/rust/api.dart';
export 'src/rust/frb_generated.dart';

import 'package:nomade_native/src/rust/frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'dart:io';

class NomadeNative {
  static Future<void> init() async {
    ExternalLibrary? lib;
    if (Platform.isIOS || Platform.isMacOS) {
      lib = ExternalLibrary.process(iKnowHowToUseIt: true);
    }
    await RustLib.init(externalLibrary: lib);
  }
}
