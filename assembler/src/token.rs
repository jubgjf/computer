use core::panic;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// 操作符
enum Operator {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Slt,
    Sltu,
    Sll,
    Srl,
    Sra,
    Addi,
    Andi,
    Ori,
    Xori,
    Slti,
    Sltiu,
    Lw,
    Sw,
    Beq,
    Bne,
    Lui,
    Jal,
    Jalr,
    Csr,
}

/// 操作符类型
enum OperatorType {
    // op rd rs1 rs2
    Dss,

    // op rd rs imm
    Dsi,

    // op rd imm
    Di,

    // op rd
    D,
}

impl Operator {
    /// 从操作符字符串（全大写）构建操作符枚举
    ///
    /// # Example
    ///
    /// ```no_run
    /// Operator::from(Some("ADD")) -> Operator::Add
    /// Operator::from(Some("SUB")) -> Operator::Sub
    ///
    /// Operator::from(Some("XYZ")) -> panic!
    /// ```
    fn from(op: Option<&str>) -> Operator {
        match op {
            Some("ADD") => Operator::Add,
            Some("SUB") => Operator::Sub,
            Some("AND") => Operator::And,
            Some("OR") => Operator::Or,
            Some("XOR") => Operator::Xor,
            Some("SLT") => Operator::Slt,
            Some("SLTU") => Operator::Sltu,
            Some("SLL") => Operator::Sll,
            Some("SRL") => Operator::Srl,
            Some("SRA") => Operator::Sra,
            Some("ADDI") => Operator::Addi,
            Some("ANDI") => Operator::Andi,
            Some("ORI") => Operator::Ori,
            Some("XORI") => Operator::Xori,
            Some("SLTI") => Operator::Slti,
            Some("SLTIU") => Operator::Sltiu,
            Some("LW") => Operator::Lw,
            Some("SW") => Operator::Sw,
            Some("BEQ") => Operator::Beq,
            Some("BNE") => Operator::Bne,
            Some("LUI") => Operator::Lui,
            Some("JAL") => Operator::Jal,
            Some("JALR") => Operator::Jalr,
            Some("CSR") => Operator::Csr,

            _ => panic!("No such operator"),
        }
    }

    /// 获取操作符类型
    ///
    /// # Example
    ///
    /// ```no_run
    /// Operator::Add -> OperatorType::Dss
    /// Operator::Addi -> OperatorType::Dsi
    /// Operator::Lui -> OperatorType::Di
    /// Operator::Csr -> OperatorType::D
    /// ```
    fn get_type(op: &Operator) -> OperatorType {
        match op {
            Operator::Add => OperatorType::Dss,
            Operator::Sub => OperatorType::Dss,
            Operator::And => OperatorType::Dss,
            Operator::Or => OperatorType::Dss,
            Operator::Xor => OperatorType::Dss,
            Operator::Slt => OperatorType::Dss,
            Operator::Sltu => OperatorType::Dss,
            Operator::Sll => OperatorType::Dss,
            Operator::Srl => OperatorType::Dss,
            Operator::Sra => OperatorType::Dss,

            Operator::Addi => OperatorType::Dsi,
            Operator::Andi => OperatorType::Dsi,
            Operator::Ori => OperatorType::Dsi,
            Operator::Xori => OperatorType::Dsi,
            Operator::Slti => OperatorType::Dsi,
            Operator::Sltiu => OperatorType::Dsi,
            Operator::Lw => OperatorType::Dsi,
            Operator::Sw => OperatorType::Dsi,
            Operator::Beq => OperatorType::Dsi,
            Operator::Bne => OperatorType::Dsi,
            Operator::Jalr => OperatorType::Dsi,

            Operator::Lui => OperatorType::Di,
            Operator::Jal => OperatorType::Di,

            Operator::Csr => OperatorType::D,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Operator::Add => write!(f, "ADD"),
            Operator::Sub => write!(f, "SUB"),
            Operator::And => write!(f, "AND"),
            Operator::Or => write!(f, "OR"),
            Operator::Xor => write!(f, "XOR"),
            Operator::Slt => write!(f, "SLT"),
            Operator::Sltu => write!(f, "SLTU"),
            Operator::Sll => write!(f, "SLL"),
            Operator::Srl => write!(f, "SRL"),
            Operator::Sra => write!(f, "SRA"),
            Operator::Addi => write!(f, "ADDI"),
            Operator::Andi => write!(f, "ANDI"),
            Operator::Ori => write!(f, "ORI"),
            Operator::Xori => write!(f, "XORI"),
            Operator::Slti => write!(f, "SLTI"),
            Operator::Sltiu => write!(f, "SLTIU"),
            Operator::Lw => write!(f, "LW"),
            Operator::Sw => write!(f, "SW"),
            Operator::Beq => write!(f, "BEQ"),
            Operator::Bne => write!(f, "BNE"),
            Operator::Lui => write!(f, "LUI"),
            Operator::Jal => write!(f, "JAL"),
            Operator::Jalr => write!(f, "JALR"),
            Operator::Csr => write!(f, "CSR"),
        }
    }
}

