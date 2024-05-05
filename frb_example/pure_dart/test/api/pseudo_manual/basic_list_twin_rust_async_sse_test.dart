// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `basic_list_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_list_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_twin_rust_async_sse.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('basic_list', () {
    addTestsIdentityFunctionCall(
        exampleBasicListTypeI8TwinRustAsyncSse, <Int8List>[
      Int8List.fromList([]),
      Int8List.fromList([0]),
      Int8List.fromList([-128]),
      Int8List.fromList([127])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeI16TwinRustAsyncSse, <Int16List>[
      Int16List.fromList([]),
      Int16List.fromList([0]),
      Int16List.fromList([-32768]),
      Int16List.fromList([32767])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeI32TwinRustAsyncSse, <Int32List>[
      Int32List.fromList([]),
      Int32List.fromList([0]),
      Int32List.fromList([-2147483648]),
      Int32List.fromList([2147483647])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeI64TwinRustAsyncSse, <Int64List>[
      Int64List.fromList([]),
      Int64List.fromList([0]),
      Int64List.fromList([-9007199254740992]),
      Int64List.fromList([9007199254740992])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeU8TwinRustAsyncSse, <Uint8List>[
      Uint8List.fromList([]),
      Uint8List.fromList([0]),
      Uint8List.fromList([255])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeU16TwinRustAsyncSse, <Uint16List>[
      Uint16List.fromList([]),
      Uint16List.fromList([0]),
      Uint16List.fromList([65535])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeU32TwinRustAsyncSse, <Uint32List>[
      Uint32List.fromList([]),
      Uint32List.fromList([0]),
      Uint32List.fromList([4294967295])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeU64TwinRustAsyncSse, <Uint64List>[
      Uint64List.fromList([]),
      Uint64List.fromList([0]),
      Uint64List.fromList([9007199254740992])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeF32TwinRustAsyncSse, <Float32List>[
      Float32List.fromList([]),
      Float32List.fromList([0]),
      Float32List.fromList([-42.5]),
      Float32List.fromList([123456])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeF64TwinRustAsyncSse, <Float64List>[
      Float64List.fromList([]),
      Float64List.fromList([0]),
      Float64List.fromList([-42.5]),
      Float64List.fromList([123456])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBoolTwinRustAsyncSse, <List<bool>>[
      <bool>[],
      <bool>[false],
      <bool>[true]
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeStringTwinRustAsyncSse, <List<String>>[
      [],
      [""],
      ["hello"],
      ["😂"]
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBytesTwinRustAsyncSse, <List<Uint8List>>[
      [],
      [Uint8List.fromList([])],
      [
        Uint8List.fromList([255, 0])
      ],
      [
        Uint8List.fromList([10, 20, 30, 40])
      ]
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicPrimitiveEnumTwinRustAsyncSseTwinRustAsyncSse,
        <List<BasicPrimitiveEnumTwinRustAsyncSse>>[
          [],
          [BasicPrimitiveEnumTwinRustAsyncSse.apple],
          [BasicPrimitiveEnumTwinRustAsyncSse.orange]
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicGeneralEnumTwinRustAsyncSseTwinRustAsyncSse,
        <List<BasicGeneralEnumTwinRustAsyncSse>>[
          [],
          [BasicGeneralEnumTwinRustAsyncSse.apple(field: "one")],
          [BasicGeneralEnumTwinRustAsyncSse.orange()]
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicStructTwinRustAsyncSseTwinRustAsyncSse,
        <List<BasicStructTwinRustAsyncSse>>[
          [],
          [BasicStructTwinRustAsyncSse(apple: null, orange: null)],
          [BasicStructTwinRustAsyncSse(apple: "one", orange: 42)]
        ]);
  });
}
