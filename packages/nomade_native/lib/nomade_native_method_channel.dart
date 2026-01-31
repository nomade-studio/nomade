import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'nomade_native_platform_interface.dart';

/// An implementation of [NomadeNativePlatform] that uses method channels.
class MethodChannelNomadeNative extends NomadeNativePlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('nomade_native');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>(
      'getPlatformVersion',
    );
    return version;
  }
}
