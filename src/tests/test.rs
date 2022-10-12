use crate::CPU;
use crate::memory::ROMSpace;

#[test]
fn ldax_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x0a);
    c.bus.write_byte(0x100, 0x65);
    c.registers.set_bc(0x100);
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x65);
}

#[test]
fn ldax_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x1a);
    c.bus.write_byte(0x100, 0x65);
    c.registers.set_de(0x100);
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x65);
}

#[test]
fn lxi_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x01);
    c.bus.write_byte(0x0001, 0x12);
    c.bus.write_byte(0x0002, 0x34);
    c.execute();
    assert_eq!(c.pc, 0x0003);
    assert_eq!(c.registers.b, 0x34);
    assert_eq!(c.registers.c, 0x12);
}

#[test]
fn lxi_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x11);
    c.bus.write_byte(0x0001, 0x12);
    c.bus.write_byte(0x0002, 0x34);
    c.execute();
    assert_eq!(c.pc, 0x0003);
    assert_eq!(c.registers.d, 0x34);
    assert_eq!(c.registers.e, 0x12);
}

#[test]
fn lxi_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x21);
    c.bus.write_byte(0x0001, 0x12);
    c.bus.write_byte(0x0002, 0x34);
    c.execute();
    assert_eq!(c.pc, 0x0003);
    assert_eq!(c.registers.h, 0x34);
    assert_eq!(c.registers.l, 0x12);
}

#[test]
fn lxi_sp() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x31);
    c.bus.write_byte(0x0001, 0x12);
    c.bus.write_byte(0x0002, 0x34);
    c.execute();
    assert_eq!(c.pc, 0x0003);
    assert_eq!(c.sp, 0x3412);
}

#[test]
fn sta() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x32);
    c.bus.write_byte(0x0001, 0x00);
    c.bus.write_byte(0x0002, 0xff);
    c.registers.a = 0x56;
    c.execute();
    assert_eq!(c.pc, 0x0003);
    assert_eq!(c.bus.read_byte(0xff00), 0x56);
}

#[test]
fn lda() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x3a);
    c.bus.write_byte(0x0001, 0x00);
    c.bus.write_byte(0x0002, 0xff);
    c.bus.write_byte(0xff00, 0x56);
    c.execute();
    assert_eq!(c.pc, 0x0003);
    assert_eq!(c.registers.a, 0x56);
}

#[test]
fn stax_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x02);
    c.registers.a = 0x49;
    c.registers.set_bc(0x1234);
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(c.bus.read_byte(0x1234), 0x49);
}

#[test]
fn stax_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x12);
    c.registers.a = 0x49;
    c.registers.set_de(0x1234);
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(c.bus.read_byte(0x1234), 0x49);
}

#[test]
fn inx_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x03);
    c.registers.set_bc(0x1234);
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(c.registers.get_bc(), 0x1235);
}

#[test]
fn inx_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x13);
    c.registers.set_de(0x1234);
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(c.registers.get_de(), 0x1235);
}

#[test]
fn inx_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x23);
    c.registers.a = 0x49;
    c.registers.set_hl(0x1234);
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(c.registers.get_hl(), 0x1235);
}

#[test]
fn inx_sp() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x33);
    c.sp = 0x0049;
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(c.sp, 0x004A);
}

#[test]
fn cmc() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x3f);
    c.bus.write_byte(0x0001, 0x3f);
    c.execute();
    assert_eq!(true, c.flags.c);
    assert_eq!(c.pc, 0x0001);
    c.execute();
    assert_eq!(false, c.flags.c);
    assert_eq!(c.pc, 0x0002);
}

#[test]
fn stc() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x37);
    c.bus.write_byte(0x0001, 0x37);
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(true, c.flags.c);
    c.execute();
    assert_eq!(c.pc, 0x0002);
    assert_eq!(true, c.flags.c);
}

#[test]
fn inrb() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x04);
    c.registers.b = 0xff;
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(0, c.registers.b);
    assert_eq!(true, c.flags.z);
}

#[test]
fn inrc() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x0C);
    c.registers.c = 0xff;
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(0, c.registers.c);
    assert_eq!(true, c.flags.z);
}

#[test]
fn inrd() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x14);
    c.registers.d = 0xff;
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(0, c.registers.d);
    assert_eq!(true, c.flags.z);
}

#[test]
fn inre() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x1C);
    c.registers.e = 0xff;
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(0, c.registers.e);
    assert_eq!(true, c.flags.z);
}

#[test]
fn inrh() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x24);
    c.registers.h = 0xff;
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(0, c.registers.h);
    assert_eq!(true, c.flags.z);
}

#[test]
fn inrl() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x2C);
    c.registers.l = 0xff;
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(0, c.registers.l);
    assert_eq!(true, c.flags.z);
}