/// 寄存器
enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Register {
    /// 从寄存器字符串（全大写）构建寄存器枚举
    ///
    /// # Example
    ///
    /// ```no_run
    /// Register::from(Some("r0")) -> Register::R0
    /// Register::from(Some("r15")) -> Register::R15
    ///
    /// Register::from(Some("r16")) -> panic!
    /// ```
    fn from(reg: Option<&str>) -> Register {
        match reg {
            Some("r0") => Register::R0,
            Some("r1") => Register::R1,
            Some("r2") => Register::R2,
            Some("r3") => Register::R3,
            Some("r4") => Register::R4,
            Some("r5") => Register::R5,
            Some("r6") => Register::R6,
            Some("r7") => Register::R7,
            Some("r8") => Register::R8,
            Some("r9") => Register::R9,
            Some("r10") => Register::R10,
            Some("r11") => Register::R11,
            Some("r12") => Register::R12,
            Some("r13") => Register::R13,
            Some("r14") => Register::R14,
            Some("r15") => Register::R15,

            _ => panic!("No such register"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Register::R0 => write!(f, "r0"),
            Register::R1 => write!(f, "r1"),
            Register::R2 => write!(f, "r2"),
            Register::R3 => write!(f, "r3"),
            Register::R4 => write!(f, "r4"),
            Register::R5 => write!(f, "r5"),
            Register::R6 => write!(f, "r6"),
            Register::R7 => write!(f, "r7"),
            Register::R8 => write!(f, "r8"),
            Register::R9 => write!(f, "r9"),
            Register::R10 => write!(f, "r10"),
            Register::R11 => write!(f, "r11"),
            Register::R12 => write!(f, "r12"),
            Register::R13 => write!(f, "r13"),
            Register::R14 => write!(f, "r14"),
            Register::R15 => write!(f, "r15"),
        }
    }
}

/// 立即数或代表立即数的标签
enum Immediate {
    // 32 位有符号数
    S32(i32),

    // TODO 8/16/32 位 有/无 符号数

    // 标签，分别代表 (标签名, 标签对应的地址)
    Label(String, u32),
}

impl Display for Immediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Immediate::S32(value) => write!(f, "S32: 0x{:x}", value),
            Immediate::Label(name, addr) => write!(f, "label: {}, addr: 0x{:x}", name, addr),
        }
    }
}

/// 指令
pub struct Instruction {
    /// 操作符
    op: Operator,

    /// 目的寄存器
    rd: Register,

    /// 源寄存器 1
    ///
    /// 只有操作符类别是
    /// OperatorType::Dss 或 OperatorType::Dsi 时，
    /// 此字段为 Some(_)
    rs1: Option<Register>,

    /// 源寄存器 2
    ///
    /// 只有操作符类别是 OperatorType::Dss 时，
    /// 有此字段
    rs2: Option<Register>,

    /// 立即数
    ///
    /// 只有操作符类别是
    /// OperatorType::Dsi 或 OperatorType::Di 时，
    /// 此字段为 Some(_)
    imm: Option<Immediate>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match Operator::get_type(&self.op) {
            OperatorType::Dss => write!(
                f,
                "op={} rd={} rs1={} rs2={}",
                self.op,
                self.rd,
                self.rs1.as_ref().unwrap(),
                self.rs2.as_ref().unwrap()
            ),
            OperatorType::Dsi => write!(
                f,
                "op={} rd={} rs={} imm={}",
                self.op,
                self.rd,
                self.rs1.as_ref().unwrap(),
                self.imm.as_ref().unwrap()
            ),
            OperatorType::Di => write!(
                f,
                "op={} rd={} imm={}",
                self.op,
                self.rd,
                self.imm.as_ref().unwrap()
            ),
            OperatorType::D => write!(f, "op={} rd={}", self.op, self.rd,),
        }
    }
}

/// 分析一行指令，给出指令结构体
pub fn analyze_line(line: String) -> Instruction {
    let mut iter = line.split_ascii_whitespace();
    let op = Operator::from(iter.next());

    match Operator::get_type(&op) {
        OperatorType::Dss => Instruction {
            op,
            rd: Register::from(iter.next()),
            rs1: Some(Register::from(iter.next())),
            rs2: Some(Register::from(iter.next())),
            imm: None,
        },
        OperatorType::Dsi => Instruction {
            op,
            rd: Register::from(iter.next()),
            rs1: Some(Register::from(iter.next())),
            rs2: None,
            imm: {
                let imm_str = iter.next().unwrap();
                Some(match imm_str.parse::<i32>() {
                    Ok(val) => Immediate::S32(val),
                    Err(_) => Immediate::Label(imm_str.to_string(), 0xFFFFFFFF),
                })
            },
        },
        OperatorType::Di => Instruction {
            op,
            rd: Register::from(iter.next()),
            rs1: None,
            rs2: None,
            imm: {
                let imm_str = iter.next().unwrap();
                Some(match imm_str.parse::<i32>() {
                    Ok(val) => Immediate::S32(val),
                    Err(_) => Immediate::Label(imm_str.to_string(), 0xFFFFFFFF),
                })
            },
        },
        OperatorType::D => Instruction {
            op,
            rd: Register::from(iter.next()),
            rs1: None,
            rs2: None,
            imm: None,
        },
    }
}

/// 将指令中的标签替换为立即数
pub fn replace_label(ins_vec: &mut Vec<(u32, Instruction)>, label_map: &HashMap<String, u32>) {
    (0..ins_vec.len()).for_each(|i| {
        let ins_addr = ins_vec[i].0;
        let mut ins = &mut ins_vec[i].1;

        if let Some(Immediate::Label(name, _)) = &ins.imm {
            let label_addr = label_map.get(name);
            match label_addr {
                Some(&label_addr) => {
                    let las: i32 = label_addr.try_into().unwrap();
                    let ias: i32 = ins_addr.try_into().unwrap();
                    ins.imm = Some(Immediate::S32(las - (ias + 4)))
                }
                _ => panic!("No such label"),
            }
        }
    });
}
