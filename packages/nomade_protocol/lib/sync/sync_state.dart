/// Synchronization states
enum SyncState {
  idle,
  syncing,
  connected,
  disconnected,
  error,
}

/// Represents the current synchronization state with details
class SyncStatus {
  final SyncState state;
  final DateTime lastSync;
  final int connectedDevices;
  final String? errorMessage;

  const SyncStatus({
    required this.state,
    required this.lastSync,
    this.connectedDevices = 0,
    this.errorMessage,
  });

  /// Create a copy with modified fields
  SyncStatus copyWith({
    SyncState? state,
    DateTime? lastSync,
    int? connectedDevices,
    String? errorMessage,
  }) {
    return SyncStatus(
      state: state ?? this.state,
      lastSync: lastSync ?? this.lastSync,
      connectedDevices: connectedDevices ?? this.connectedDevices,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }

  /// Check if currently syncing
  bool get isSyncing => state == SyncState.syncing;

  /// Check if connected
  bool get isConnected => state == SyncState.connected;

  /// Check if has error
  bool get hasError => state == SyncState.error;

  @override
  String toString() {
    return 'SyncStatus(state: $state, lastSync: $lastSync, '
        'connectedDevices: $connectedDevices, errorMessage: $errorMessage)';
  }
}
