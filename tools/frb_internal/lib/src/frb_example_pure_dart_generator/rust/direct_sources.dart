import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';

Map<String, String> generateRustDirectSources() {
  return {
    'pseudo_manual/primitive.rs': _generatePrimitive(),
    'pseudo_manual/optional_primitive.rs': _generateOptionalPrimitive(),
    'pseudo_manual/primitive_list.rs': _generatePrimitiveList(),
    'pseudo_manual/structure.rs': _generateStructure(),
  };
}

String _generatePrimitive() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    final name = ty.name;
    builder.body += 'pub fn example_primitive_type_${name}_twin_normal(arg: $name) -> $name { arg }\n';
  }
  return builder.toString();
}

String _generatePrimitiveList() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    final name = ty.name;
    final listType = 'Vec<$name>';
    builder.body += 'pub fn example_primitive_list_type_${name}_twin_normal(arg: $listType) -> $listType { arg }\n';
  }
  return builder.toString();
}

String _generateOptionalPrimitive() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    final name = ty.name;
    builder.body +=
        'pub fn example_optional_primitive_type_${name}_twin_normal(arg: Option<$name>) -> Option<$name> { arg }\n';
  }
  return builder.toString();
}

String _generateStructure() {
  final builder = RustFileBuilder();

  builder.body += '''
pub struct StructSimpleTwinNormal {
    pub field: i32,
}

#[frb(dart_metadata = ("freezed"))]
pub struct StructFreezedTwinNormal {
    pub field: i32,
}

pub fn func_struct_simple_twin_normal(arg: StructSimpleTwinNormal) -> StructSimpleTwinNormal {
    arg
}

pub fn func_struct_simple_boxed_twin_normal(
    arg: Box<StructSimpleTwinNormal>,
) -> Box<StructSimpleTwinNormal> {
    arg
}
  ''';

  return builder.toString();
}

class PrimitiveTypeInfo {
  final String name;
  final List<String> interestValues;

  const PrimitiveTypeInfo(this.name, this.interestValues);
}

const kPrimitiveTypes = [
  PrimitiveTypeInfo('i8', ['0', '-128', '127']),
  PrimitiveTypeInfo('i16', ['0', '-32768', '32767']),
  PrimitiveTypeInfo('i32', ['0', '-2147483648', '2147483647']),
  PrimitiveTypeInfo(
      'i64', ['BigInt.parse("0")', 'BigInt.parse("-9223372036854775808")', 'BigInt.parse("9223372036854775807")']),
  PrimitiveTypeInfo('u8', ['0', '255']),
  PrimitiveTypeInfo('u16', ['0', '65535']),
  PrimitiveTypeInfo('u32', ['0', '4294967295']),
  PrimitiveTypeInfo('u64', [
    'BigInt.parse("0")',
    // 'BigInt.parse("18446744073709551615")', // not support numbers bigger than max i64 yet (but implementable)
    'BigInt.parse("9223372036854775807")',
  ]),
  PrimitiveTypeInfo('f32', ['0', '-42.5', '123456']),
  PrimitiveTypeInfo('f64', ['0', '-42.5', '123456']),
  PrimitiveTypeInfo('bool', ['false', 'true']),
];
