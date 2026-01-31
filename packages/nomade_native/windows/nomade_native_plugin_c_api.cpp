#include "include/nomade_native/nomade_native_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "nomade_native_plugin.h"

void NomadeNativePluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  nomade_native::NomadeNativePlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