#[test]
fn inrm() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x34);
    c.bus.write_byte(0x0001, 0x34);
    c.bus.write_byte(0x100, 0xff);
    c.registers.set_hl(0x100);
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(0, c.bus.read_byte(0x100));
    assert_eq!(true, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 0x0002);
    assert_eq!(1, c.bus.read_byte(0x100));
    assert_eq!(false, c.flags.z);
}

#[test]
fn inra() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x3C);
    c.registers.a = 0x0f;
    c.execute();
    assert_eq!(c.pc, 0x0001);
    assert_eq!(0x10, c.registers.a);
    assert_eq!(false, c.flags.z);
    assert_eq!(true, c.flags.a);
}

#[test]
fn dcr_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x05);
    c.bus.write_byte(0x0001, 0x05);
    c.registers.b = 0x01;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0, c.registers.b);
    assert_eq!(true, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(0xff, c.registers.b);
    assert_eq!(false, c.flags.z);
}

#[test]
fn dcr_c() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x0d);
    c.bus.write_byte(0x0001, 0x0d);
    c.registers.c = 0x01;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0, c.registers.c);
    assert_eq!(true, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(0xff, c.registers.c);
    assert_eq!(false, c.flags.z);
}

#[test]
fn dcr_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x15);
    c.bus.write_byte(0x0001, 0x15);
    c.registers.d = 0x01;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0, c.registers.d);
    assert_eq!(true, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(0xff, c.registers.d);
    assert_eq!(false, c.flags.z);
}

#[test]
fn dcr_e() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x1d);
    c.bus.write_byte(0x0001, 0x1d);
    c.registers.e = 0x01;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0, c.registers.e);
    assert_eq!(true, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(0xff, c.registers.e);
    assert_eq!(false, c.flags.z);
}

#[test]
fn dcr_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x25);
    c.bus.write_byte(0x0001, 0x25);
    c.registers.h = 0x01;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0, c.registers.h);
    assert_eq!(true, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(0xff, c.registers.h);
    assert_eq!(false, c.flags.z);
}

#[test]
fn dcr_l() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x2d);
    c.bus.write_byte(0x0001, 0x2d);
    c.registers.l = 0x01;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0, c.registers.l);
    assert_eq!(true, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(0xff, c.registers.l);
    assert_eq!(false, c.flags.z);
}

#[test]
fn dcr_m() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x35);
    c.bus.write_byte(0x0001, 0x35);
    c.bus.write_byte(0x100, 0x55);
    c.registers.set_hl(0x0100);
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0x54, c.bus.read_byte(0x0100));
    assert_eq!(false, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(0x53, c.bus.read_byte(0x0100));
    assert_eq!(false, c.flags.z);
}

#[test]
fn dcr_a() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x3d);
    c.bus.write_byte(0x0001, 0x3d);
    c.registers.a = 0x01;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0, c.registers.a);
    assert_eq!(true, c.flags.z);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(0xff, c.registers.a);
    assert_eq!(false, c.flags.z);
}

#[test]
fn cma() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x2F);
    c.registers.a = 0b11001100;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0b00110011, c.registers.a);
    
}

#[test]
fn add() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x82);
    c.registers.a = 0x6C;
    c.registers.d = 0x2E;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0x9A, c.registers.a);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.flags.p, true);
    assert_eq!(c.flags.s, true);
    assert_eq!(c.flags.a, true);
}

#[test]
fn adc() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x89);
    c.registers.a = 0x42;
    c.registers.c = 0x3D;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0x7F, c.registers.a);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.flags.p, false);
    assert_eq!(c.flags.s, false);
    assert_eq!(c.flags.a, false);
}

#[test]
fn sub() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x97);
    c.registers.a = 0x3E;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0x00, c.registers.a);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.flags.p, true);
    assert_eq!(c.flags.s, false);
    assert_eq!(c.flags.a, true);
}

#[test]
fn sbb() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x9D);
    c.registers.a = 0x04;
    c.flags.c = true;
    c.registers.l = 0x02;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0x01, c.registers.a);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.flags.p, false);
    assert_eq!(c.flags.s, false);
    assert_eq!(c.flags.a, true);
}

#[test]
fn ana() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xA1);
    c.registers.a = 0xFC;
    c.registers.c = 0x0F;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0x0C, c.registers.a);
}

#[test]
fn ora() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xB1);
    c.registers.a = 0x33;
    c.registers.c = 0x0F;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0x3F, c.registers.a);
}

#[test]
fn cmp() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xBB);
    c.registers.a = 0x0A;
    c.registers.e = 0x05;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(0x0A, c.registers.a);
    assert_eq!(0x05, c.registers.e);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn rlc() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x07);
    c.registers.a = 0xF2;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0xE5);
}

#[test]
fn rrc() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x0F);
    c.registers.a = 0xF2;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x79);
}

#[test]
fn ral() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x17);
    c.registers.a = 0xB5;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0x6A);
}

#[test]
fn rar() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x1F);
    c.registers.a = 0x6A;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0xB5);
}

