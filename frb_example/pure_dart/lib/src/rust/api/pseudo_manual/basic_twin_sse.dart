// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.30.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'basic_twin_sse.freezed.dart';

Future<int> exampleBasicTypeI8TwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeI8TwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeI16TwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeI16TwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeI32TwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeI32TwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeI64TwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeI64TwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeU8TwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeU8TwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeU16TwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeU16TwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeU32TwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeU32TwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeU64TwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeU64TwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeIsizeTwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeIsizeTwinSse(arg: arg, hint: hint);

Future<int> exampleBasicTypeUsizeTwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeUsizeTwinSse(arg: arg, hint: hint);

Future<double> exampleBasicTypeF32TwinSse(
        {required double arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeF32TwinSse(arg: arg, hint: hint);

Future<double> exampleBasicTypeF64TwinSse(
        {required double arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeF64TwinSse(arg: arg, hint: hint);

Future<bool> exampleBasicTypeBoolTwinSse({required bool arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeBoolTwinSse(arg: arg, hint: hint);

Future<String> exampleBasicTypeStringTwinSse(
        {required String arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeStringTwinSse(arg: arg, hint: hint);

Future<Uint8List> exampleBasicTypeBytesTwinSse(
        {required List<int> arg, dynamic hint}) =>
    RustLib.instance.api.exampleBasicTypeBytesTwinSse(arg: arg, hint: hint);

Future<BasicPrimitiveEnumTwinSse>
    exampleBasicTypeBasicPrimitiveEnumTwinSseTwinSse(
            {required BasicPrimitiveEnumTwinSse arg, dynamic hint}) =>
        RustLib.instance.api.exampleBasicTypeBasicPrimitiveEnumTwinSseTwinSse(
            arg: arg, hint: hint);

Future<BasicGeneralEnumTwinSse> exampleBasicTypeBasicGeneralEnumTwinSseTwinSse(
        {required BasicGeneralEnumTwinSse arg, dynamic hint}) =>
    RustLib.instance.api
        .exampleBasicTypeBasicGeneralEnumTwinSseTwinSse(arg: arg, hint: hint);

Future<BasicStructTwinSse> exampleBasicTypeBasicStructTwinSseTwinSse(
        {required BasicStructTwinSse arg, dynamic hint}) =>
    RustLib.instance.api
        .exampleBasicTypeBasicStructTwinSseTwinSse(arg: arg, hint: hint);

@freezed
sealed class BasicGeneralEnumTwinSse with _$BasicGeneralEnumTwinSse {
  const BasicGeneralEnumTwinSse._();

  const factory BasicGeneralEnumTwinSse.apple({
    required String field,
  }) = BasicGeneralEnumTwinSse_Apple;
  const factory BasicGeneralEnumTwinSse.orange() =
      BasicGeneralEnumTwinSse_Orange;
}

enum BasicPrimitiveEnumTwinSse {
  apple,
  orange,
}

class BasicStructTwinSse {
  final String? apple;
  final int? orange;

  const BasicStructTwinSse({
    this.apple,
    this.orange,
  });

  @override
  int get hashCode => apple.hashCode ^ orange.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BasicStructTwinSse &&
          runtimeType == other.runtimeType &&
          apple == other.apple &&
          orange == other.orange;
}
