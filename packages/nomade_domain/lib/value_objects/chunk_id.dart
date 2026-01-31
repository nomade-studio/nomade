import 'package:equatable/equatable.dart';

/// Chunk identifier (content hash)
class ChunkId extends Equatable {
  final String value;

  const ChunkId(this.value);

  @override
  List<Object?> get props => [value];

  @override
  String toString() => value;
}
