// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.39.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../auxiliary/sample_types.dart';
import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'misc_example_twin_rust_async_sse.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `visibility_restricted_func_twin_rust_async_sse`
// These types are ignored because they are not used by any `pub` functions: `MySizeFreezedTwinRustAsyncSse`

Future<MyTreeNodeTwinRustAsyncSse> handleComplexStructTwinRustAsyncSse(
        {required MyTreeNodeTwinRustAsyncSse s}) =>
    RustLib.instance.api
        .crateApiPseudoManualMiscExampleTwinRustAsyncSseHandleComplexStructTwinRustAsyncSse(
            s: s);

Future<List<WeekdaysTwinRustAsyncSse>> listOfPrimitiveEnumsTwinRustAsyncSse(
        {required List<WeekdaysTwinRustAsyncSse> weekdays}) =>
    RustLib.instance.api
        .crateApiPseudoManualMiscExampleTwinRustAsyncSseListOfPrimitiveEnumsTwinRustAsyncSse(
            weekdays: weekdays);

Future<MyNestedStructTwinRustAsyncSse> handleNestedStructTwinRustAsyncSse(
        {required MyNestedStructTwinRustAsyncSse s}) =>
    RustLib.instance.api
        .crateApiPseudoManualMiscExampleTwinRustAsyncSseHandleNestedStructTwinRustAsyncSse(
            s: s);

Future<BigBuffersTwinRustAsyncSse> handleBigBuffersTwinRustAsyncSse() => RustLib
    .instance.api
    .crateApiPseudoManualMiscExampleTwinRustAsyncSseHandleBigBuffersTwinRustAsyncSse();

Future<AbcTwinRustAsyncSse> testAbcEnumTwinRustAsyncSse(
        {required AbcTwinRustAsyncSse abc}) =>
    RustLib.instance.api
        .crateApiPseudoManualMiscExampleTwinRustAsyncSseTestAbcEnumTwinRustAsyncSse(
            abc: abc);

Future<StructWithEnumTwinRustAsyncSse> testStructWithEnumTwinRustAsyncSse(
        {required StructWithEnumTwinRustAsyncSse se}) =>
    RustLib.instance.api
        .crateApiPseudoManualMiscExampleTwinRustAsyncSseTestStructWithEnumTwinRustAsyncSse(
            se: se);

Future<String> handleStringTwinRustAsyncSse({required String s}) => RustLib
    .instance.api
    .crateApiPseudoManualMiscExampleTwinRustAsyncSseHandleStringTwinRustAsyncSse(
        s: s);

Future<String> handleCharTwinRustAsyncSse({required String arg}) => RustLib
    .instance.api
    .crateApiPseudoManualMiscExampleTwinRustAsyncSseHandleCharTwinRustAsyncSse(
        arg: arg);

Future<Uint8List> handleVecU8TwinRustAsyncSse({required List<int> v}) => RustLib
    .instance.api
    .crateApiPseudoManualMiscExampleTwinRustAsyncSseHandleVecU8TwinRustAsyncSse(
        v: v);

Future<MySize> handleStructTwinRustAsyncSse(
        {required MySize arg, required MySize boxed}) =>
    RustLib.instance.api
        .crateApiPseudoManualMiscExampleTwinRustAsyncSseHandleStructTwinRustAsyncSse(
            arg: arg, boxed: boxed);

Future<int> positionalArgumentsTwinRustAsyncSse(int a, int b) => RustLib
    .instance.api
    .crateApiPseudoManualMiscExampleTwinRustAsyncSsePositionalArgumentsTwinRustAsyncSse(
        a: a, b: b);

class ATwinRustAsyncSse {
  final String a;

  const ATwinRustAsyncSse({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ATwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

@freezed
sealed class AbcTwinRustAsyncSse with _$AbcTwinRustAsyncSse {
  const AbcTwinRustAsyncSse._();

  const factory AbcTwinRustAsyncSse.a(
    ATwinRustAsyncSse field0,
  ) = AbcTwinRustAsyncSse_A;
  const factory AbcTwinRustAsyncSse.b(
    BTwinRustAsyncSse field0,
  ) = AbcTwinRustAsyncSse_B;
  const factory AbcTwinRustAsyncSse.c(
    CTwinRustAsyncSse field0,
  ) = AbcTwinRustAsyncSse_C;
  const factory AbcTwinRustAsyncSse.justInt(
    int field0,
  ) = AbcTwinRustAsyncSse_JustInt;
}

class BTwinRustAsyncSse {
  final int b;

  const BTwinRustAsyncSse({
    required this.b,
  });

  @override
  int get hashCode => b.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          b == other.b;
}

class BigBuffersTwinRustAsyncSse {
  final Int64List int64;
  final Uint64List uint64;

  const BigBuffersTwinRustAsyncSse({
    required this.int64,
    required this.uint64,
  });

  @override
  int get hashCode => int64.hashCode ^ uint64.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BigBuffersTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          int64 == other.int64 &&
          uint64 == other.uint64;
}

class CTwinRustAsyncSse {
  final bool c;

  const CTwinRustAsyncSse({
    required this.c,
  });

  @override
  int get hashCode => c.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          c == other.c;
}

class MyNestedStructTwinRustAsyncSse {
  final MyTreeNodeTwinRustAsyncSse treeNode;
  final WeekdaysTwinRustAsyncSse weekday;

  const MyNestedStructTwinRustAsyncSse({
    required this.treeNode,
    required this.weekday,
  });

  @override
  int get hashCode => treeNode.hashCode ^ weekday.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyNestedStructTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          treeNode == other.treeNode &&
          weekday == other.weekday;
}

class MyTreeNodeTwinRustAsyncSse {
  final int valueI32;
  final Uint8List valueVecU8;
  final bool valueBoolean;
  final List<MyTreeNodeTwinRustAsyncSse> children;

  const MyTreeNodeTwinRustAsyncSse({
    required this.valueI32,
    required this.valueVecU8,
    required this.valueBoolean,
    required this.children,
  });

  @override
  int get hashCode =>
      valueI32.hashCode ^
      valueVecU8.hashCode ^
      valueBoolean.hashCode ^
      children.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyTreeNodeTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          valueI32 == other.valueI32 &&
          valueVecU8 == other.valueVecU8 &&
          valueBoolean == other.valueBoolean &&
          children == other.children;
}

class StructWithEnumTwinRustAsyncSse {
  final AbcTwinRustAsyncSse abc1;
  final AbcTwinRustAsyncSse abc2;

  const StructWithEnumTwinRustAsyncSse({
    required this.abc1,
    required this.abc2,
  });

  @override
  int get hashCode => abc1.hashCode ^ abc2.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithEnumTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          abc1 == other.abc1 &&
          abc2 == other.abc2;
}

enum WeekdaysTwinRustAsyncSse {
  monday,
  tuesday,
  wednesday,
  thursday,
  friday,
  saturday,
  sunday,
  ;
}
