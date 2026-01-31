import 'package:equatable/equatable.dart';
import 'package:json_annotation/json_annotation.dart';

part 'task.g.dart';

enum TaskStatus {
  todo,
  inProgress,
  done,
  archived,
}

enum Priority {
  low,
  medium,
  high,
  urgent,
}

@JsonSerializable()
class Task extends Equatable {
  final String id;
  final String title;
  final String? description;
  final TaskStatus status;
  final Priority priority;
  final DateTime? dueDate;
  final DateTime createdAt;
  final DateTime? completedAt;

  const Task({
    required this.id,
    required this.title,
    this.description,
    required this.status,
    required this.priority,
    this.dueDate,
    required this.createdAt,
    this.completedAt,
  });

  factory Task.fromJson(Map<String, dynamic> json) => _$TaskFromJson(json);

  Map<String, dynamic> toJson() => _$TaskToJson(this);

  @override
  List<Object?> get props => [
        id,
        title,
        description,
        status,
        priority,
        dueDate,
        createdAt,
        completedAt,
      ];
}
