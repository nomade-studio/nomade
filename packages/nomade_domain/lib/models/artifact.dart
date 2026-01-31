import 'package:equatable/equatable.dart';

/// Types of artifacts that can be shared
enum ArtifactType {
  text,
  file,
  image,
  url,
}

/// Represents a shareable artifact (clipboard item, file, etc.)
class Artifact extends Equatable {
  final String id;
  final ArtifactType type;
  final String deviceId;
  final DateTime timestamp;
  final Map<String, dynamic> metadata;
  final String? content;
  final String? filePath;

  const Artifact({
    required this.id,
    required this.type,
    required this.deviceId,
    required this.timestamp,
    this.metadata = const {},
    this.content,
    this.filePath,
  });

  /// Create an artifact from JSON
  factory Artifact.fromJson(Map<String, dynamic> json) {
    return Artifact(
      id: json['id'] as String,
      type: ArtifactType.values.firstWhere(
        (e) => e.name == json['type'],
        orElse: () => ArtifactType.text,
      ),
      deviceId: json['device_id'] as String,
      timestamp: DateTime.parse(json['timestamp'] as String),
      metadata: json['metadata'] as Map<String, dynamic>? ?? {},
      content: json['content'] as String?,
      filePath: json['file_path'] as String?,
    );
  }

  /// Convert artifact to JSON
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'type': type.name,
      'device_id': deviceId,
      'timestamp': timestamp.toIso8601String(),
      'metadata': metadata,
      if (content != null) 'content': content,
      if (filePath != null) 'file_path': filePath,
    };
  }

  /// Create a copy with modified fields
  Artifact copyWith({
    String? id,
    ArtifactType? type,
    String? deviceId,
    DateTime? timestamp,
    Map<String, dynamic>? metadata,
    String? content,
    String? filePath,
  }) {
    return Artifact(
      id: id ?? this.id,
      type: type ?? this.type,
      deviceId: deviceId ?? this.deviceId,
      timestamp: timestamp ?? this.timestamp,
      metadata: metadata ?? this.metadata,
      content: content ?? this.content,
      filePath: filePath ?? this.filePath,
    );
  }

  @override
  List<Object?> get props =>
      [id, type, deviceId, timestamp, metadata, content, filePath];
}
