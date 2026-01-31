import 'package:equatable/equatable.dart';
import 'package:json_annotation/json_annotation.dart';
import 'message.dart';

part 'conversation.g.dart';

@JsonSerializable()
class Conversation extends Equatable {
  final String id;
  final String title;
  final List<Message> messages;
  final DateTime createdAt;
  final DateTime updatedAt;

  const Conversation({
    required this.id,
    required this.title,
    required this.messages,
    required this.createdAt,
    required this.updatedAt,
  });

  factory Conversation.fromJson(Map<String, dynamic> json) =>
      _$ConversationFromJson(json);

  Map<String, dynamic> toJson() => _$ConversationToJson(this);

  @override
  List<Object?> get props => [id, title, messages, createdAt, updatedAt];
}
