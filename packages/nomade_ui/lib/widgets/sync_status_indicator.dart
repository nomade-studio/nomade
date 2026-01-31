import 'package:flutter/material.dart';
import 'package:nomade_protocol/nomade_protocol.dart';

/// A widget that displays the current sync status
class SyncStatusIndicator extends StatelessWidget {
  final SyncStatus status;

  const SyncStatusIndicator({
    super.key,
    required this.status,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: _getColor().withValues(alpha: .1),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: _getColor(), width: 1),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(_getIcon(), size: 16, color: _getColor()),
          const SizedBox(width: 6),
          Text(
            _getStatusText(),
            style: TextStyle(
              color: _getColor(),
              fontSize: 12,
              fontWeight: FontWeight.w500,
            ),
          ),
          if (status.connectedDevices > 0) ...[
            const SizedBox(width: 4),
            Text(
              '(${status.connectedDevices})',
              style: TextStyle(
                color: _getColor(),
                fontSize: 12,
              ),
            ),
          ],
        ],
      ),
    );
  }

  Color _getColor() {
    switch (status.state) {
      case SyncState.idle:
        return Colors.grey;
      case SyncState.syncing:
        return Colors.blue;
      case SyncState.connected:
        return Colors.green;
      case SyncState.disconnected:
        return Colors.orange;
      case SyncState.error:
        return Colors.red;
    }
  }

  IconData _getIcon() {
    switch (status.state) {
      case SyncState.idle:
        return Icons.sync_disabled;
      case SyncState.syncing:
        return Icons.sync;
      case SyncState.connected:
        return Icons.sync;
      case SyncState.disconnected:
        return Icons.sync_problem;
      case SyncState.error:
        return Icons.error;
    }
  }

  String _getStatusText() {
    switch (status.state) {
      case SyncState.idle:
        return 'Idle';
      case SyncState.syncing:
        return 'Syncing...';
      case SyncState.connected:
        return 'Connected';
      case SyncState.disconnected:
        return 'Disconnected';
      case SyncState.error:
        return 'Error';
    }
  }
}
