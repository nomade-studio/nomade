import 'package:equatable/equatable.dart';
import 'package:json_annotation/json_annotation.dart';
import '../value_objects/document_id.dart';

part 'document.g.dart';

@JsonSerializable()
class Document extends Equatable {
  final DocumentId id;
  final String title;
  final String content;
  final DateTime createdAt;
  final DateTime updatedAt;
  final Map<String, dynamic> metadata;

  const Document({
    required this.id,
    required this.title,
    required this.content,
    required this.createdAt,
    required this.updatedAt,
    this.metadata = const {},
  });

  factory Document.fromJson(Map<String, dynamic> json) =>
      _$DocumentFromJson(json);

  Map<String, dynamic> toJson() => _$DocumentToJson(this);

  Document copyWith({
    DocumentId? id,
    String? title,
    String? content,
    DateTime? createdAt,
    DateTime? updatedAt,
    Map<String, dynamic>? metadata,
  }) {
    return Document(
      id: id ?? this.id,
      title: title ?? this.title,
      content: content ?? this.content,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      metadata: metadata ?? this.metadata,
    );
  }

  @override
  List<Object?> get props => [id, title, content, createdAt, updatedAt, metadata];
}
