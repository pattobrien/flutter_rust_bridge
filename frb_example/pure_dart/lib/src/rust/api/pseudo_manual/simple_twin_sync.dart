// ignore_for_file: invalid_use_of_internal_member

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

int simpleAdderTwinSync({required int a, required int b, dynamic hint}) =>
    RustLib.instance.api.simpleAdderTwinSync(a: a, b: b, hint: hint);
