import 'package:equatable/equatable.dart';

/// Device type
enum DeviceType {
  desktop,
  mobile,
  tablet,
  unknown,
}

/// Represents a device in the Nomade network
class Device extends Equatable {
  final String id;
  final String name;
  final DeviceType type;
  final String platform;
  final DateTime lastSeen;
  final bool isOnline;
  final String? ipAddress;

  const Device({
    required this.id,
    required this.name,
    required this.type,
    required this.platform,
    required this.lastSeen,
    this.isOnline = false,
    this.ipAddress,
  });

  /// Create a device from JSON
  factory Device.fromJson(Map<String, dynamic> json) {
    return Device(
      id: json['id'] as String,
      name: json['name'] as String,
      type: DeviceType.values.firstWhere(
        (e) => e.name == json['type'],
        orElse: () => DeviceType.unknown,
      ),
      platform: json['platform'] as String,
      lastSeen: DateTime.parse(json['last_seen'] as String),
      isOnline: json['is_online'] as bool? ?? false,
      ipAddress: json['ip_address'] as String?,
    );
  }

  /// Convert device to JSON
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'type': type.name,
      'platform': platform,
      'last_seen': lastSeen.toIso8601String(),
      'is_online': isOnline,
      if (ipAddress != null) 'ip_address': ipAddress,
    };
  }

  /// Create a copy with modified fields
  Device copyWith({
    String? id,
    String? name,
    DeviceType? type,
    String? platform,
    DateTime? lastSeen,
    bool? isOnline,
    String? ipAddress,
  }) {
    return Device(
      id: id ?? this.id,
      name: name ?? this.name,
      type: type ?? this.type,
      platform: platform ?? this.platform,
      lastSeen: lastSeen ?? this.lastSeen,
      isOnline: isOnline ?? this.isOnline,
      ipAddress: ipAddress ?? this.ipAddress,
    );
  }

  @override
  List<Object?> get props => [id, name, type, platform, lastSeen, isOnline, ipAddress];
}
