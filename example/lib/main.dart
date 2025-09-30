import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:apple_like_avatar_generator/apple_like_avatar_generator.dart';

Future<void> main() async {
  await AppleLikeAvatarGenerator.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('Avatar Generator Example')),
        body: Center(
          child: ClipOval(
            child: FutureBuilder<Uint8List>(
              future: AppleLikeAvatarGenerator.generateWithName("John Doe"),
              builder: (context, snapshot) {
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return const CircularProgressIndicator();
                } else if (snapshot.hasError) {
                  return Text('Error: ${snapshot.error}');
                } else if (snapshot.hasData) {
                  return Image.memory(snapshot.data!, width: 512, height: 512);
                } else {
                  return const Text('No data');
                }
              },
            ),
          ),
        ),
      ),
    );
  }
}
