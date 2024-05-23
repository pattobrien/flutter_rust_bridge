// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `basic_map_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_map_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';
import 'dart:typed_data';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_twin_rust_async.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('basic_map', () {
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI8TwinRustAsync, <Map<int, int>>[
      {},
      {42: 0},
      {42: -128},
      {42: 127}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI16TwinRustAsync, <Map<int, int>>[
      {},
      {42: 0},
      {42: -32768},
      {42: 32767}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI32TwinRustAsync, <Map<int, int>>[
      {},
      {42: 0},
      {42: -2147483648},
      {42: 2147483647}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI64TwinRustAsync, <Map<int, int>>[
      {},
      {42: 0},
      {42: -9007199254740992},
      {42: 9007199254740992},
      {42: -9223372036854775808},
      {42: 9223372036854775807}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI128TwinRustAsync, <Map<int, BigInt>>[
      {},
      {42: BigInt.parse("0")},
      {42: BigInt.parse("-9007199254740992")},
      {42: BigInt.parse("9007199254740992")},
      {42: BigInt.parse("-9223372036854775808")},
      {42: BigInt.parse("9223372036854775807")},
      {42: BigInt.parse("-170141183460469231731687303715884105728")},
      {42: BigInt.parse("170141183460469231731687303715884105727")}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU8TwinRustAsync, <Map<int, int>>[
      {},
      {42: 0},
      {42: 255}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU16TwinRustAsync, <Map<int, int>>[
      {},
      {42: 0},
      {42: 65535}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU32TwinRustAsync, <Map<int, int>>[
      {},
      {42: 0},
      {42: 4294967295}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU64TwinRustAsync, <Map<int, BigInt>>[
      {},
      {42: BigInt.parse("0")},
      {42: BigInt.parse("9007199254740992")},
      {42: BigInt.parse("9223372036854775807")},
      {42: BigInt.parse("18446744073709551615")}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU128TwinRustAsync, <Map<int, BigInt>>[
      {},
      {42: BigInt.parse("0")},
      {42: BigInt.parse("9007199254740992")},
      {42: BigInt.parse("18446744073709551615")},
      {42: BigInt.parse("340282366920938463463374607431768211455")}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeIsizeTwinRustAsync, <Map<int, int>>[
      {},
      {42: 0},
      {42: -2147483648},
      {42: 2147483647}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeUsizeTwinRustAsync, <Map<int, BigInt>>[
      {},
      {42: BigInt.parse("0")},
      {42: BigInt.parse("4294967295")}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeF32TwinRustAsync, <Map<int, double>>[
      {},
      {42: 0},
      {42: -42.5},
      {42: 123456}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeF64TwinRustAsync, <Map<int, double>>[
      {},
      {42: 0},
      {42: -42.5},
      {42: 123456}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBoolTwinRustAsync, <Map<int, bool>>[
      {},
      {42: false},
      {42: true}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeStringTwinRustAsync, <Map<int, String>>[
      {},
      {42: ""},
      {42: "hello"},
      {42: "😂"}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBytesTwinRustAsync, <Map<int, Uint8List>>[
      {},
      {42: Uint8List.fromList([])},
      {
        42: Uint8List.fromList([255, 0])
      },
      {
        42: Uint8List.fromList([10, 20, 30, 40])
      }
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicPrimitiveEnumTwinRustAsyncTwinRustAsync,
        <Map<int, BasicPrimitiveEnumTwinRustAsync>>[
          {},
          {42: BasicPrimitiveEnumTwinRustAsync.apple},
          {42: BasicPrimitiveEnumTwinRustAsync.orange}
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicGeneralEnumTwinRustAsyncTwinRustAsync,
        <Map<int, BasicGeneralEnumTwinRustAsync>>[
          {},
          {42: BasicGeneralEnumTwinRustAsync.apple(field: "one")},
          {42: BasicGeneralEnumTwinRustAsync.orange()}
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicStructTwinRustAsyncTwinRustAsync,
        <Map<int, BasicStructTwinRustAsync>>[
          {},
          {42: BasicStructTwinRustAsync(apple: null, orange: null)},
          {42: BasicStructTwinRustAsync(apple: "one", orange: 42)}
        ]);
  });
}
