#[derive(Debug)]
pub enum Gate<'a> {
    Total(u32),
    Input(u32),
    Output(u32),
    NizkInput(u32),
    // 类型, 输入数, 输入列表, 输出数, 输出列表
    Complex {
        typ: &'a str,
        inputs: Vec<u32>,
        outputs: Vec<u32>,
    },
}
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, multispace0, space0, space1},
    combinator::{map, opt, rest},
    multi::separated_list0,
    sequence::{delimited, tuple},
};

// --- 基础解析器 ---
fn parse_u32(input: &str) -> IResult<&str, u32> {
    digit1(input).map(|(i, v)| (i, v.parse().unwrap()))
}

fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(tag("<"), separated_list0(space1, parse_u32), tag(">")).parse(input)
}

// --- 核心 Gate 解析器 ---
// 兼容: split in 1 <17> out 32 <23 24 ...>
// 兼容: const-mul-xxx in 1 <0> out 1 <887>
fn parse_complex_gate(input: &str) -> IResult<&str, Gate> {
    let (input, (typ, _, _, _, _, _, inputs, _, _, _, _, _, outputs)) = tuple((
        take_while1(|c: char| !c.is_whitespace() && c != '#'), // 类型名
        space1,
        // 2. 关键字 "in" (这是你之前报错的关键！)
        tag("in"),
        space1,
        // 3. 输入数量 (直接跳过，因为列表长度已知)
        parse_u32,
        space1,
        // 4. 输入列表 <...>
        parse_list,
        space1,
        // 5. 关键字 "out"
        tag("out"),
        space1,
        // 6. 输出数量
        parse_u32,
        space1,
        // 7. 输出列表 <...>
        parse_list,
    ))
    .parse(input)?;

    // 自动清理掉行尾的注释
    let (input, _) = opt(tuple((space0, tag("#"), take_while1(|_| true)))).parse(input)?;

    Ok((
        input,
        Gate::Complex {
            typ,
            inputs,
            outputs,
        },
    ))
}

// --- 总入口 ---
pub fn parse_line(input: &str) -> IResult<&str, Option<Gate>> {
    let input = input.trim();
    if input.is_empty() || input.starts_with('#') {
        return Ok(("", None));
    }

    // 使用 alt 尝试不同的匹配模式
    // 关键点：我们需要先匹配指令，然后忽略后面所有的内容（包括空格和注释）
    let (remaining, result) = alt((
        // total 378450
        map(tuple((tag("total"), space1, parse_u32)), |(_, _, id)| {
            Some(Gate::Total(id))
        }),
        // input 0
        map(tuple((tag("input"), space1, parse_u32)), |(_, _, id)| {
            Some(Gate::Input(id))
        }),
        // nizkinput 2
        map(
            tuple((tag("nizkinput"), space1, parse_u32)),
            |(_, _, id)| Some(Gate::NizkInput(id)),
        ),
        // output 100
        map(tuple((tag("output"), space1, parse_u32)), |(_, _, id)| {
            Some(Gate::Output(id))
        }),
        // 复杂的 Gate (add, mul, split, pack, xor...)
        map(parse_complex_gate, Some),
    ))
    .parse(input)
    .map_err(|e| {
        // 如果报错，打印出当前在哪一行报错，方便调试
        eprintln!("Parse Error at line: [{}]", input);
        e
    })?;

    // // 2. 【关键修正】显式消耗掉剩余的所有字符（包括空格和注释）
    // // 这样 remaining 就会变成空字符串 ""，从而避免上层检查失败
    // let (remaining, _) = rest(remaining)?;

    Ok(("", result))
}
pub fn parse(input: &str) -> Vec<Gate> {
    input
        .split("\n")
        .filter_map(|line| {
            (!line.trim().is_empty()).then(|| parse_line(line.trim()).expect(line).1.expect(line))
        })
        .collect()
}
pub fn evaluate_gate(gate: Gate, wire_values: &mut Vec<i128>) {
    match gate {
        Gate::Complex {
            typ,
            inputs,
            outputs,
            ..
        } => {
            match typ {
                "add" => {
                    let sum: i128 = inputs.iter().map(|&id| wire_values[id as usize]).sum();
                    wire_values[outputs[0] as usize] = sum;
                }
                "mul" => {
                    wire_values[outputs[0] as usize] =
                        wire_values[inputs[0] as usize] * wire_values[inputs[1] as usize];
                }
                // 处理 const-mul-xxxx
                t if t.starts_with("const-mul-") => {
                    let hex_str = t.strip_prefix("const-mul-").unwrap();
                    // let val = parse_hex(hex_str);
                    // wire_values[outputs[0] as usize] = val * wire_values[inputs[0] as usize];
                }
                _ => {}
            }
        }
        _ => {}
    }
}
use nom::{
    character::complete::{hex_digit1, line_ending},
    combinator::map_res,
    // multi::separated_list0,
    sequence::separated_pair,
    // IResult,
};
use num_bigint::BigUint;
use num_traits::Num;
pub fn read_field_element_from_hex(input_str: &str) -> BigUint {
    // 相当于 mpz_init_set_str(integ, inputStr, 16)
    // 注意：Rust 的 parse_radix 不需要中转十进制字符串
    BigUint::from_str_radix(input_str, 10).unwrap_or_else(|_| {
        BigUint::from_str_radix(input_str, 16).unwrap_or_else(|_| BigUint::from(0u32))
    })
}
// 模拟你的 Field 结构
#[derive(Debug)]
pub struct WireEntry {
    pub id: u32,
    pub value: BigUint,
}

fn parse_input_line(input: &str) -> IResult<&str, WireEntry> {
    map(
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u32>()),
            space1,
            // 直接解析十六进制部分
            map_res(hex_digit1, |s: &str| BigUint::from_str_radix(s, 16)),
        ),
        |(id, value)| WireEntry { id, value },
    )
    .parse(input)
}

// 解析整个文件
pub fn parse_all_inputs(input: &str) -> IResult<&str, Vec<WireEntry>> {
    separated_list0(line_ending, parse_input_line).parse(&input)
}
