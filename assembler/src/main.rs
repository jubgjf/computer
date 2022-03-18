use crate::token::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod token;

fn main() {
    let src_path = "assembler/test/test.s";

    if let Ok(lines) = read_lines(src_path) {
        // 当前行对应的地址
        let mut address = 0;

        let mut label_map = HashMap::<String, u32>::new();
        let mut ins_vec = Vec::<(u32, Instruction)>::new();

        // 分析每一行
        lines.for_each(|line| {
            if let Ok(l) = line {
                let lt = l.trim();
                if !lt.is_empty() {
                    match lt.chars().next() {
                        Some(';') => { /* 此行为注释 */ }
                        _ => {
                            // 此行为标签
                            let possible_label = lt.split_whitespace().next().unwrap();
                            if possible_label.chars().rev().next().unwrap() == ':' {
                                let label_name =
                                    &possible_label.to_string()[..possible_label.len() - 1];
                                // println!("[Label]: {}", label_name);
                                label_map.insert(label_name.to_string(), address);
                            } else {
                                // 此行为指令
                                let ins = analyze_line(l);
                                // println!("[Addr]: 0x{:x} [Ins]: {}", address, ins);
                                ins_vec.push((address, ins));
                                address += 4;
                            }
                        }
                    }
                } else {
                    // 此行为空行
                }
            }
        });

        // 将指令中的标签替换为立即数
        replace_label(&mut ins_vec, &label_map);
        for (addr, ins) in &ins_vec {
            println!("[Addr]: 0x{:x} [Ins]: {}", addr, ins);
        }
    }
}

/// 读取文件
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
