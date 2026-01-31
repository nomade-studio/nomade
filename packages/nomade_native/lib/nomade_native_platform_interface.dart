import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'nomade_native_method_channel.dart';

abstract class NomadeNativePlatform extends PlatformInterface {
  /// Constructs a NomadeNativePlatform.
  NomadeNativePlatform() : super(token: _token);

  static final Object _token = Object();

  static NomadeNativePlatform _instance = MethodChannelNomadeNative();

  /// The default instance of [NomadeNativePlatform] to use.
  ///
  /// Defaults to [MethodChannelNomadeNative].
  static NomadeNativePlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [NomadeNativePlatform] when
  /// they register themselves.
  static set instance(NomadeNativePlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
