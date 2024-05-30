// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.36.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// The type `DROP_COUNT` is not used by any `pub` functions, thus it is ignored.
// The functions `deref`, `initialize`, `drop` are not `pub`, thus are ignored.

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DroppableTwinSyncSse>>
@sealed
class DroppableTwinSyncSse extends RustOpaque {
  // Not to be used by end users
  DroppableTwinSyncSse.frbInternalDcoDecode(List<dynamic> wire)
      : super.frbInternalDcoDecode(wire, _kStaticData);

  // Not to be used by end users
  DroppableTwinSyncSse.frbInternalSseDecode(
      BigInt ptr, int externalSizeOnNative)
      : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_DroppableTwinSyncSse,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_DroppableTwinSyncSse,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_DroppableTwinSyncSsePtr,
  );

  Stream<int> createStreamTwinSyncSse({dynamic hint}) => RustLib.instance.api
      .crateApiPseudoManualDroppingTwinSyncSseDroppableTwinSyncSseCreateStreamTwinSyncSse(
          that: this, hint: hint);

  static int getDropCountTwinSyncSse({dynamic hint}) => RustLib.instance.api
      .crateApiPseudoManualDroppingTwinSyncSseDroppableTwinSyncSseGetDropCountTwinSyncSse(
          hint: hint);

  static DroppableTwinSyncSse newTwinSyncSse({dynamic hint}) => RustLib
      .instance.api
      .crateApiPseudoManualDroppingTwinSyncSseDroppableTwinSyncSseNewTwinSyncSse(
          hint: hint);

  void simpleMethodTwinSyncSse({dynamic hint}) => RustLib.instance.api
      .crateApiPseudoManualDroppingTwinSyncSseDroppableTwinSyncSseSimpleMethodTwinSyncSse(
          that: this, hint: hint);
}
