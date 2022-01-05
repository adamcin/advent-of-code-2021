mod common;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Reg {
    W = 0,
    X,
    Y,
    Z,
}
use Reg::*;
impl Reg {
    fn parse(opd: &str) -> Option<Self> {
        match opd {
            "w" => Some(W),
            "x" => Some(X),
            "y" => Some(Y),
            "z" => Some(Z),
            _ => None,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Opd {
    Num(isize),
    Var(Reg),
}
use Opd::*;
impl Opd {
    fn parse(opd: &str) -> Option<Self> {
        Reg::parse(opd)
            .map(|reg| Var(reg))
            .or_else(|| opd.parse().ok().map(|num| Num(num)))
    }

    fn eval(&self, alu: &Alu) -> isize {
        match self {
            Num(val) => *val,
            Var(reg) => alu.get_reg(reg),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Inst {
    Inp(Reg),
    Add(Reg, Opd),
    Mul(Reg, Opd),
    Div(Reg, Opd),
    Mod(Reg, Opd),
    Eql(Reg, Opd),
}
use Inst::*;
impl Inst {
    fn parse(line: &str) -> Option<Self> {
        if let Some((code, opds)) = line.split_once(" ") {
            if code == "inp" {
                Reg::parse(opds).map(|reg| Inp(reg))
            } else if let Some((op_reg, op_opd)) = opds.split_once(" ") {
                Reg::parse(op_reg)
                    .zip(Opd::parse(op_opd))
                    .and_then(|(reg, opd)| match code {
                        "add" => Some(Add(reg, opd)),
                        "mul" => Some(Mul(reg, opd)),
                        "div" => Some(Div(reg, opd)),
                        "mod" => Some(Mod(reg, opd)),
                        "eql" => Some(Eql(reg, opd)),
                        _ => None,
                    })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_inp(&self) -> bool {
        match self {
            Inp(_) => true,
            _ => false,
        }
    }

    fn exec(&self, alu: &mut Alu) {
        match self {
            Add(reg, opd) => alu.set_reg(reg, alu.get_reg(reg) + opd.eval(alu)),
            Mul(reg, opd) => alu.set_reg(reg, alu.get_reg(reg) * opd.eval(alu)),
            Div(reg, opd) => {
                assert!(opd.eval(alu) != 0, "attempted div by 0");
                alu.set_reg(reg, alu.get_reg(reg) / opd.eval(alu))
            }
            Mod(reg, opd) => {
                assert!(alu.get_reg(reg) >= 0, "attempted neg mod");
                assert!(opd.eval(alu) > 0, "attempted mod div by 0 or neg");
                alu.set_reg(reg, alu.get_reg(reg) % opd.eval(alu))
            }
            Eql(reg, opd) => alu.set_reg(
                reg,
                if alu.get_reg(reg) == opd.eval(alu) {
                    1
                } else {
                    0
                },
            ),
            _ => {}
        }
    }

    fn is_reset_reg(&self) -> Option<Reg> {
        match self {
            Inp(reg) => Some(*reg),
            Mul(reg, Num(0)) => Some(*reg),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct AluProg {
    insts: Vec<Inst>,
}

impl AluProg {
    fn new(insts: Vec<Inst>) -> Self {
        assert!(
            insts.first().unwrap().is_inp(),
            "insts must start with inp operation"
        );
        Self { insts: insts }
    }

    fn parse(lines: Vec<String>) -> Vec<Self> {
        let mut progs: Vec<_> = Vec::new();
        let mut prog: Vec<_> = Vec::new();
        for parsed in lines.iter().map(|line| Inst::parse(line)) {
            if let Some(inst) = parsed {
                match inst {
                    Inp(_) => {
                        if !prog.is_empty() {
                            progs.push(AluProg::new(prog));
                            prog = vec![inst];
                        } else {
                            prog.push(inst);
                        }
                    }
                    _ => prog.push(inst),
                }
            }
        }

        if !prog.is_empty() {
            progs.push(AluProg::new(prog));
        }
        progs
    }

    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    fn reg_input(&self, out_reg: &Reg, mod_reg: &Reg) -> Option<(isize, isize)> {
        for (inst0, (inst1, (inst2, inst3))) in self.insts.iter().zip(
            self.insts
                .iter()
                .skip(1)
                .zip(self.insts.iter().skip(2).zip(self.insts.iter().skip(3))),
        ) {
            match (inst0, inst1, inst2, inst3) {
                (
                    Mul(mreg0, Num(0)),
                    Add(mreg1, Var(oreg0)),
                    Mod(mreg2, Num(mod_num)),
                    Div(oreg1, Num(div_num)),
                ) => {
                    if mreg0 == mod_reg
                        && mreg1 == mod_reg
                        && mreg2 == mod_reg
                        && oreg0 == out_reg
                        && oreg1 == out_reg
                    {
                        return Some((*div_num, *mod_num));
                    }
                }
                _ => {}
            }
        }
        return None;
    }

    fn reg_equiv(&self, in_reg: &Reg, mod_reg: &Reg) -> Option<isize> {
        for (inst0, (inst1, inst2)) in self
            .insts
            .iter()
            .zip(self.insts.iter().skip(1).zip(self.insts.iter().skip(2)))
        {
            match (inst0, inst1, inst2) {
                (Add(mreg0, Num(diff)), Eql(mreg1, Var(ireg)), Eql(mreg2, Num(0))) => {
                    if mreg0 == mod_reg && mreg1 == mod_reg && mreg2 == mod_reg && ireg == in_reg {
                        return Some(*diff);
                    }
                }
                _ => {}
            }
        }
        return None;
    }

    fn reg_scaler(&self, out_reg: &Reg, mod_reg: &Reg, acc_reg: &Reg) -> Option<(isize, isize)> {
        for (inst0, (inst1, (inst2, (inst3, inst4)))) in self.insts.iter().zip(
            self.insts.iter().skip(1).zip(
                self.insts
                    .iter()
                    .skip(2)
                    .zip(self.insts.iter().skip(3).zip(self.insts.iter().skip(4))),
            ),
        ) {
            match (inst0, inst1, inst2, inst3, inst4) {
                (
                    Mul(acc0, Num(0)),
                    Add(acc1, Num(on1)),
                    Mul(acc2, Var(mreg)),
                    Add(acc3, Num(on0)),
                    Mul(oreg, Var(acc4)),
                ) => {
                    if acc0 == acc_reg
                        && acc0 == acc_reg
                        && acc1 == acc_reg
                        && acc2 == acc_reg
                        && acc3 == acc_reg
                        && acc4 == acc_reg
                        && mreg == mod_reg
                        && oreg == out_reg
                    {
                        return Some((*on0, (*on0 + *on1)));
                    }
                }
                _ => {}
            }
        }
        return None;
    }

    // mul y 0
    // add y w
    // add y 11
    // mul y x // 9 + 11 = y=20
    // add z y // equiv: z*26+20, which forces input to prog[7] to be 7, not 9
    fn reg_output(
        &self,
        in_reg: &Reg,
        out_reg: &Reg,
        mod_reg: &Reg,
        acc_reg: &Reg,
    ) -> Option<isize> {
        for (inst0, (inst1, (inst2, (inst3, inst4)))) in self.insts.iter().zip(
            self.insts.iter().skip(1).zip(
                self.insts
                    .iter()
                    .skip(2)
                    .zip(self.insts.iter().skip(3).zip(self.insts.iter().skip(4))),
            ),
        ) {
            match (inst0, inst1, inst2, inst3, inst4) {
                (
                    Mul(acc0, Num(0)),
                    Add(acc1, Var(ireg)),
                    Add(acc2, Num(diff)),
                    Mul(acc3, Var(mreg)),
                    Add(oreg, Var(acc4)),
                ) => {
                    if acc0 == acc_reg
                        && acc0 == acc_reg
                        && acc1 == acc_reg
                        && acc2 == acc_reg
                        && acc3 == acc_reg
                        && acc4 == acc_reg
                        && mreg == mod_reg
                        && oreg == out_reg
                        && ireg == in_reg
                    {
                        return Some(*diff);
                    }
                }
                _ => {}
            }
        }
        return None;
    }
}

fn read() -> Vec<AluProg> {
    AluProg::parse(common::read_test_input("data/day-24/input.txt"))
}

#[derive(Debug, Clone)]
struct Alu {
    regs: [isize; 4],
}

impl Alu {
    fn new() -> Self {
        Alu { regs: [0; 4] }
    }

    fn run(&mut self, prog: &AluProg, input: isize) {
        if let Some((Inp(reg), ops)) = prog.insts.split_first() {
            self.set_reg(reg, input);
            for op in ops {
                op.exec(self);
            }
        }
    }

    fn get_reg(&self, reg: &Reg) -> isize {
        self.regs[*reg as usize]
    }

    fn set_reg(&mut self, reg: &Reg, val: isize) {
        self.regs[*reg as usize] = val;
    }

    fn reset(&mut self) {
        self.regs = [0; 4];
    }
}

#[test]
fn day24_test_read_progs() {
    let progs = read();
    assert_eq!(14, progs.len());
}

#[test]
fn day24pre_part1_0() {
    let mut alu = Alu::new();
    let prog = AluProg::new(vec![Inp(X), Mul(X, Num(-1))]);
    alu.run(&prog, 10);
    assert_eq!(-10, alu.get_reg(&X));
}

#[test]
fn day24pre_part1_1() {
    let mut alu = Alu::new();
    let prog1 = AluProg::new(vec![Inp(Z)]);
    let prog2 = AluProg::new(vec![Inp(X), Mul(Z, Num(3)), Eql(Z, Var(X))]);
    alu.run(&prog1, 10);
    alu.run(&prog2, 30);
    assert_eq!(1, alu.get_reg(&Z));
}

// in_z=0
#[test]
fn day24pre_part1_inp_00() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[0];
    let in_z = 0;
    let w = 9;
    alu.reset();
    alu.set_reg(&Z, in_z);
    alu.run(prog, w);
    let z = alu.get_reg(&Z);
    assert!(z == 11, "invalid outz {} for w {} and in_z {}", z, w, in_z);
    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 14
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 12
    // mul y x
    // add z y
}

// in_z=0
#[test]
fn day24pre_part1_inp_01() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[1];
    let in_z = 0;
    let w = 9;
    alu.reset();
    alu.set_reg(&Z, in_z);
    alu.run(prog, w);
    let z = alu.get_reg(&Z);
    assert!(z == 11, "invalid outz {} for w {} and in_z {}", z, w, in_z);
    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 15
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 7
    // mul y x
    // add z y
}
// in_z=0
#[test]
fn day24pre_part1_inp_02() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[2];
    let in_z = 0;
    let w = 9;
    alu.reset();
    alu.set_reg(&Z, in_z);
    alu.run(prog, w);
    let z = alu.get_reg(&Z);
    assert!(z == 11, "invalid outz {} for w {} and in_z {}", z, w, in_z);
    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 12
    // eql x w
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y

    // mul y 0
    // add y w
    // add y 1
    // mul y x
    // add z y
}

// in_z=0 and w=9 gets us out_z=11
#[test]
fn day24pre_part1_inp_03() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[3];
    for in_z in 0..=3 {
        let w = 9;
        alu.reset();
        alu.set_reg(&Z, in_z);
        alu.run(prog, w);
        let z = alu.get_reg(&Z);
        assert!(
            z % 26 == 11,
            "invalid outz {} for w {} and in_z {}",
            z,
            w,
            in_z
        );
    }
    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 11
    // eql x w
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x // x is 1
    // add y 1 // y is 26
    // mul z y // equiv to z*26

    // mul y 0
    // add y w
    // add y 2 // y is w+2
    // mul y x // x is 1
    // add z y // equiv to z*26 + w + 2
}

// in_z=14 and w=9 gets us out_z=0
// in_z=11 and w=6 gets us out_z=0
#[test]
fn day24pre_part1_inp_04() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[4];
    for z in 0..=11 {
        let in_z = (z * 26) + (11 - z);
        let w = (in_z % 26) - 5;
        if w < 1 {
            break;
        }
        alu.reset();
        alu.set_reg(&Z, in_z);
        alu.run(prog, w);
        let out_z = alu.get_reg(&Z);
        assert!(z == 0, "invalid outz {} for w {} and in_z {}", z, w, in_z);
    }
    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -5
    // eql x w w+5=in_z%26
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y

    // mul y 0
    // add y w
    // add y 4
    // mul y x
    // add z y
}

// in_z=0 and w=9 gets us out_z=24
#[test]
fn day24pre_part1_inp_05() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[5];
    let in_z = 0;
    let w = 9;
    alu.reset();
    alu.set_reg(&Z, in_z);
    alu.run(prog, w);
    let z = alu.get_reg(&Z);
    assert!(z == 24, "invalid outz {} for w {} and in_z {}", z, w, in_z);
    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1 // z not reduced so in_z MUST be zero
    // add x 14 // never x=w
    // eql x w
    // eql x 0 // always x=1

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y // if z is non-zero, out_z will exceed 24

    // mul y 0
    // add y w
    // add y 15
    // mul y x // x always non-zero, so y is w+15, which makes w=9 work if in_z=0
    // add z y
}

// in_z=0..=24 with w=9 gets us out_z=z*26+20
#[test]
fn day24pre_part1_inp_06() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[6];
    for in_z in 0..=24 {
        let w = 9;
        alu.reset();
        alu.set_reg(&Z, in_z);
        alu.run(prog, w);
        let z = alu.get_reg(&Z);
        assert!(
            z % 26 == 20 && z / 26 <= 24,
            "invalid outz {} for w {} and in_z {}",
            z,
            w,
            in_z
        );
    }
    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 15 // always x != w
    // eql x w
    // eql x 0 // always 1

    // mul y 0
    // add y 25
    // mul y x
    // add y 1 // y=26
    // mul z y // equiv: z*26

    // mul y 0
    // add y w
    // add y 11
    // mul y x // 9 + 11 = y=20
    // add z y // equiv: z*26+20, which forces input to prog[7] to be 7, not 9
}

// by itself, this prog can handle w=9 with z>21, but prog[6] can't produce a compatible out_z for that.
// in_z=0..=21 with w=9 gets us out_z=0..=24
// in_z=(0..=24).map(|z| z*26 + 20) with w=7 gets us out_z=0..24
// 20, 46, etc.
#[test]
fn day24pre_part1_inp_07() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[7];
    for in_z in 0..=21 {
        let w = 9;
        alu.reset();
        alu.set_reg(&Z, in_z);
        alu.run(prog, w);
        let z = alu.get_reg(&Z);
        assert!(
            z >= 0 && z <= 24,
            "invalid outz {} for w {} and in_z {}",
            z,
            w,
            in_z
        );
    }

    for in_z in (0..=24).map(|z| (z * 26) + 20) {
        let w = 7;
        alu.reset();
        alu.set_reg(&Z, in_z);
        alu.run(prog, w);
        let z = alu.get_reg(&Z);
        assert!(
            z >= 0 && z <= 24,
            "invalid outz {} for w {} and in_z {}",
            z,
            w,
            in_z
        );
    }

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -13 // 0..=21 with w=9 for x=1 or z*26 + 22 with w=9 for x=0
    // eql x w
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y

    // mul y 0
    // add y w
    // add y 5
    // mul y x
    // add z y
}

// in_z=0..=24 with w=9 gets us out_z=12
#[test]
fn day24pre_part1_inp_08() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[8];
    for in_z in 0..=24 {
        let w = 9;
        alu.reset();
        alu.set_reg(&Z, in_z);
        alu.run(prog, w);
        let z = alu.get_reg(&Z);
        assert!(
            z >= 12 && z <= 17,
            "invalid outz {} for w {} and in_z {}",
            z,
            w,
            in_z
        );
    }

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -16
    // eql x w
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y // blown if both x and z are non-zero

    // mul y 0
    // add y w
    // add y 3
    // mul y x
    // add z y

    // best we can do with z=0,w=9,x=1 is out_z=12.
    // in_z of (z*26) + 17..=25 with matching w+16 gets us x=0
    // z would have to be 9..=17 to get us to prog[10] by itself
    // in_z < 26 and x!=w+16 gets us to out_z=12.
    // -> specifically, in_z=0..=24 with w=9
}

// in_z must be 9..=17 with matching w+8 to arrive at out_z = 0 for prog[10]
// in_z from prog[8] is max 12, which means best w=4
#[test]
fn day24pre_part1_inp_09() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[9];
    for in_z in 9..=17 {
        let w = in_z - 8;
        alu.reset();
        alu.set_reg(&Z, in_z);
        alu.run(prog, w);
        let z = alu.get_reg(&Z);
        assert!(z == 0, "invalid outz {} for w {} and in_z {}", z, w, in_z);
    }

    // inp w
    // mul x 0
    // add x z
    // mod x 26  // in_z must be 9..=17
    // div z 26  // for z to be zero here, in_z must be 0..=25
    // add x -8  // x in 9..=17 for subtraction to w
    // eql x w   // x = w for x to be 1
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y // z must be zero since y is at least 1

    // mul y 0
    // add y w
    // add y 9
    // mul y x // for y to be zero X MUST be zero
    // add z y // this MUST be zero, which means y MUST be zero
}

// in_z MUST be zero for valid output to prog[11]
#[test]
fn day24pre_part1_inp_10() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[10];
    for w in 1..=9 {
        alu.reset();
        alu.set_reg(&Z, 0);
        alu.run(prog, w);
        let z = alu.get_reg(&Z);
        assert!(z >= 0 && z <= 25, "invalid outz {} for w {}", z, w);
    }

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 15
    // eql x w // 15 can't be equal to w for w=1..=9
    // eql x 0 // so x is always 1 here

    // mul y 0
    // add y 25 // y = 25
    // mul y x  // y = 25
    // add y 1  // y = 26
    // mul z y  // if z != 0 before this, it is at least 26 from here on

    // mul y 0
    // add y w
    // add y 2  // y = w + 2
    // mul y x  // x is still one, making y at least 3 here
    // add z y // adds 3-11 to z. if z is not zero, it is at least 29-37, well out of range.
}

// in_z must be 0..=25 for w 1..=9
#[test]
fn day24pre_part1_inp_11() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[11];
    for in_z in 0..=25 {
        for w in 1..=9 {
            alu.reset();
            alu.set_reg(&Z, in_z);
            alu.run(prog, w);
            let z = alu.get_reg(&Z);
            assert!(
                z >= 0 && z <= 25,
                "invalid outz {} for w {} and inz {}",
                z,
                w,
                in_z
            );
        }
    }

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -8
    // eql x w
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y

