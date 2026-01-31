import 'package:flutter/material.dart';
import 'package:nomade_domain/nomade_domain.dart';

/// A list tile widget for displaying a device
class DeviceListTile extends StatelessWidget {
  final Device device;
  final VoidCallback? onTap;

  const DeviceListTile({
    super.key,
    required this.device,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: _buildIcon(),
      title: Text(device.name),
      subtitle: Text('${device.platform} • ${_getStatusText()}'),
      trailing: device.isOnline
          ? const Icon(Icons.circle, color: Colors.green, size: 12)
          : const Icon(Icons.circle, color: Colors.grey, size: 12),
      onTap: onTap,
    );
  }

  Widget _buildIcon() {
    IconData iconData;
    switch (device.type) {
      case DeviceType.desktop:
        iconData = Icons.computer;
        break;
      case DeviceType.mobile:
        iconData = Icons.phone_android;
        break;
      case DeviceType.tablet:
        iconData = Icons.tablet;
        break;
      case DeviceType.unknown:
        iconData = Icons.device_unknown;
        break;
    }
    return Icon(iconData);
  }

  String _getStatusText() {
    if (device.isOnline) {
      return 'Online';
    } else {
      final diff = DateTime.now().difference(device.lastSeen);
      if (diff.inMinutes < 1) {
        return 'Offline';
      } else if (diff.inHours < 1) {
        return 'Last seen ${diff.inMinutes}m ago';
      } else if (diff.inDays < 1) {
        return 'Last seen ${diff.inHours}h ago';
      } else {
        return 'Last seen ${diff.inDays}d ago';
      }
    }
  }
}