#[test]
fn push() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xD5);
    c.registers.d = 0x8F;
    c.registers.e = 0x9D;
    c.sp = 0x3A2C;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.sp, 0x3A2A);
    assert_eq!(c.bus.read_byte(0x3A2B), 0x8F);
    assert_eq!(c.bus.read_byte(0x3A2A), 0x9D);
}

#[test]
fn push_psw() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xF5);
    c.registers.a = 0x1F;
    c.flags.c = true;
    c.flags.z = true;
    c.flags.p = true;
    c.flags.s = false;
    c.flags.a = false;
    c.sp = 0x502A;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.sp, 0x5028);
    assert_eq!(c.bus.read_byte(0x5029), 0x1F);
    assert_eq!(c.bus.read_byte(0x5028), 0x47);
}

#[test]
fn pop() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xE1);
    c.bus.write_byte(0x1239, 0x3D);
    c.bus.write_byte(0x123A, 0x93);
    c.sp = 0x1239;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.sp, 0x123B);
    assert_eq!(c.registers.l, 0x3D);
    assert_eq!(c.registers.h, 0x93);
}

#[test]
fn pop_psw() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xF1);
    c.bus.write_byte(0x2C00, 0xC3);
    c.bus.write_byte(0x2C01, 0xFF);
    c.sp = 0x2C00;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0xFF);
    assert_eq!(c.flags.s, true);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.p, false);
}

#[test]
fn dad_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x09);
    c.registers.set_bc(0x339F);
    c.registers.set_hl(0xA17B);
    c.execute();
    assert_eq!(c.registers.h, 0xD5);
    assert_eq!(c.registers.l, 0x1A);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.pc, 1);
}

#[test]
fn dad_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x19);
    c.registers.set_de(0x339F);
    c.registers.set_hl(0xA17B);
    c.execute();
    assert_eq!(c.registers.h, 0xD5);
    assert_eq!(c.registers.l, 0x1A);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.pc, 1);
}

#[test]
fn dad_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x29);
    c.registers.set_hl(0x339F);
    c.execute();
    assert_eq!(c.registers.h, 0x67);
    assert_eq!(c.registers.l, 0x3e);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.pc, 1);
}

#[test]
fn dad_sp() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x39);
    c.sp = 0x339F;
    c.registers.set_hl(0xA17B);
    c.execute();
    assert_eq!(c.registers.h, 0xD5);
    assert_eq!(c.registers.l, 0x1A);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.pc, 1);
}

#[test]
fn dcx_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x0b);
    c.registers.set_bc(0);
    c.execute();
    assert_eq!(c.registers.get_bc(), 0xffff);
    assert_eq!(c.pc, 1);
}

#[test]
fn dcx_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x1b);
    c.registers.set_de(0);
    c.execute();
    assert_eq!(c.registers.get_de(), 0xffff);
    assert_eq!(c.pc, 1);
}

#[test]
fn dcx_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x2b);
    c.registers.set_hl(0);
    c.execute();
    assert_eq!(c.registers.get_hl(), 0xffff);
    assert_eq!(c.pc, 1);
}

#[test]
fn dcx_sp() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x3b);
    c.sp = 0xFFFF;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.sp, 0xFFFE);
}

#[test]
fn xchg() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xeb);
    c.registers.set_de(0x3355);
    c.registers.set_hl(0x00FF);
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.get_de(), 0x00FF);
    assert_eq!(c.registers.get_hl(), 0x3355);
}

#[test]
fn xthl() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xe3);
    c.sp = 0x10AD;
    c.registers.set_hl(0x0B3C);
    c.bus.write_byte(0x10ad, 0xF0);
    c.bus.write_byte(0x10ae, 0x0d);
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.get_hl(), 0x0df0);
    assert_eq!(c.bus.read_byte(0x10ad), 0x3c);
    assert_eq!(c.bus.read_byte(0x10ae), 0x0b);
}

#[test]
fn mvi_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x06);
    c.bus.write_byte(0x0001, 0x88);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.b, 0x88);
}

#[test]
fn mvi_c() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x0e);
    c.bus.write_byte(0x0001, 0x88);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.c, 0x88);
}

#[test]
fn mvi_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x16);
    c.bus.write_byte(0x0001, 0x88);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.d, 0x88);
}

#[test]
fn mvi_e() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x1e);
    c.bus.write_byte(0x0001, 0x88);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.e, 0x88);
}

#[test]
fn mvi_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x26);
    c.bus.write_byte(0x0001, 0x88);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.h, 0x88);
}

#[test]
fn mvi_l() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x2e);
    c.bus.write_byte(0x0001, 0x88);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.l, 0x88);
}

#[test]
fn mvi_m() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x36);
    c.bus.write_byte(0x0001, 0x88);
    c.registers.set_hl(0x100);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.bus.read_byte(0x100), 0x88);
}

#[test]
fn mvi_a() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x3e);
    c.bus.write_byte(0x0001, 0x88);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.a, 0x88);
}

