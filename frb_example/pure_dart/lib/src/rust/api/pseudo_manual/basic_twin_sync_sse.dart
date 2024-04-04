// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.30.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'basic_twin_sync_sse.freezed.dart';

int exampleBasicTypeI8TwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeI8TwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeI16TwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeI16TwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeI32TwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeI32TwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeI64TwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeI64TwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeU8TwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeU8TwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeU16TwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeU16TwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeU32TwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeU32TwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeU64TwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeU64TwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeIsizeTwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeIsizeTwinSyncSse(arg: arg, hint: hint);

int exampleBasicTypeUsizeTwinSyncSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeUsizeTwinSyncSse(arg: arg, hint: hint);

double exampleBasicTypeF32TwinSyncSse({required double arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeF32TwinSyncSse(arg: arg, hint: hint);

double exampleBasicTypeF64TwinSyncSse({required double arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeF64TwinSyncSse(arg: arg, hint: hint);

bool exampleBasicTypeBoolTwinSyncSse({required bool arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeBoolTwinSyncSse(arg: arg, hint: hint);

String exampleBasicTypeStringTwinSyncSse({required String arg, dynamic hint}) =>
    RustLib.instance.api
        .exampleBasicTypeStringTwinSyncSse(arg: arg, hint: hint);

Uint8List exampleBasicTypeBytesTwinSyncSse(
        {required List<int> arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeBytesTwinSyncSse(arg: arg, hint: hint);

BasicPrimitiveEnumTwinSyncSse
    exampleBasicTypeBasicPrimitiveEnumTwinSyncSseTwinSyncSse(
            {required BasicPrimitiveEnumTwinSyncSse arg, dynamic hint}) =>
        RustLib.instance.api
            .exampleBasicTypeBasicPrimitiveEnumTwinSyncSseTwinSyncSse(
                arg: arg, hint: hint);

BasicGeneralEnumTwinSyncSse
    exampleBasicTypeBasicGeneralEnumTwinSyncSseTwinSyncSse(
            {required BasicGeneralEnumTwinSyncSse arg, dynamic hint}) =>
        RustLib.instance.api
            .exampleBasicTypeBasicGeneralEnumTwinSyncSseTwinSyncSse(
                arg: arg, hint: hint);

BasicStructTwinSyncSse exampleBasicTypeBasicStructTwinSyncSseTwinSyncSse(
        {required BasicStructTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeBasicStructTwinSyncSseTwinSyncSse(
        arg: arg, hint: hint);

@freezed
sealed class BasicGeneralEnumTwinSyncSse with _$BasicGeneralEnumTwinSyncSse {
  const BasicGeneralEnumTwinSyncSse._();

  const factory BasicGeneralEnumTwinSyncSse.apple({
    required String field,
  }) = BasicGeneralEnumTwinSyncSse_Apple;
  const factory BasicGeneralEnumTwinSyncSse.orange() =
      BasicGeneralEnumTwinSyncSse_Orange;
}

enum BasicPrimitiveEnumTwinSyncSse {
  apple,
  orange,
}

class BasicStructTwinSyncSse {
  final String? apple;
  final int? orange;

  const BasicStructTwinSyncSse({
    this.apple,
    this.orange,
  });

  @override
  int get hashCode => apple.hashCode ^ orange.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BasicStructTwinSyncSse &&
          runtimeType == other.runtimeType &&
          apple == other.apple &&
          orange == other.orange;
}
