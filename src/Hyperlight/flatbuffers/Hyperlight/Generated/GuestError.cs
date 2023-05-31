// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace Hyperlight.Generated
{

using global::System;
using global::System.Collections.Generic;
using global::Google.FlatBuffers;

public struct GuestError : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_23_5_26(); }
  public static GuestError GetRootAsGuestError(ByteBuffer _bb) { return GetRootAsGuestError(_bb, new GuestError()); }
  public static GuestError GetRootAsGuestError(ByteBuffer _bb, GuestError obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public static bool VerifyGuestError(ByteBuffer _bb) {Google.FlatBuffers.Verifier verifier = new Google.FlatBuffers.Verifier(_bb); return verifier.VerifyBuffer("", false, GuestErrorVerify.Verify); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public GuestError __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public Hyperlight.Generated.ErrorCode Code { get { int o = __p.__offset(4); return o != 0 ? (Hyperlight.Generated.ErrorCode)__p.bb.GetUlong(o + __p.bb_pos) : Hyperlight.Generated.ErrorCode.NoError; } }
  public string Message { get { int o = __p.__offset(6); return o != 0 ? __p.__string(o + __p.bb_pos) : null; } }
#if ENABLE_SPAN_T
  public Span<byte> GetMessageBytes() { return __p.__vector_as_span<byte>(6, 1); }
#else
  public ArraySegment<byte>? GetMessageBytes() { return __p.__vector_as_arraysegment(6); }
#endif
  public byte[] GetMessageArray() { return __p.__vector_as_array<byte>(6); }

  public static Offset<Hyperlight.Generated.GuestError> CreateGuestError(FlatBufferBuilder builder,
      Hyperlight.Generated.ErrorCode code = Hyperlight.Generated.ErrorCode.NoError,
      StringOffset messageOffset = default(StringOffset)) {
    builder.StartTable(2);
    GuestError.AddCode(builder, code);
    GuestError.AddMessage(builder, messageOffset);
    return GuestError.EndGuestError(builder);
  }

  public static void StartGuestError(FlatBufferBuilder builder) { builder.StartTable(2); }
  public static void AddCode(FlatBufferBuilder builder, Hyperlight.Generated.ErrorCode code) { builder.AddUlong(0, (ulong)code, 0); }
  public static void AddMessage(FlatBufferBuilder builder, StringOffset messageOffset) { builder.AddOffset(1, messageOffset.Value, 0); }
  public static Offset<Hyperlight.Generated.GuestError> EndGuestError(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Hyperlight.Generated.GuestError>(o);
  }
  public static void FinishGuestErrorBuffer(FlatBufferBuilder builder, Offset<Hyperlight.Generated.GuestError> offset) { builder.Finish(offset.Value); }
  public static void FinishSizePrefixedGuestErrorBuffer(FlatBufferBuilder builder, Offset<Hyperlight.Generated.GuestError> offset) { builder.FinishSizePrefixed(offset.Value); }
}


static public class GuestErrorVerify
{
  static public bool Verify(Google.FlatBuffers.Verifier verifier, uint tablePos)
  {
    return verifier.VerifyTableStart(tablePos)
      && verifier.VerifyField(tablePos, 4 /*Code*/, 8 /*Hyperlight.Generated.ErrorCode*/, 8, false)
      && verifier.VerifyString(tablePos, 6 /*Message*/, false)
      && verifier.VerifyTableEnd(tablePos);
  }
}

}