#[test]
fn adi() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xc6);
    c.bus.write_byte(0x0001, 0x42);
    c.registers.a = 0x14;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.a, 0x56);
    assert_eq!(c.flags.p, true);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.s, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn aci() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xce);
    c.bus.write_byte(0x0001, 0xbe);
    c.bus.write_byte(0x0002, 0xce);
    c.bus.write_byte(0x0003, 0x42);
    c.registers.a = 0x56;
    c.flags.c = false;
    c.execute();
    c.execute();
    assert_eq!(c.pc, 4);
    assert_eq!(c.registers.a, 0x57);
    assert_eq!(c.flags.p, false);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.s, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sui() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xd6);
    c.bus.write_byte(0x0001, 0x01);
    c.registers.a = 0x00;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.a, 0xFF);
    assert_eq!(c.flags.p, true);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.s, true);
    assert_eq!(c.flags.c, true);
}

#[test]
fn sbi() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xaf);
    c.bus.write_byte(0x0001, 0xde);
    c.bus.write_byte(0x0002, 0x01);
    c.execute();
    c.execute();
    assert_eq!(c.pc, 3);
    assert_eq!(c.registers.a, 0xFF);
    assert_eq!(c.flags.p, true);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.s, true);
    assert_eq!(c.flags.c, true);
}

#[test]
fn ani() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x79);
    c.bus.write_byte(0x0001, 0xe6);
    c.bus.write_byte(0x0002, 0x0f);
    c.registers.c = 0x3a;
    c.execute();
    c.execute();
    assert_eq!(c.pc, 3);
    assert_eq!(c.registers.a, 0x0a);
    assert_eq!(c.flags.p, true);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.s, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn xri() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xee);
    c.bus.write_byte(0x0001, 0x81);
    c.registers.a = 0x3b;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.registers.a, 0b1011_1010);
}

#[test]
fn ori() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x79);
    c.bus.write_byte(0x0001, 0xf6);
    c.bus.write_byte(0x0002, 0x0f);
    c.registers.c = 0xb5;
    c.execute();
    c.execute();
    assert_eq!(c.pc, 3);
    assert_eq!(c.registers.a, 0xbf);
}

#[test]
fn cpi() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x3e);
    c.bus.write_byte(0x0001, 0x4a);
    c.bus.write_byte(0x0002, 0xfe);
    c.bus.write_byte(0x0003, 0x40);
    c.execute();
    c.execute();
    assert_eq!(c.pc, 4);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.flags.z, false);
}

#[test]
fn shld() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x22);
    c.bus.write_byte(0x0001, 0x0a);
    c.bus.write_byte(0x0002, 0x01);
    c.registers.set_hl(0xae29);
    c.execute();
    assert_eq!(c.pc, 3);
    assert_eq!(c.bus.read_word(0x010a), 0xae29);
}

#[test]
fn lhld() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x2a);
    c.bus.write_byte(0x0001, 0x5b);
    c.bus.write_byte(0x0002, 0x02);
    c.bus.write_byte(0x025b, 0xff);
    c.bus.write_byte(0x025c, 0x03);
    c.execute();
    assert_eq!(c.pc, 3);
    assert_eq!(c.registers.l, 0xff);
    assert_eq!(c.registers.h, 0x03);
}

#[test]
fn pchl() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xe9);
    c.registers.h = 0x41;
    c.registers.l = 0x3e;
    c.execute();
    assert_eq!(c.pc, 0x413e);
}

#[test]
fn jmp() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xc3);
    c.bus.write_byte(0x0001, 0x00);
    c.bus.write_byte(0x0002, 0x3e);
    c.execute();
    assert_eq!(c.pc, 0x3e00);
}

#[test]
fn daa() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x27);
    c.registers.a = 0x9B;
    c.flags.a = false;
    c.flags.c = false;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 1);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, true);
}

#[test]
fn sphl() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xf9);
    c.registers.h = 0x50;
    c.registers.l = 0x6c;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.sp, 0x506c)
}

#[test]
fn nop() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x00);
    c.execute();
    assert_eq!(c.pc, 1);
}

#[test]
fn mov_b() {
    let mut c = CPU::new();
    c.registers.b = 0x11;
    c.registers.c = 0x15;
    c.registers.d = 0x1F;
    c.registers.e = 0x21;
    c.registers.h = 0x25;
    c.registers.l = 0x2F;
    c.bus.write_byte(0x252f, 0x31);
    c.registers.a = 0x3F;
    c.bus.write_byte(0x0000, 0x40);
    c.bus.write_byte(0x0001, 0x41);
    c.bus.write_byte(0x0002, 0x42);
    c.bus.write_byte(0x0003, 0x43);
    c.bus.write_byte(0x0004, 0x44);
    c.bus.write_byte(0x0005, 0x45);
    c.bus.write_byte(0x0006, 0x46);
    c.bus.write_byte(0x0007, 0x47);
    c.execute();
    assert_eq!(c.registers.b, 0x11);
    c.execute();
    assert_eq!(c.registers.b, 0x15);
    c.execute();
    assert_eq!(c.registers.b, 0x1f);
    c.execute();
    assert_eq!(c.registers.b, 0x21);
    c.execute();
    assert_eq!(c.registers.b, 0x25);
    c.execute();
    assert_eq!(c.registers.b, 0x2f);
    c.execute();
    assert_eq!(c.registers.b, 0x31);
    c.execute();
    assert_eq!(c.registers.b, 0x3f);
    assert_eq!(c.pc, 8);
}

