import 'package:flutter/material.dart';
import 'package:nomade_domain/nomade_domain.dart';

/// A card widget for displaying an artifact
class ArtifactCard extends StatelessWidget {
  final Artifact artifact;
  final VoidCallback? onTap;

  const ArtifactCard({
    super.key,
    required this.artifact,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      child: ListTile(
        leading: _buildIcon(),
        title: Text(_getTitle()),
        subtitle: Text(_getSubtitle()),
        trailing: Text(
          _formatTimestamp(),
          style: Theme.of(context).textTheme.bodySmall,
        ),
        onTap: onTap,
      ),
    );
  }

  Widget _buildIcon() {
    IconData iconData;
    switch (artifact.type) {
      case ArtifactType.text:
        iconData = Icons.text_fields;
        break;
      case ArtifactType.file:
        iconData = Icons.insert_drive_file;
        break;
      case ArtifactType.image:
        iconData = Icons.image;
        break;
      case ArtifactType.url:
        iconData = Icons.link;
        break;
    }
    return Icon(iconData);
  }

  String _getTitle() {
    switch (artifact.type) {
      case ArtifactType.text:
        final content = artifact.content;
        if (content == null) return 'Text content';
        final maxLength = content.length < 50 ? content.length : 50;
        return content.substring(0, maxLength);
      case ArtifactType.file:
        return artifact.metadata['filename'] as String? ?? 'File';
      case ArtifactType.image:
        return 'Image';
      case ArtifactType.url:
        return artifact.content ?? 'URL';
    }
  }

  String _getSubtitle() {
    return 'From ${artifact.deviceId}';
  }

  String _formatTimestamp() {
    final now = DateTime.now();
    final diff = now.difference(artifact.timestamp);

    if (diff.inMinutes < 1) {
      return 'Just now';
    } else if (diff.inHours < 1) {
      return '${diff.inMinutes}m ago';
    } else if (diff.inDays < 1) {
      return '${diff.inHours}h ago';
    } else {
      return '${diff.inDays}d ago';
    }
  }
}
