# Nomade Protocol

Protocol definitions and message schemas for Nomade's synchronization protocol.

## Structure

- `messages/`: Protocol message definitions
  - `PairingPayload`: QR code pairing format
  - `SyncMessage`: Synchronization messages
- `constants/`: Protocol constants

## Messages

### PairingPayload
Format for QR code device pairing. See [docs/pairing.md](../../docs/pairing.md).

### SyncMessage
Base message format for synchronization. See [docs/sync-protocol.md](../../docs/sync-protocol.md).

## Constants

Protocol-level constants like version numbers, timeouts, and limits.

## Code Generation

This package uses `json_serializable` for JSON serialization:

```bash
dart run build_runner build
```