#[test]
fn mov_c() {
    let mut c = CPU::new();
    c.registers.b = 0x11;
    c.registers.c = 0x15;
    c.registers.d = 0x1F;
    c.registers.e = 0x21;
    c.registers.h = 0x25;
    c.registers.l = 0x2F;
    c.bus.write_byte(0x252f, 0x31);
    c.registers.a = 0x3F;
    c.bus.write_byte(0x0000, 0x48);
    c.bus.write_byte(0x0001, 0x49);
    c.bus.write_byte(0x0002, 0x4a);
    c.bus.write_byte(0x0003, 0x4b);
    c.bus.write_byte(0x0004, 0x4c);
    c.bus.write_byte(0x0005, 0x4d);
    c.bus.write_byte(0x0006, 0x4e);
    c.bus.write_byte(0x0007, 0x4f);
    c.execute();
    assert_eq!(c.registers.c, 0x11);
    c.execute();
    assert_eq!(c.registers.c, 0x11);
    c.execute();
    assert_eq!(c.registers.c, 0x1f);
    c.execute();
    assert_eq!(c.registers.c, 0x21);
    c.execute();
    assert_eq!(c.registers.c, 0x25);
    c.execute();
    assert_eq!(c.registers.c, 0x2f);
    c.execute();
    assert_eq!(c.registers.c, 0x31);
    c.execute();
    assert_eq!(c.registers.c, 0x3f);
    assert_eq!(c.pc, 8);
}

#[test]
fn mov_d() {
    let mut c = CPU::new();
    c.registers.b = 0x11;
    c.registers.c = 0x15;
    c.registers.d = 0x1F;
    c.registers.e = 0x21;
    c.registers.h = 0x25;
    c.registers.l = 0x2F;
    c.bus.write_byte(0x252f, 0x31);
    c.registers.a = 0x3F;
    c.bus.write_byte(0x0000, 0x50);
    c.bus.write_byte(0x0001, 0x51);
    c.bus.write_byte(0x0002, 0x52);
    c.bus.write_byte(0x0003, 0x53);
    c.bus.write_byte(0x0004, 0x54);
    c.bus.write_byte(0x0005, 0x55);
    c.bus.write_byte(0x0006, 0x56);
    c.bus.write_byte(0x0007, 0x57);
    c.execute();
    assert_eq!(c.registers.d, 0x11);
    c.execute();
    assert_eq!(c.registers.d, 0x15);
    c.execute();
    assert_eq!(c.registers.d, 0x15);
    c.execute();
    assert_eq!(c.registers.d, 0x21);
    c.execute();
    assert_eq!(c.registers.d, 0x25);
    c.execute();
    assert_eq!(c.registers.d, 0x2f);
    c.execute();
    assert_eq!(c.registers.d, 0x31);
    c.execute();
    assert_eq!(c.registers.d, 0x3f);
    assert_eq!(c.pc, 8);
}

#[test]
fn mov_e() {
    let mut c = CPU::new();
    c.registers.b = 0x11;
    c.registers.c = 0x15;
    c.registers.d = 0x1F;
    c.registers.e = 0x21;
    c.registers.h = 0x25;
    c.registers.l = 0x2F;
    c.bus.write_byte(0x252f, 0x31);
    c.registers.a = 0x3F;
    c.bus.write_byte(0x0000, 0x58);
    c.bus.write_byte(0x0001, 0x59);
    c.bus.write_byte(0x0002, 0x5a);
    c.bus.write_byte(0x0003, 0x5b);
    c.bus.write_byte(0x0004, 0x5c);
    c.bus.write_byte(0x0005, 0x5d);
    c.bus.write_byte(0x0006, 0x5e);
    c.bus.write_byte(0x0007, 0x5f);
    c.execute();
    assert_eq!(c.registers.e, 0x11);
    c.execute();
    assert_eq!(c.registers.e, 0x15);
    c.execute();
    assert_eq!(c.registers.e, 0x1f);
    c.execute();
    assert_eq!(c.registers.e, 0x1f);
    c.execute();
    assert_eq!(c.registers.e, 0x25);
    c.execute();
    assert_eq!(c.registers.e, 0x2f);
    c.execute();
    assert_eq!(c.registers.e, 0x31);
    c.execute();
    assert_eq!(c.registers.e, 0x3f);
    assert_eq!(c.pc, 8);
}

