// package org.bouncycastle.crypto;

// import org.bouncycastle.util.Strings;

// /**
//  * super class for all Password Based Encryption (PBE) parameter generator classes.
//  */
// public abstract class PBEParametersGenerator
// {
//     protected byte[]  password;
//     protected byte[]  salt;
//     protected int     iterationCount;

//     /**
//      * base constructor.
//      */
//     protected PBEParametersGenerator()
//     {
//     }

//     /**
//      * initialise the PBE generator.
//      *
//      * @param password the password converted into bytes (see below).
//      * @param salt the salt to be mixed with the password.
//      * @param iterationCount the number of iterations the "mixing" function
//      * is to be applied for.
//      */
//     public void init(
//         byte[]  password,
//         byte[]  salt,
//         int     iterationCount)
//     {
//         this.password = password;
//         this.salt = salt;
//         this.iterationCount = iterationCount;
//     }

//     /**
//      * return the password byte array.
//      *
//      * @return the password byte array.
//      */
//     public byte[] getPassword()
//     {
//         return password;
//     }

//     /**
//      * return the salt byte array.
//      *
//      * @return the salt byte array.
//      */
//     public byte[] getSalt()
//     {
//         return salt;
//     }

//     /**
//      * return the iteration count.
//      *
//      * @return the iteration count.
//      */
//     public int getIterationCount()
//     {
//         return iterationCount;
//     }

//     /**
//      * generate derived parameters for a key of length keySize.
//      *
//      * @param keySize the length, in bits, of the key required.
//      * @return a parameters object representing a key.
//      */
//     public abstract CipherParameters generateDerivedParameters(int keySize);

//     /**
//      * generate derived parameters for a key of length keySize, and
//      * an initialisation vector (IV) of length ivSize.
//      *
//      * @param keySize the length, in bits, of the key required.
//      * @param ivSize the length, in bits, of the iv required.
//      * @return a parameters object representing a key and an IV.
//      */
//     public abstract CipherParameters generateDerivedParameters(int keySize, int ivSize);

//     /**
//      * generate derived parameters for a key of length keySize, specifically
//      * for use with a MAC.
//      *
//      * @param keySize the length, in bits, of the key required.
//      * @return a parameters object representing a key.
//      */
//     public abstract CipherParameters generateDerivedMacParameters(int keySize);

//     /**
//      * converts a password to a byte array according to the scheme in
//      * PKCS5 (ascii, no padding)
//      *
//      * @param password a character array representing the password.
//      * @return a byte array representing the password.
//      */
//     public static byte[] PKCS5PasswordToBytes(
//         char[]  password)
//     {
//         if (password != null)
//         {
//             byte[]  bytes = new byte[password.length];

//             for (int i = 0; i != bytes.length; i++)
//             {
//                 bytes[i] = (byte)password[i];
//             }

//             return bytes;
//         }
//         else
//         {
//             return new byte[0];
//         }
//     }

//     /**
//      * converts a password to a byte array according to the scheme in
//      * PKCS5 (UTF-8, no padding)
//      *
//      * @param password a character array representing the password.
//      * @return a byte array representing the password.
//      */
//     public static byte[] PKCS5PasswordToUTF8Bytes(
//         char[]  password)
//     {
//         if (password != null)
//         {
//             return Strings.toUTF8ByteArray(password);
//         }
//         else
//         {
//             return new byte[0];
//         }
//     }

//     /**
//      * converts a password to a byte array according to the scheme in
//      * PKCS12 (unicode, big endian, 2 zero pad bytes at the end).
//      *
//      * @param password a character array representing the password.
//      * @return a byte array representing the password.
//      */
//     public static byte[] PKCS12PasswordToBytes(
//         char[]  password)
//     {
//         if (password != null && password.length > 0)
//         {
//                                        // +1 for extra 2 pad bytes.
//             byte[]  bytes = new byte[(password.length + 1) * 2];

//             for (int i = 0; i != password.length; i ++)
//             {
//                 bytes[i * 2] = (byte)(password[i] >>> 8);
//                 bytes[i * 2 + 1] = (byte)password[i];
//             }

//             return bytes;
//         }
//         else
//         {
//             return new byte[0];
//         }
//     }
// }
use crate::crypto::CipherParameters;

/// 對應 Java 的 PBEParametersGenerator 抽象類別
pub trait PbeParametersGenerator {
    /// 初始化生成器
    fn init(&mut self, password: Vec<u8>, salt: Vec<u8>, iteration_count: usize);

    /// 生成指定長度的金鑰參數 (Bits)
    fn generate_derived_parameters(&self, key_size: usize) -> Box<dyn CipherParameters>;

    /// 生成指定長度的金鑰與 IV 參數 (Bits)
    fn generate_derived_parameters_with_iv(&self, key_size: usize, iv_size: usize) -> Box<dyn CipherParameters>;

    /// 生成指定長度的 MAC 金鑰參數 (Bits)
    fn generate_derived_mac_parameters(&self, key_size: usize) -> Box<dyn CipherParameters>;
}

/// 密碼轉換工具函數 (對應 Java 的靜態方法)
pub struct PbeUtils;

impl PbeUtils {
    /// PKCS5: 簡單 8-bit 轉換 (ASCII 截斷)
    pub fn pkcs5_password_to_bytes(password: &[char]) -> Vec<u8> {
        password.iter().map(|&c| c as u8).collect()
    }

    /// PKCS5: UTF-8 轉換
    pub fn pkcs5_password_to_utf8_bytes(password: &[char]) -> Vec<u8> {
        password.iter().collect::<String>().into_bytes()
    }

    /// PKCS12: UTF-16 大端序，末尾補兩個 0 字節
    pub fn pkcs12_password_to_bytes(password: &[char]) -> Vec<u8> {
        if password.is_empty() {
            return Vec::new();
        }
        // 長度為 (len + 1) * 2
        let mut bytes = Vec::with_capacity((password.len() + 1) * 2);
        for &c in password {
            let val = c as u16;
            bytes.push((val >> 8) as u8);
            bytes.push((val & 0xff) as u8);
        }
        // 末尾補兩個零字節 (Null terminator)
        bytes.push(0);
        bytes.push(0);
        bytes
    }
}
pub struct Pkcs5S2ParametersGenerator {
    password: Vec<u8>,
    salt: Vec<u8>,
    iteration_count: usize,
}

impl PbeParametersGenerator for Pkcs5S2ParametersGenerator {
    fn init(&mut self, password: Vec<u8>, salt: Vec<u8>, iteration_count: usize) {
        self.password = password;
        self.salt = salt;
        self.iteration_count = iteration_count;
    }

    fn generate_derived_parameters(&self, key_size: usize) -> Box<dyn CipherParameters> {
        // 執行 PBKDF2 算法邏輯...
        todo!()
    }
    
    // ... 實現其他方法
}
