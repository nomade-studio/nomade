import 'package:nomade_domain/nomade_domain.dart';
import 'sync_state.dart';

/// Coordinates synchronization between devices
class SyncCoordinator {
  SyncStatus _status = SyncStatus(
    state: SyncState.idle,
    lastSync: DateTime.now(),
  );

  /// Get current sync status
  SyncStatus get status => _status;

  /// Initialize the sync coordinator
  Future<void> initialize() async {
    // TODO: Initialize connection to Rust core via flutter_rust_bridge
    _updateStatus(SyncState.idle);
  }

  /// Start synchronization
  Future<void> startSync() async {
    _updateStatus(SyncState.syncing);
    try {
      // TODO: Implement actual sync via Rust core
      await Future.delayed(const Duration(seconds: 1));
      _updateStatus(SyncState.connected);
    } catch (e) {
      _updateStatus(SyncState.error, errorMessage: e.toString());
    }
  }

  /// Stop synchronization
  Future<void> stopSync() async {
    _updateStatus(SyncState.disconnected);
  }

  /// Sync an artifact to other devices
  Future<void> syncArtifact(Artifact artifact) async {
    if (_status.state != SyncState.connected) {
      throw StateError('Cannot sync artifact while not connected');
    }
    // TODO: Implement artifact sync via Rust core
  }

  /// Get list of connected devices
  Future<List<Device>> getConnectedDevices() async {
    // TODO: Implement via Rust core
    return [];
  }

  /// Clean up resources
  Future<void> dispose() async {
    await stopSync();
  }

  void _updateStatus(SyncState state, {String? errorMessage}) {
    _status = SyncStatus(
      state: state,
      lastSync: DateTime.now(),
      connectedDevices: _status.connectedDevices,
      errorMessage: errorMessage,
    );
  }
}
