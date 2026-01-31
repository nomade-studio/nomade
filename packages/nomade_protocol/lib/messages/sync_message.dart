import 'package:equatable/equatable.dart';
import 'package:json_annotation/json_annotation.dart';

part 'sync_message.g.dart';

enum MessageType {
  hello,
  helloAck,
  syncRequest,
  syncOperation,
  syncComplete,
  artifactRequest,
  artifactMetadata,
  artifactChunk,
  artifactComplete,
  heartbeat,
  error,
}

@JsonSerializable()
class SyncMessage extends Equatable {
  final int version;
  final MessageType messageType;
  final String messageId;
  final int timestamp;
  final Map<String, dynamic> payload;

  const SyncMessage({
    required this.version,
    required this.messageType,
    required this.messageId,
    required this.timestamp,
    required this.payload,
  });

  factory SyncMessage.fromJson(Map<String, dynamic> json) =>
      _$SyncMessageFromJson(json);

  Map<String, dynamic> toJson() => _$SyncMessageToJson(this);

  @override
  List<Object?> get props => [version, messageType, messageId, timestamp, payload];
}