#[test]
fn mov_h() {
    let mut c = CPU::new();
    c.registers.b = 0x11;
    c.registers.c = 0x15;
    c.registers.d = 0x1F;
    c.registers.e = 0x21;
    c.registers.h = 0x25;
    c.registers.l = 0x2F;
    c.bus.write_byte(0x2f2f, 0x31);
    c.registers.a = 0x3F;
    c.bus.write_byte(0x0000, 0x60);
    c.bus.write_byte(0x0001, 0x61);
    c.bus.write_byte(0x0002, 0x62);
    c.bus.write_byte(0x0003, 0x63);
    c.bus.write_byte(0x0004, 0x64);
    c.bus.write_byte(0x0005, 0x65);
    c.bus.write_byte(0x0006, 0x66);
    c.bus.write_byte(0x0007, 0x67);
    c.execute();
    assert_eq!(c.registers.h, 0x11);
    c.execute();
    assert_eq!(c.registers.h, 0x15);
    c.execute();
    assert_eq!(c.registers.h, 0x1f);
    c.execute();
    assert_eq!(c.registers.h, 0x21);
    c.execute();
    assert_eq!(c.registers.h, 0x21);
    c.execute();
    assert_eq!(c.registers.h, 0x2f);
    c.execute();
    assert_eq!(c.registers.h, 0x31);
    c.execute();
    assert_eq!(c.registers.h, 0x3f);
    assert_eq!(c.pc, 8);
}

#[test]
fn mov_l() {
    let mut c = CPU::new();
    c.registers.b = 0x11;
    c.registers.c = 0x15;
    c.registers.d = 0x1F;
    c.registers.e = 0x21;
    c.registers.h = 0x25;
    c.registers.l = 0x2F;
    c.bus.write_byte(0x2525, 0x31);
    c.registers.a = 0x3F;
    c.bus.write_byte(0x0000, 0x68);
    c.bus.write_byte(0x0001, 0x69);
    c.bus.write_byte(0x0002, 0x6a);
    c.bus.write_byte(0x0003, 0x6b);
    c.bus.write_byte(0x0004, 0x6c);
    c.bus.write_byte(0x0005, 0x6d);
    c.bus.write_byte(0x0006, 0x6e);
    c.bus.write_byte(0x0007, 0x6f);
    c.execute();
    assert_eq!(c.registers.l, 0x11);
    c.execute();
    assert_eq!(c.registers.l, 0x15);
    c.execute();
    assert_eq!(c.registers.l, 0x1f);
    c.execute();
    assert_eq!(c.registers.l, 0x21);
    c.execute();
    assert_eq!(c.registers.l, 0x25);
    c.execute();
    assert_eq!(c.registers.l, 0x25);
    c.execute();
    assert_eq!(c.registers.l, 0x31);
    c.execute();
    assert_eq!(c.registers.l, 0x3f);
    assert_eq!(c.pc, 8);
}

#[test]
fn mov_m() {
    let mut c = CPU::new();
    c.registers.b = 0x11;
    c.registers.c = 0x15;
    c.registers.d = 0x1F;
    c.registers.e = 0x21;
    c.registers.h = 0x25;
    c.registers.l = 0x2F;
    c.bus.write_byte(0x2f2f, 0x31);
    c.registers.a = 0x3F;
    c.bus.write_byte(0x0000, 0x70);
    c.bus.write_byte(0x0001, 0x71);
    c.bus.write_byte(0x0002, 0x72);
    c.bus.write_byte(0x0003, 0x73);
    c.bus.write_byte(0x0004, 0x74);
    c.bus.write_byte(0x0005, 0x75);
    c.bus.write_byte(0x0006, 0x77);
    c.execute();
    assert_eq!(c.bus.read_byte(0x252f), 0x11);
    c.execute();
    assert_eq!(c.bus.read_byte(0x252f), 0x15);
    c.execute();
    assert_eq!(c.bus.read_byte(0x252f), 0x1f);
    c.execute();
    assert_eq!(c.bus.read_byte(0x252f), 0x21);
    c.execute();
    assert_eq!(c.bus.read_byte(0x252f), 0x25);
    c.execute();
    assert_eq!(c.bus.read_byte(0x252f), 0x2f);
    c.execute();
    assert_eq!(c.bus.read_byte(0x252f), 0x3f);
    assert_eq!(c.pc, 7);
}

#[test]
fn mov_a() {
    let mut c = CPU::new();
    c.registers.b = 0x11;
    c.registers.c = 0x15;
    c.registers.d = 0x1F;
    c.registers.e = 0x21;
    c.registers.h = 0x25;
    c.registers.l = 0x2F;
    c.bus.write_byte(0x252f, 0x31);
    c.registers.a = 0x3F;
    c.bus.write_byte(0x0000, 0x78);
    c.bus.write_byte(0x0001, 0x79);
    c.bus.write_byte(0x0002, 0x7a);
    c.bus.write_byte(0x0003, 0x7b);
    c.bus.write_byte(0x0004, 0x7c);
    c.bus.write_byte(0x0005, 0x7d);
    c.bus.write_byte(0x0006, 0x7e);
    c.bus.write_byte(0x0007, 0x7f);
    c.execute();
    assert_eq!(c.registers.a, 0x11);
    c.execute();
    assert_eq!(c.registers.a, 0x15);
    c.execute();
    assert_eq!(c.registers.a, 0x1f);
    c.execute();
    assert_eq!(c.registers.a, 0x21);
    c.execute();
    assert_eq!(c.registers.a, 0x25);
    c.execute();
    assert_eq!(c.registers.a, 0x2f);
    c.execute();
    assert_eq!(c.registers.a, 0x31);
    c.execute();
    assert_eq!(c.registers.a, 0x31);
    assert_eq!(c.pc, 8);
}

