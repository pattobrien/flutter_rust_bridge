// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.30.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<String> simpleUseAsyncSpawn({required String arg, dynamic hint}) =>
    RustLib.instance.api.simpleUseAsyncSpawn(arg: arg, hint: hint);

Future<String> simpleUseAsyncSpawnBlocking(
        {required String arg, dynamic hint}) =>
    RustLib.instance.api.simpleUseAsyncSpawnBlocking(arg: arg, hint: hint);

Future<Stream<String>> simpleUseAsyncSpawnLocal(
        {required String arg, dynamic hint}) =>
    RustLib.instance.api.simpleUseAsyncSpawnLocal(arg: arg, hint: hint);
