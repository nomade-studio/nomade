import 'package:flutter/material.dart';
import 'package:nomade_native/nomade_native.dart';
import 'package:nomade_ui/nomade_ui.dart';
import 'package:nomade_domain/nomade_domain.dart';
import 'package:uuid/uuid.dart';

class ChatScreen extends StatefulWidget {
  const ChatScreen({super.key});

  @override
  State<ChatScreen> createState() => _ChatScreenState();
}

class _ChatScreenState extends State<ChatScreen> {
  // Using Artifacts to represent messages
  final List<Artifact> _messages = [];
  final _uuid = const Uuid();

  // Helper to create a text artifact
  Artifact _createArtifact(String text, String deviceId) {
    return Artifact(
      id: _uuid.v4(),
      type: ArtifactType.text,
      deviceId: deviceId,
      timestamp: DateTime.now(),
      content: text,
    );
  }

  Future<void> _sendMessage(String text) async {
    setState(() {
      _messages.add(_createArtifact(text, 'local'));
    });

    try {
      // Synchronous call to Rust
      final response = processMessage(input: text);
      setState(() {
        _messages.add(_createArtifact(response, 'rust'));
      });
    } catch (e) {
      setState(() {
        _messages.add(_createArtifact('Error: $e', 'system'));
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Nomade Chat'),
      ),
      body: Column(
        children: [
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.all(8.0),
              itemCount: _messages.length,
              itemBuilder: (context, index) {
                final artifact = _messages[index];
                final isMe = artifact.deviceId == 'local';
                return ChatBubble(
                  message: artifact.content ?? '',
                  isMe: isMe,
                );
              },
            ),
          ),
          ChatInput(
            onSend: _sendMessage,
          ),
        ],
      ),
    );
  }
}
