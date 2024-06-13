// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.39.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'event_listener_twin_rust_async.freezed.dart';

Future<
    Stream<
        EventTwinRustAsync>> registerEventListenerTwinRustAsync() => RustLib
    .instance.api
    .crateApiPseudoManualEventListenerTwinRustAsyncRegisterEventListenerTwinRustAsync();

Future<void> closeEventListenerTwinRustAsync() => RustLib.instance.api
    .crateApiPseudoManualEventListenerTwinRustAsyncCloseEventListenerTwinRustAsync();

Future<void> createEventTwinRustAsync(
        {required String address, required String payload}) =>
    RustLib.instance.api
        .crateApiPseudoManualEventListenerTwinRustAsyncCreateEventTwinRustAsync(
            address: address, payload: payload);

@freezed
class EventTwinRustAsync with _$EventTwinRustAsync {
  const EventTwinRustAsync._();
  const factory EventTwinRustAsync({
    required String address,
    required String payload,
  }) = _EventTwinRustAsync;
  Future<String> asStringTwinRustAsync() => RustLib.instance.api
          .crateApiPseudoManualEventListenerTwinRustAsyncEventTwinRustAsyncAsStringTwinRustAsync(
        that: this,
      );
}
