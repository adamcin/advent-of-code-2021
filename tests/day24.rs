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