    // mul y 0
    // add y w
    // add y 3
    // mul y x
    // add z y
}

// prog[11].z must be in range of 0-25 and not be equal to w, with w in the
// range of 2-9 to produce output between 5-12
#[test]
fn day24pre_part1_inp_12() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[12];

    assert_eq!(Some((26, 26)), prog.reg_input(&Z, &X));
    assert_eq!(Some(0), prog.reg_equiv(&W, &X));
    assert_eq!(Some((1, 26)), prog.reg_scaler(&Z, &X, &Y));
    assert_eq!(Some(3), prog.reg_output(&W, &Z, &X, &Y));

    for in_z in 0..=25 {
        for w in 2..=9 {
            if w == in_z {
                continue;
            }
            alu.reset();
            alu.set_reg(&Z, in_z);
            alu.run(prog, w);
            let z = alu.get_reg(&Z);
            assert!(z >= 5 && z <= 13);
        }
    }

    for in_z in 0..=25 {
        for w in 2..=9 {
            if w == in_z {
                continue;
            }
            alu.reset();
            alu.set_reg(&Z, in_z);
            alu.run(prog, w);
            let z = alu.get_reg(&Z);
            assert!(z >= 5 && z <= 13);
        }
    }

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x 0
    // eql x w
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y

    // mul y 0
    // add y w
    // add y 3
    // mul y x
    // add z y
}

