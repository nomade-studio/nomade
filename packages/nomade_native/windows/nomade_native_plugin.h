#ifndef FLUTTER_PLUGIN_NOMADE_NATIVE_PLUGIN_H_
#define FLUTTER_PLUGIN_NOMADE_NATIVE_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace nomade_native {

class NomadeNativePlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  NomadeNativePlugin();

  virtual ~NomadeNativePlugin();

  // Disallow copy and assign.
  NomadeNativePlugin(const NomadeNativePlugin&) = delete;
  NomadeNativePlugin& operator=(const NomadeNativePlugin&) = delete;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace nomade_native

#endif  // FLUTTER_PLUGIN_NOMADE_NATIVE_PLUGIN_H_
