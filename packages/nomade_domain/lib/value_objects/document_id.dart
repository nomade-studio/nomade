import 'package:equatable/equatable.dart';

/// Document identifier (UUID v4)
class DocumentId extends Equatable {
  final String value;

  const DocumentId(this.value);

  @override
  List<Object?> get props => [value];

  @override
  String toString() => value;
}
