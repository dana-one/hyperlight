// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace Hyperlight.Generated
{

using global::System;
using global::System.Collections.Generic;
using global::Google.FlatBuffers;

public struct hlulong : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_23_5_26(); }
  public static hlulong GetRootAshlulong(ByteBuffer _bb) { return GetRootAshlulong(_bb, new hlulong()); }
  public static hlulong GetRootAshlulong(ByteBuffer _bb, hlulong obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public hlulong __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public ulong Value { get { int o = __p.__offset(4); return o != 0 ? __p.bb.GetUlong(o + __p.bb_pos) : (ulong)0; } }

  public static Offset<Hyperlight.Generated.hlulong> Createhlulong(FlatBufferBuilder builder,
      ulong value = 0) {
    builder.StartTable(1);
    hlulong.AddValue(builder, value);
    return hlulong.Endhlulong(builder);
  }

  public static void Starthlulong(FlatBufferBuilder builder) { builder.StartTable(1); }
  public static void AddValue(FlatBufferBuilder builder, ulong value) { builder.AddUlong(0, value, 0); }
  public static Offset<Hyperlight.Generated.hlulong> Endhlulong(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Hyperlight.Generated.hlulong>(o);
  }
  public hlulongT UnPack() {
    var _o = new hlulongT();
    this.UnPackTo(_o);
    return _o;
  }
  public void UnPackTo(hlulongT _o) {
    _o.Value = this.Value;
  }
  public static Offset<Hyperlight.Generated.hlulong> Pack(FlatBufferBuilder builder, hlulongT _o) {
    if (_o == null) return default(Offset<Hyperlight.Generated.hlulong>);
    return Createhlulong(
      builder,
      _o.Value);
  }
}

public class hlulongT
{
  public ulong Value { get; set; }

  public hlulongT() {
    this.Value = 0;
  }
}


static public class hlulongVerify
{
  static public bool Verify(Google.FlatBuffers.Verifier verifier, uint tablePos)
  {
    return verifier.VerifyTableStart(tablePos)
      && verifier.VerifyField(tablePos, 4 /*Value*/, 8 /*ulong*/, 8, false)
      && verifier.VerifyTableEnd(tablePos);
  }
}

}