#[test]
fn hlt() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x76);
    c.execute();
    assert_eq!(c.halt, true);
    assert_eq!(c.pc, 1);
}

#[test]
fn add_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x80);
    c.registers.a = 0x0f;
    c.registers.b = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1e);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn add_c() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x81);
    c.registers.a = 0x0f;
    c.registers.c = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1e);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn add_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x82);
    c.registers.a = 0x0f;
    c.registers.d = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1e);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn add_e() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x83);
    c.registers.a = 0x0f;
    c.registers.e = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1e);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn add_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x84);
    c.registers.a = 0x0f;
    c.registers.h = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1e);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn add_l() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x85);
    c.registers.a = 0x0f;
    c.registers.l = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1e);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn add_m() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x86);
    c.bus.write_byte(0x100, 0x53);
    c.registers.a = 0x0f;
    c.registers.set_hl(0x100);
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x62);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn add_a() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x87);
    c.registers.a = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1e);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn adc_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x88);
    c.registers.a = 0x0f;
    c.registers.b = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1f);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn adc_c() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x89);
    c.registers.a = 0x0f;
    c.registers.c = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1f);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn adc_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x8a);
    c.registers.a = 0x0f;
    c.registers.d = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1f);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn adc_e() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x8b);
    c.registers.a = 0x0f;
    c.registers.e = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1f);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn adc_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x8c);
    c.registers.a = 0x0f;
    c.registers.h = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1f);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn adc_l() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x8d);
    c.registers.a = 0x0f;
    c.registers.l = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1f);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn adc_m() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x8e);
    c.bus.write_byte(0x100, 0x53);
    c.registers.a = 0x0f;
    c.registers.set_hl(0x100);
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x63);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn adc_a() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x8f);
    c.registers.a = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x1f);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sub_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x90);
    c.registers.a = 0x0f;
    c.registers.b = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sub_c() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x91);
    c.registers.a = 0x0f;
    c.registers.c = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sub_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x92);
    c.registers.a = 0x0f;
    c.registers.d = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sub_e() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x93);
    c.registers.a = 0x0f;
    c.registers.e = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sub_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x94);
    c.registers.a = 0x0f;
    c.registers.h = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sub_l() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x95);
    c.registers.a = 0x0f;
    c.registers.l = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sub_m() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x96);
    c.registers.a = 0x0f;
    c.bus.write_byte(0x100, 2);
    c.registers.set_hl(0x100);
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x0d);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sub_a() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x97);
    c.registers.a = 0x0f;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sbb_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x98);
    c.registers.a = 0x0f;
    c.registers.b = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0xff);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn sbb_c() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x99);
    c.registers.a = 0x0f;
    c.registers.c = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0xff);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn sbb_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x9a);
    c.registers.a = 0x0f;
    c.registers.d = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0xff);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn sbb_e() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x9b);
    c.registers.a = 0x0f;
    c.registers.e = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0xff);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn sbb_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x9c);
    c.registers.a = 0x0f;
    c.registers.h = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0xff);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn sbb_l() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x9d);
    c.registers.a = 0x0f;
    c.registers.l = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0xff);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn sbb_m() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x9e);
    c.registers.a = 0x0f;
    c.bus.write_byte(0x100, 2);
    c.registers.set_hl(0x100);
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0x0c);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.a, true);
    assert_eq!(c.flags.c, false);
}

#[test]
fn sbb_a() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x9f);
    c.registers.a = 0x0f;
    c.flags.c = true;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.registers.a, 0xff);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.a, false);
    assert_eq!(c.flags.c, true);
}

#[test]
fn rst_0() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xc7);
    c.sp = 0xff00;
    c.execute();
    assert_eq!(c.pc, 0);
    assert_eq!(c.sp, 0xfefe);
}

#[test]
fn rst_1() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xcf);
    c.sp = 0xff00;
    c.execute();
    assert_eq!(c.pc, 8);
    assert_eq!(c.sp, 0xfefe);
}

#[test]
fn rst_2() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xd7);
    c.sp = 0xff00;
    c.execute();
    assert_eq!(c.pc, 0x10);
    assert_eq!(c.sp, 0xfefe);
}

#[test]
fn rst_3() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xdf);
    c.sp = 0xff00;
    c.execute();
    assert_eq!(c.pc, 0x18);
    assert_eq!(c.sp, 0xfefe);
}

