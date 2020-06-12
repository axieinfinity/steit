using System;
using System.Collections.Generic;

using Steit.Codec;

namespace Steit.State {
    public static class State {
        // public static IState? GetNested(this IState? state, IEnumerable<UInt32> path) {
        public static IState GetNested(this IState state, IEnumerable<UInt32> path) {
            foreach (var tag in path) {
                state = state?.GetNested(tag);
            }

            return state;
        }

        public static void Replace(this IState state, IReader reader, bool shouldNotify = true) {
            if (state is IEnumState) {
                var variant = reader.ReadUInt32();
                state.ReplaceAt(variant, WireType.Sized, reader, shouldNotify);
                return;
            }

            while (!reader.EndOfStream()) {
                var (tag, wireType) = reader.ReadKey();
                var expectedWireType = state.GetWireType(tag);
                var fieldReader = wireType == WireType.Sized ? reader.GetNested() : reader;

                // `expectedWireType` being `null` means that `state` either:
                // (1) doesn't recognize this field;
                // (2) has no clear expectation for the field's wire type (e.g. there are multiple valid wire types); or
                // (3) doesn't support update operations.
                //
                // An exception is expected to be thrown in case (3).
                // So, if we skip the field here, that would cause an exception leak
                // and make the program continue functioning with undefined behavior.
                //
                // That's why we leave to `state.ReplaceAt()` to handle instead,
                // since `state` knows best which case will happen.
                //
                // `state.ReplaceAt()` will then handle the cases as below:
                // * Case (1): It skips the unrecognized field.
                // * Case (2) or when wire types match: It reads the field based on passed `wireType`.
                // * Case (3): It throws an exception.
                //
                // On the other hand, if `expectedWireType` is clearly defined but doesn't match with `wireType`,
                // we don't hesitate to skip the field right here.

                if (expectedWireType != null && wireType != expectedWireType) {
                    var path = state.Path.GetNested(tag);
                    Console.Error.WriteLine("Expected wire type {0} for path {1}, got {2}.", expectedWireType, path, wireType);
                    fieldReader.SkipField(wireType);
                    continue;
                }

                state.ReplaceAt(tag, wireType, fieldReader, shouldNotify);
            }
        }
    }
}