// in_z=5..=13 with matching w+4 achieves out_z=0
#[test]
fn day24pre_part1_inp_13() {
    let mut alu = Alu::new();
    let progs = read();
    let prog = &progs[13];

    assert_eq!(Some((26, 26)), prog.reg_input(&Z, &X));
    assert_eq!(Some(-4), prog.reg_equiv(&W, &X));
    assert_eq!(Some((1, 26)), prog.reg_scaler(&Z, &X, &Y));
    assert_eq!(Some(11), prog.reg_output(&W, &Z, &X, &Y));

    for w in 1..=9 {
        alu.reset();
        alu.set_reg(&Z, w + 4);
        alu.run(prog, w);
        assert_eq!(0, alu.get_reg(&Z));
    }

    // inp w
    // mul x 0
    // add x z
    // mod x 26
    // div z 26
    // add x -4
    // eql x w
    // eql x 0

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y

    // mul y 0
    // add y w
    // add y 11
    // mul y x
    // add z y

    // prev z must be positive or zero
    // z is truncated to range 0-25
    // x is (z mod 26) - 4 (-4 to 21)
    // x is (w != x) ( )
    // y is (26 if w != z - 4 or else 1 )
    // z is (original if y is 26 or else original / 26)
    // y is w + 11
    //
}

