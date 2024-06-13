// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.39.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<UuidValue> handleUuidTwinRustAsyncSse({required UuidValue id}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinRustAsyncSseHandleUuidTwinRustAsyncSse(
            id: id);

Future<List<UuidValue>> handleUuidsTwinRustAsyncSse(
        {required List<UuidValue> ids}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinRustAsyncSseHandleUuidsTwinRustAsyncSse(
            ids: ids);

Future<FeatureUuidTwinRustAsyncSse> handleNestedUuidsTwinRustAsyncSse(
        {required FeatureUuidTwinRustAsyncSse ids}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinRustAsyncSseHandleNestedUuidsTwinRustAsyncSse(
            ids: ids);

class FeatureUuidTwinRustAsyncSse {
  final UuidValue one;

  const FeatureUuidTwinRustAsyncSse({
    required this.one,
  });

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureUuidTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          one == other.one;
}
