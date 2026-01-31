import 'package:flutter/material.dart';
import 'package:nomade_native/nomade_native.dart';
import 'package:nomade_app/screens/chat_screen.dart';

void main() async {
  await NomadeNative.init();
  runApp(const NomadeApp());
}

class NomadeApp extends StatelessWidget {
  const NomadeApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Nomade',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const ChatScreen(),
    );
  }
}