#[derive(Debug, Clone)]
struct Monad {}

impl Monad {
    fn solve(&self, progs: &[AluProg]) -> Vec<isize> {
        if let Some((tail, progs)) = progs.split_last() {}
        vec![]
    }

    fn factors(prog: &AluProg) -> Option<(isize, isize, isize)> {
        if let (Some((divisor, _)), Some(input_shift), Some(output_shift)) = (
            prog.reg_input(&Z, &X),
            prog.reg_equiv(&W, &X),
            prog.reg_output(&W, &Z, &X, &Y),
        ) {
            Some((divisor, input_shift, output_shift))
        } else {
            None
        }
    }
}

use std::cmp::{max, min};

fn madd26(outzs: &[isize]) -> isize {
    if let Some((tail, head)) = outzs.split_last() {
        (26 * madd26(head)) + tail
    } else {
        0
    }
}

// i. div. inz.    outz.
// 00    +         (13, 21)
// 01    +         (8, 16)
// 02    +         (2, 10)
// 03    +         (3, 11)
// 04    - (6, 14)
// 05    +         (16, 24)
// 06    +         (12, 20)
// 07    - (14, 22)
// 08    - (17, 25)
// 09    - (9, 17)
// 10    +         (3, 11)
// 11    - (9, 17)
// 12    - (1, 9)
// 13    - (5, 13) 
#[test]
fn day24part1() {
    let progs = read();
    let mut alu = Alu::new();
    // i. div. inz.    outz.
    // 00    +         (13, 21)>(13) w=1
    // 01    +         (8, 16) >(8..9). w=(1..2)   <(13)
    // 02    +         (2, 10) >(9..10) w=(8..9)   <((13)*26)+(8..9)
    // 03    +         (3, 11) >(6..11) w=(4..9)   <((((13)*26)+(8..9))*26)+(9..10)
    // 04    - (6, 14)         <((((((13)*26)+(8..9))*26)+(9..10))*26)+(6..11)        w=(1..6)
    // 05    +         (16, 24)>(17..24) w=(2..8)  <((((13)*26)+(8..9))*26)+(9..10)
    // 06    +         (12, 20)>(14..20) w=(3..9)  <((((((13)*26)+(8..9))*26)+(9..10))*26)+(17..24)
    // 07    - (14, 20)        <((((((((13)*26)+(8..9))*26)+(9..10))*26)+(17..24))*26)+(14..20) w=(1..7)
    // 08    - (17, 24)        <((((((13)*26)+(8..9))*26)+(9..10))*26)+(17..24) w=(1..8)
    // 09    - (9, 10)         <((((13)*26)+(8..9))*26)+(9..10) (w=1,2)
    // 10    +         (3, 11) >(9..11) w=(7/8/9)  <((5..13)*26)+(8..9)
    // 11    - (9, 11)         <((((13)*26)+(8..9))*26)+(9..11) (w=1,2,3)
    // 12    - (1, 9)          <((13)*26)+(8..9) w=8..9
    // 13    - (13)            <(13) w=9
    //.                        0  1  2  3  4  5  6  7  8  9  10 11 12 13 
    // largest: let inputs: [isize; 14] = [1, 2, 9, 9, 6, 9, 9, 7, 8, 2, 9, 3, 9, 9];
    // 12996997829399
    let inputs: [isize; 14] = [1, 1, 8, 4, 1, 2, 3, 1, 1, 1, 7, 1, 8, 9];
    // 11841231117189
    let take = 14; 
    let factors: Vec<(isize, (isize, isize, isize))> = inputs
        .iter()
        .cloned()
        .zip(progs.iter().flat_map(|prog| Monad::factors(prog)))
        .collect();
    let outzs: Vec<isize> =
        factors
            .iter()
            .fold(Vec::new(), |acc, (w, (divisor, ishift, oshift))| {
                if let Some(last) = acc.last() {
                    if *divisor == 26 && w - ishift == last % 26 {
                        vec![acc.to_owned(), vec![last / 26]].concat()
                    } else {
                        vec![acc.to_owned(), vec![((26 / divisor) * last) + oshift + w]].concat()
                    }
                } else {
                    if *divisor == 26 && *ishift >= 1 && *ishift <= 9 {
                        vec![0]
                    } else {
                        vec![oshift + w]
                    }
                }
            });
    let expects: [isize; 14] = [
        outzs[0], outzs[1], outzs[2], outzs[3], outzs[3] / 26, outzs[5], outzs[6], outzs[6] / 26, outzs[7] / 26, outzs[8] / 26, outzs[10], outzs[10] / 26, outzs[11] / 26, 0,
    ];
    for (i, ((input, expect), prog)) in inputs
        .iter()
        .zip(outzs.iter())
        .zip(progs.iter())
        .take(take)
        .enumerate()
    {
        alu.run(prog, *input);
        let last_outz = if i == 0 { 0 } else { outzs[i - 1] };
        assert_eq!(
            if take == i + 1 { expects[i] } else { *expect },
            alu.get_reg(&Z),
            "expect outz for prog {} with w {} and in_z {} mod26 {} div26 {} factors: {:?}",
            i,
            input,
            last_outz,
            last_outz % 26,
            last_outz / 26,
            factors[i],
        );
    }
    //assert_eq!(expects[take - 1], outzs[take - 1], "expect 0 for last outz");
}

