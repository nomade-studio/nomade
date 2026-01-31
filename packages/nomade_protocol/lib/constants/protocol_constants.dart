/// Protocol version
const int PROTOCOL_VERSION = 1;

/// Default QUIC port
const int DEFAULT_QUIC_PORT = 8443;

/// QR code validity duration (seconds)
const int QR_CODE_MAX_AGE_SECONDS = 300; // 5 minutes

/// Maximum chunk size for artifact transfer (bytes)
const int ARTIFACT_CHUNK_SIZE = 64 * 1024; // 64 KB

/// Heartbeat interval (seconds)
const int HEARTBEAT_INTERVAL_SECONDS = 30;

/// Connection timeout (seconds)
const int CONNECTION_TIMEOUT_SECONDS = 10;

/// Maximum operations per second (rate limiting)
const int MAX_OPERATIONS_PER_SECOND = 100;

/// Maximum artifact transfer rate (bytes per second)
const int MAX_ARTIFACT_TRANSFER_RATE = 10 * 1024 * 1024; // 10 MB/s
