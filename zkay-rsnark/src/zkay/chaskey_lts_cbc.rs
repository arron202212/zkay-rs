




use  zkay::zkay_util::unsigned_bigint_to_bytes;
use  zkay::zkay_util::unsigned_bytes_to_bigint;

pub struct ChaskeyLtsCbc;
impl  ChaskeyLtsCbc{

     fn  parse(val:String , len:i32 )->Vec<byte> {
        return unsignedBigintToBytes(BigInteger::new(val, 16), len);
    }

      const blocksize:i32 = 16;
      const ivlen:i32 = blocksize;
      const keylen:i32 = blocksize;
      const  msglen:i32 = 2*blocksize; // Must be multiple of blocksize

 pub fn crypt(encrypt:bool , key:Vec<byte>, iv:Vec<byte> , input:Vec<byte>)->       Vec<byte> {
        // Initialize chaskey cipher in cbc mode
let chaskeyEngine =  ChaskeyLTSEngine::new();
let cbc =  CBCBlockCipher::new(chaskeyEngine);
let cipher =  BufferedBlockCipher::new(cbc); // Don't need padding since size is always statically known in zkay and input is multiple of block size
let params =  ParametersWithIV::new(KeyParameter::new(key), iv);
        cipher.init(encrypt, params);

        // Encrypt / Decrypt
            assert!(cipher.getOutputSize(input.length) == input.length ,"Wrong size");
let outbuf =  vec![byte::default();cipher.getOutputSize(input.length)];
let out_size =  cipher.processBytes(input, 0, input.length, outbuf, 0);
            assert!(cipher.doFinal(outbuf, out_size) == 0,"Input not aligned to block size");

        return outbuf;
    }
}
    pub  fn  main(args:Vec<String>){
        // Parse inputs
            assert!(args.length == 4,"expected 4 arguments [enc|dec, key, iv, plain|cipher]");
assert!(args[0]=="enc"||args[0]=="dec","First argument must be either 'enc' or 'dec'");
        let  enc=args[0]=="enc";
        
let key =  parse(args[1], keylen);
let iv =  parse(args[2], ivlen);
let input =  parse(args[3], msglen);

        // Perform encryption/decryption
let output =  crypt(enc, key, iv, input);

        // Output result
        println!(unsignedBytesToBigInt(output).toString(16));
    }

