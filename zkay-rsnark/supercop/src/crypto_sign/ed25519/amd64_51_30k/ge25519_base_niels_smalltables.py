import re

# 將你所有的十六進制數據貼在下面這個字串中
raw_data = """
{{{0x00003905d740913e, 0x0000ba2817d673a2, 0x00023e2827f4e67c, 0x000133d2e0c21a34, 0x00044fd2f9298f81}},
 {{0x000493c6f58c3b85, 0x0000df7181c325f7, 0x0000f50b0b3e4cb7, 0x0005329385a44c32, 0x00007cf9d3a33d4b}},
 {{0x000515674b6fbb59, 0x00001dd454bd5b77, 0x00055f1be90784fc, 0x00066566ea4e8e64, 0x0004f0ebe1faf16e}}},
"""


def process_file(input_path):
    # 1. 從文件讀取原始字串
    with open(input_path, "r", encoding="utf-8") as f:
        raw_text = f.read()

    # 2. 使用正則表達式抓取所有 Fe25519 數組 (5個十六進制值)
    # 匹配模式: {0x..., 0x..., 0x..., 0x..., 0x...}
    fe_pattern = r'\{([0x0-9a-fA-F, \s]+)\}'
    all_fes = re.findall(fe_pattern, raw_text)

    # 3. 輸出 Rust 格式
    for i in range(0, len(all_fes), 3):
        if i + 2 < len(all_fes):
            print(f"Ge25519Niels {{")
            print(f"    y_plus_x: Fe25519([{all_fes[i].strip()}]),")
            print(f"    y_minus_x: Fe25519([{all_fes[i+1].strip()}]),")
            print(f"    xy2d: Fe25519([{all_fes[i+2].strip()}]),")
            print(f"}},")

# 執行
process_file("ge25519_base_niels_smalltables.data")

def convert_to_rust(text):
    # 匹配內層大括號中的 5 個十六進制數值
    fe_pattern = r'\{([0x0-9a-fA-F, \n\t]+)\}'
    all_fes = re.findall(fe_pattern, text)
    
    # 每 3 個 Fe25519 組成一個 Niels 點
    rust_entries = []
    for i in range(0, len(all_fes), 3):
        if i + 2 >= len(all_fes): break
        
        # 清理換行與空格
        def clean(s): return ", ".join([x.strip() for x in s.split(',') if x.strip()])
        
        entry = (
            "    Ge25519Niels {\n"
            f"        y_plus_x: Fe25519([{clean(all_fes[i])}]),\n"
            f"        y_minus_x: Fe25519([{clean(all_fes[i+1])}]),\n"
            f"        xy2d: Fe25519([{clean(all_fes[i+2])}]),\n"
            "    },"
        )
        rust_entries.append(entry)
    
    header = "pub const GE25519_BASE_MULTIPLES_NIELS: [Ge25519Niels; " + str(len(rust_entries)) + "] = [\n"
    return header + "\n".join(rust_entries) + "\n];"

print(convert_to_rust(raw_data))
