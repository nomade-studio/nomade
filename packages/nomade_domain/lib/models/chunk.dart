import 'package:equatable/equatable.dart';
import 'package:json_annotation/json_annotation.dart';
import '../value_objects/chunk_id.dart';
import '../value_objects/document_id.dart';

part 'chunk.g.dart';

@JsonSerializable()
class Chunk extends Equatable {
  final ChunkId id;
  final DocumentId documentId;
  final String content;
  final int position;
  final Map<String, dynamic> metadata;

  const Chunk({
    required this.id,
    required this.documentId,
    required this.content,
    required this.position,
    this.metadata = const {},
  });

  factory Chunk.fromJson(Map<String, dynamic> json) => _$ChunkFromJson(json);

  Map<String, dynamic> toJson() => _$ChunkToJson(this);

  @override
  List<Object?> get props => [id, documentId, content, position, metadata];
}