#[test]
fn day24_test_factors() {
    let progs = read();
    println!("i. div.\tinz.\toutz.");
    for (i, prog) in progs.iter().enumerate() {
        if let Some((divisor, input_shift, output_shift)) = Monad::factors(prog) {
            let w_bounds: Option<(isize, isize)> = if input_shift > 9 {
                None
            } else {
                let ws: Vec<isize> = (1..=9).filter(|w| w - input_shift <= 25).collect();
                let w_min = ws.iter().fold(10, |a, v| min(a, *v));
                let w_max = ws.iter().fold(0, |a, v| max(a, *v));
                Some((w_min, w_max))
            };
            let mod_z_bounds: Option<(isize, isize)> =
                w_bounds.map(|(w_min, w_max)| (w_min - input_shift, w_max - input_shift));
            println!(
                "{:02} {}\t{}\t{}",
                i,
                if divisor == 1 { "   +" } else { "   -" },
                // w_bounds
                //     .map(|bs| format!("{:?}", bs))
                //     .unwrap_or("".to_owned()),
                mod_z_bounds
                    .map(|bs| format!("{:?}", bs))
                    .unwrap_or("".to_owned()),
                w_bounds
                    .map(|_| "\t".to_owned())
                    .unwrap_or(format!("{:?}", (1 + output_shift, 9 + output_shift))),
            );
        }
    }
}
