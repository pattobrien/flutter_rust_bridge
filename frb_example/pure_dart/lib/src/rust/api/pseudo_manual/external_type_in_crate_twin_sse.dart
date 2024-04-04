// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.30.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../auxiliary/new_module_system/sub_module.dart';
import '../../auxiliary/old_module_system/sub_module.dart';
import '../../auxiliary/sample_types.dart';
import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<bool> useImportedStructTwinSse(
        {required MyStruct myStruct, dynamic hint}) =>
    RustLib.instance.api
        .useImportedStructTwinSse(myStruct: myStruct, hint: hint);

Future<bool> useImportedEnumTwinSse({required MyEnum myEnum, dynamic hint}) =>
    RustLib.instance.api.useImportedEnumTwinSse(myEnum: myEnum, hint: hint);

Future<OldSimpleStruct> callOldModuleSystemTwinSse({dynamic hint}) =>
    RustLib.instance.api.callOldModuleSystemTwinSse(hint: hint);

Future<NewSimpleStruct> callNewModuleSystemTwinSse({dynamic hint}) =>
    RustLib.instance.api.callNewModuleSystemTwinSse(hint: hint);