#[test]
fn rst_4() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xe7);
    c.sp = 0xff00;
    c.execute();
    assert_eq!(c.pc, 0x20);
    assert_eq!(c.sp, 0xfefe);
}

#[test]
fn rst_5() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xef);
    c.sp = 0xff00;
    c.execute();
    assert_eq!(c.pc, 0x28);
    assert_eq!(c.sp, 0xfefe);
}

#[test]
fn rst_6() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xf7);
    c.sp = 0xff00;
    c.execute();
    assert_eq!(c.pc, 0x30);
    assert_eq!(c.sp, 0xfefe);
}

#[test]
fn rst_7() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xff);
    c.sp = 0xff00;
    c.execute();
    assert_eq!(c.pc, 0x38);
    assert_eq!(c.sp, 0xfefe);
}

#[test]
fn cmp_b() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xb8);
    c.bus.write_byte(0x0001, 0xb8);
    c.registers.a = 0x12;
    c.registers.b = 0x12;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.b, 0x12);
    c.registers.b = 0x27;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.b, 0x27);
}

#[test]
fn cmp_c() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xb9);
    c.bus.write_byte(0x0001, 0xb9);
    c.registers.a = 0x12;
    c.registers.c = 0x12;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.c, 0x12);
    c.registers.c = 0x27;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.c, 0x27);
}

#[test]
fn cmp_d() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xba);
    c.bus.write_byte(0x0001, 0xba);
    c.registers.a = 0x12;
    c.registers.d = 0x12;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.d, 0x12);
    c.registers.d = 0x27;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.d, 0x27);
}

#[test]
fn cmp_e() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xbb);
    c.bus.write_byte(0x0001, 0xbb);
    c.registers.a = 0x12;
    c.registers.e = 0x12;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.e, 0x12);
    c.registers.e = 0x27;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.e, 0x27);
}

#[test]
fn cmp_h() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xbc);
    c.bus.write_byte(0x0001, 0xbc);
    c.registers.a = 0x12;
    c.registers.h = 0x12;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.h, 0x12);
    c.registers.h = 0x27;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.h, 0x27);
}

#[test]
fn cmp_l() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xbd);
    c.bus.write_byte(0x0001, 0xbd);
    c.registers.a = 0x12;
    c.registers.l = 0x12;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.l, 0x12);
    c.registers.l = 0x27;
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.registers.l, 0x27);
}

#[test]
fn cmp_m() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xbe);
    c.bus.write_byte(0x0001, 0xbe);
    c.registers.a = 0x12;
    c.registers.set_hl(0x100);
    c.bus.write_byte(0x100, 0x12);
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.bus.read_byte(0x100), 0x12);
    c.bus.write_byte(0x100, 0x27);
    c.execute();
    assert_eq!(c.pc, 2);
    assert_eq!(c.flags.z, false);
    assert_eq!(c.flags.c, true);
    assert_eq!(c.registers.a, 0x12);
    assert_eq!(c.bus.read_byte(0x100), 0x27);
}

#[test]
fn cmp_a() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0xbf);
    c.bus.write_byte(0x0001, 0xbf);
    c.registers.a = 0x12;
    c.execute();
    assert_eq!(c.pc, 1);
    assert_eq!(c.flags.z, true);
    assert_eq!(c.flags.c, false);
    assert_eq!(c.registers.a, 0x12);
}

#[test]
fn dasm() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x35);
    c.registers.set_hl(0x3412);
    assert_eq!(c.dasm(0), String::from("35        DCR (HL)"));
}

#[test]
fn dasm_mvi() {
    let mut c = CPU::new();
    c.bus.write_byte(0x0000, 0x3E);
    c.bus.write_byte(0x0001, 0x55);
    assert_eq!(c.dasm(0), String::from("3E 55     MVI A,$55"));
}

#[test]
fn rom_space_byte() {
    let mut c = CPU::new();
    c.bus.rom_space = Some(ROMSpace{start: 0xfff0, end: 0xffff});
    c.bus.write_byte(0xffef, 0x3E);
    c.bus.write_byte(0xfff0, 0x55);
    c.bus.write_byte(0xffff, 0x55);
    c.bus.write_byte(0x0000, 0x55);
    assert_eq!(c.bus.read_byte(0xffef), 0x3e);
    assert_eq!(c.bus.read_byte(0xfff0), 0);
    assert_eq!(c.bus.read_byte(0xffff), 0);
    assert_eq!(c.bus.read_byte(0x0000), 0x55);
}

#[test]
fn rom_space_word() {
    let mut c = CPU::new();
    c.bus.rom_space = Some(ROMSpace{start: 0xfff0, end: 0xffff});
    c.bus.write_word(0xffee, 0x3E3E);
    c.bus.write_word(0xfff0, 0x5566);
    assert_eq!(c.bus.read_word(0xffee), 0x3e3e);
    assert_eq!(c.bus.read_byte(0xfff0), 0);
}
