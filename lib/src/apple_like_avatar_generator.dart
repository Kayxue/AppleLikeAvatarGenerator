import 'dart:typed_data';

import './rust/api/core.dart' as core;
import 'rust/frb_generated.dart';

class AppleLikeAvatarGenerator {
  AppleLikeAvatarGenerator._();

  /// Initializes the library.
  static Future<void> init() async {
    await RustLib.init();
  }

  /// Generates an avatar image based on the provided [name].
  static Future<Uint8List> generateWithName(String name) async {
    return core.generateWithName(name);
  }

  /// Generates an avatar image based on the provided [firstName] and [lastName].
  static Future<Uint8List> generateWithFirstNameLastName({
    required String firstName,
    required String lastName,
  }) async {
    return core.generateWithFirstNameLastName(firstName, lastName);
  }
}
