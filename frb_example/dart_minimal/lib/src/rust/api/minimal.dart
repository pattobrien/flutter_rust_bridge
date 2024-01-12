// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.16.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<int> minimalAdder({required int a, required int b, dynamic hint}) =>
    RustLib.instance.api.minimalAdder(a: a, b: b, hint: hint);

// Rust type: flutter_rust_bridge::RustOpaque<flutter_rust_bridge::for_generated::rust_async::RwLock<Apple>, NomRustOpaqueCodec>
@sealed
class Apple extends RustOpaque {
  Apple.dcoDecode(List<dynamic> wire) : super.dcoDecode(wire, _kStaticData);

  Apple.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_Apple,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_Apple,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_ApplePtr,
  );

  Future<void> appleMethodRef({dynamic hint}) =>
      RustLib.instance.api.appleAppleMethodRef(
        that: this,
      );

  static Future<Apple> newApple({dynamic hint}) =>
      RustLib.instance.api.appleNew(hint: hint);
}
