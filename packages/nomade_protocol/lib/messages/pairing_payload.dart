import 'package:equatable/equatable.dart';
import 'package:json_annotation/json_annotation.dart';

part 'pairing_payload.g.dart';

/// QR code pairing payload
@JsonSerializable()
class PairingPayload extends Equatable {
  final int version;
  final String deviceId;
  final String deviceName;
  final String publicKey;
  final String endpoint;
  final int timestamp;
  final String signature;

  const PairingPayload({
    required this.version,
    required this.deviceId,
    required this.deviceName,
    required this.publicKey,
    required this.endpoint,
    required this.timestamp,
    required this.signature,
  });

  factory PairingPayload.fromJson(Map<String, dynamic> json) =>
      _$PairingPayloadFromJson(json);

  Map<String, dynamic> toJson() => _$PairingPayloadToJson(this);

  /// Create a QR code URI from this payload
  String toQrCodeUri() {
    // TODO: Implement base64 encoding
    return 'nomade://pair/...';
  }

  /// Parse a QR code URI
  static PairingPayload fromQrCodeUri(String uri) {
    // TODO: Implement base64 decoding
    throw UnimplementedError();
  }

  @override
  List<Object?> get props => [
        version,
        deviceId,
        deviceName,
        publicKey,
        endpoint,
        timestamp,
        signature,
      ];
}
