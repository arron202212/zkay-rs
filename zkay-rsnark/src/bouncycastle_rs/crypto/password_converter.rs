// package org.bouncycastle.crypto;

// /**
//  * Standard char[] to byte[] converters for password based derivation algorithms.
//  */
// public enum PasswordConverter
//     implements CharToByteConverter
// {
//     /**
//      * Do a straight char[] to 8 bit conversion.
//      */
//     ASCII
//         {
//             public String getType()
//             {
//                 return "ASCII";
//             }

//             public byte[] convert(char[] password)
//             {
//                 return PBEParametersGenerator.PKCS5PasswordToBytes(password);
//             }
//         },
//     /**
//      * Do a char[] conversion by producing UTF-8 data.
//      */
//     UTF8
//         {
//             public String getType()
//             {
//                 return "UTF8";
//             }

//             public byte[] convert(char[] password)
//             {
//                 return PBEParametersGenerator.PKCS5PasswordToUTF8Bytes(password);
//             }
//         },
//     /**
//      * Do char[] to BMP conversion (i.e. 2 bytes per character).
//      */
//     PKCS12
//         {
//             public String getType()
//             {
//                 return "PKCS12";
//             }

//             public byte[] convert(char[] password)
//             {
//                 return PBEParametersGenerator.PKCS12PasswordToBytes(password);
//             }
//         };
// }
/// 對應 Java 的 PasswordConverter 枚舉
pub enum PasswordConverter {
    ASCII,
    UTF8,
    PKCS12,
}

impl CharToByteConverter for PasswordConverter {
    fn get_type(&self) -> &str {
        match self {
            PasswordConverter::ASCII => "ASCII",
            PasswordConverter::UTF8 => "UTF8",
            PasswordConverter::PKCS12 => "PKCS12",
        }
    }

    fn convert(&self, password: &[char]) -> Vec<u8> {
        match self {
            // 對應 PKCS5PasswordToBytes: 僅保留低 8 位
            PasswordConverter::ASCII => {
                password.iter().map(|&c| c as u8).collect()
            }
            
            // 對應 PKCS5PasswordToUTF8Bytes: 標準 UTF-8 編碼
            PasswordConverter::UTF8 => {
                password.iter().collect::<String>().into_bytes()
            }
            
            // 對應 PKCS12PasswordToBytes: 大端序 UTF-16 (BMP)，末尾通常帶兩個 0 字節
            PasswordConverter::PKCS12 => {
                let mut bytes = Vec::with_capacity(password.len() * 2 + 2);
                for &c in password {
                    let u16_val = c as u16; // 轉為 UTF-16 碼元
                    bytes.push((u16_val >> 8) as u8); // 高位
                    bytes.push((u16_val & 0xff) as u8); // 低位
                }
                // PKCS#12 規範通常要求以 null 終止 (2 個 zero bytes)
                bytes.push(0);
                bytes.push(0);
                bytes
            }
        }
    }
}
fn main(){
let converter = PasswordConverter::UTF8;
let password_chars: Vec<char> = "P@ssword123".chars().collect();
let encoded = converter.convert(&password_chars);

println!("Type: {}, Bytes: {:?}", converter.get_type(), encoded);
}