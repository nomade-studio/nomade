package com.example.nomade_native

import io.flutter.embedding.engine.plugins.FlutterPlugin

/** NomadeNativePlugin */
class NomadeNativePlugin : FlutterPlugin {
    override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        // No-op: This plugin uses FFI and does not typically need a MethodChannel.
        // The Rust library is loaded dynamically by Dart.
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        // No cleanup needed.
    }
}
