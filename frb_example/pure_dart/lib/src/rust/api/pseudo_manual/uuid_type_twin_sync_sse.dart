// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.36.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

// The functions `fmt`, `clone` are not `pub`, thus are ignored.

UuidValue handleUuidTwinSyncSse({required UuidValue id, dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinSyncSseHandleUuidTwinSyncSse(
            id: id, hint: hint);

List<UuidValue> handleUuidsTwinSyncSse(
        {required List<UuidValue> ids, dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinSyncSseHandleUuidsTwinSyncSse(
            ids: ids, hint: hint);

FeatureUuidTwinSyncSse handleNestedUuidsTwinSyncSse(
        {required FeatureUuidTwinSyncSse ids, dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinSyncSseHandleNestedUuidsTwinSyncSse(
            ids: ids, hint: hint);

class FeatureUuidTwinSyncSse {
  final UuidValue one;

  const FeatureUuidTwinSyncSse({
    required this.one,
  });

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureUuidTwinSyncSse &&
          runtimeType == other.runtimeType &&
          one == other.one;
}
