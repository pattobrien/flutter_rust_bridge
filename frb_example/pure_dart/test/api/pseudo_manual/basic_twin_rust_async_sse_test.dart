// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `basic_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';
import 'dart:typed_data';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('basic', () {
    addTestsIdentityFunctionCall(
        exampleBasicTypeI8TwinRustAsyncSse, <int>[0, -128, 127]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeI16TwinRustAsyncSse, <int>[0, -32768, 32767]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeI32TwinRustAsyncSse, <int>[0, -2147483648, 2147483647]);
    addTestsIdentityFunctionCall(exampleBasicTypeI64TwinRustAsyncSse, <int>[
      0,
      -9007199254740992,
      9007199254740992,
      -9223372036854775808,
      9223372036854775807
    ]);
    addTestsIdentityFunctionCall(exampleBasicTypeI128TwinRustAsyncSse, <BigInt>[
      BigInt.parse("0"),
      BigInt.parse("-9007199254740992"),
      BigInt.parse("9007199254740992"),
      BigInt.parse("-9223372036854775808"),
      BigInt.parse("9223372036854775807"),
      BigInt.parse("-170141183460469231731687303715884105728"),
      BigInt.parse("170141183460469231731687303715884105727")
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeU8TwinRustAsyncSse, <int>[0, 255]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeU16TwinRustAsyncSse, <int>[0, 65535]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeU32TwinRustAsyncSse, <int>[0, 4294967295]);
    addTestsIdentityFunctionCall(exampleBasicTypeU64TwinRustAsyncSse, <BigInt>[
      BigInt.parse("0"),
      BigInt.parse("9007199254740992"),
      BigInt.parse("9223372036854775807"),
      BigInt.parse("18446744073709551615")
    ]);
    addTestsIdentityFunctionCall(exampleBasicTypeU128TwinRustAsyncSse, <BigInt>[
      BigInt.parse("0"),
      BigInt.parse("9007199254740992"),
      BigInt.parse("18446744073709551615"),
      BigInt.parse("340282366920938463463374607431768211455")
    ]);
    addTestsIdentityFunctionCall(exampleBasicTypeIsizeTwinRustAsyncSse,
        <int>[0, -2147483648, 2147483647]);
    addTestsIdentityFunctionCall(exampleBasicTypeUsizeTwinRustAsyncSse,
        <BigInt>[BigInt.parse("0"), BigInt.parse("4294967295")]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeF32TwinRustAsyncSse, <double>[0, -42.5, 123456]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeF64TwinRustAsyncSse, <double>[0, -42.5, 123456]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeBoolTwinRustAsyncSse, <bool>[false, true]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeStringTwinRustAsyncSse, <String>["", "hello", "😂"]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeBytesTwinRustAsyncSse, <Uint8List>[
      Uint8List.fromList([]),
      Uint8List.fromList([255, 0]),
      Uint8List.fromList([10, 20, 30, 40])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeBasicPrimitiveEnumTwinRustAsyncSseTwinRustAsyncSse,
        <BasicPrimitiveEnumTwinRustAsyncSse>[
          BasicPrimitiveEnumTwinRustAsyncSse.apple,
          BasicPrimitiveEnumTwinRustAsyncSse.orange
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeBasicGeneralEnumTwinRustAsyncSseTwinRustAsyncSse,
        <BasicGeneralEnumTwinRustAsyncSse>[
          BasicGeneralEnumTwinRustAsyncSse.apple(field: "one"),
          BasicGeneralEnumTwinRustAsyncSse.orange()
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeBasicStructTwinRustAsyncSseTwinRustAsyncSse,
        <BasicStructTwinRustAsyncSse>[
          BasicStructTwinRustAsyncSse(apple: null, orange: null),
          BasicStructTwinRustAsyncSse(apple: "one", orange: 42)
        ]);
  });
}
