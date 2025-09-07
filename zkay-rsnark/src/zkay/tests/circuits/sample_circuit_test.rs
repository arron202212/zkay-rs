#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    util::util::{BigInteger, Util},
    zkay::tests::circuits::{
        sample_dec_circuit, sample_enc_circuit, sample_mul_circuit, sample_rehom_circuit,
    },
};

#[cfg(test)]
mod test {
    use super::*;
    #[inline]
    fn pbixs(bs: &str) -> String {
        Util::parse_big_int(bs).to_str_radix(16)
    }
    #[test]
    pub fn test_sample_enc_circuit_compile() {
        sample_enc_circuit::main(vec!["compile".to_owned()]);
    }

    #[test]
    pub fn test_sample_enc_circuit_prove() {
        // zk__out0_cipher = Enc(3, glob_key_Elgamal__owner, zk__out0_cipher_R)
        // zk__in0_cipher_val = <42>
        // zk__out1_cipher = <42 + 3>

        // 'glob_key_Elgamal__owner' = in[0:2]
        // '_zk__foo.0.zk__in0_cipher_val' = in[2:6]
        // '_zk__foo.0.zk__out0_cipher' = out[6:10]
        // '_zk__foo.0.zk__out1_cipher' = out[10:14]
        // '_zk__foo.0.zk__out0_cipher_R' = priv[0:1]

        let pkx =
            pbixs("2543111965495064707612623550577403881714453669184859408922451773306175031318");
        let pky =
            pbixs("20927827475527585117296730644692999944545060105133073020125343132211068382185");
        let out0_r =
            pbixs("4992017890738015216991440853823451346783754228142718316135811893930821210517");
        let out0_c1x =
            pbixs("17990166387038654353532224054392704246273066434684370089496246721960255371329");
        let out0_c1y =
            pbixs("15866190370882469414665095798958204707796441173247149326160843221134574846694");
        let out0_c2x =
            pbixs("20611619168289996179170076826255394452844088446249762902489426332728314449540");
        let out0_c2y =
            pbixs("15977019707513990678856869992098745075741339619245698210811867116749537641408");
        let in0_c1x =
            pbixs("20000451794290380375914691798920385097103434955980148521154607378788339649411");
        let in0_c1y =
            pbixs("3379688933589504078077257631396507733503572474143535438012650064116108361323");
        let in0_c2x =
            pbixs("17692342451347357823507390319100928261770955547170665908868317402407559496644");
        let in0_c2y =
            pbixs("10685998684618216791975894032544668032271032005273052481243516059128881465545");
        let out1_c1x =
            pbixs("18885199402227818148211810144232318738102042906622969713112212912459159846007");
        let out1_c1y =
            pbixs("11125071952177567933017599368067887482603292954302203070407920687516147981132");
        let out1_c2x =
            pbixs("20036470080915178878390944667725801469044803295396841663384258912114611255016");
        let out1_c2y =
            pbixs("18986185709423663075397883577572338596028661172318034324882291197251276265727");

        // argument order: in, out, priv
        let args = vec![
            "prove".to_owned(),
            pkx,
            pky,
            in0_c1x,
            in0_c1y,
            in0_c2x,
            in0_c2y,
            out0_c1x,
            out0_c1y,
            out0_c2x,
            out0_c2y,
            out1_c1x,
            out1_c1y,
            out1_c2x,
            out1_c2y,
            out0_r,
        ];
        sample_enc_circuit::main(args);
    }

    #[test]
    pub fn test_sample_dec_circuit_compile() {
        sample_dec_circuit::main(vec!["compile".to_owned()]);
    }

    #[test]
    pub fn test_sample_dec_circuit_prove() {
        // zk__in0_cipher_val = Enc(42, glob_key_Elgamal__me, ...)
        // secret0_plain_val = 42
        // zk__out0_plain_val = 42
        // zk__in0_cipher_val_R = (secret key of me)

        // 'glob_key_Elgamal__me' = in[0:2]
        // '_zk__bar.0.zk__in0_cipher_val' = in[2:6]
        // '_zk__bar.0.zk__out0_plain_val' = out[6:7]
        // '_zk__bar.0.secret0_plain_val' = priv[0:1]
        // '_zk__bar.0.zk__in0_cipher_val_R' = priv[1:2]

        let pkx =
            pbixs("2543111965495064707612623550577403881714453669184859408922451773306175031318");
        let pky =
            pbixs("20927827475527585117296730644692999944545060105133073020125343132211068382185");
        let in0_c1x =
            pbixs("17990166387038654353532224054392704246273066434684370089496246721960255371329");
        let in0_c1y =
            pbixs("15866190370882469414665095798958204707796441173247149326160843221134574846694");
        let in0_c2x =
            pbixs("13578016172019942326633412365679613147103709674318008979748420035774874659858");
        let in0_c2y =
            pbixs("15995926508900361671313404296634773295236345482179714831868518062689263430374");
        let out0 = BigInteger::from(42).to_str_radix(16);
        let secret0 = BigInteger::from(42).to_str_radix(16);
        let skey =
            pbixs("448344687855328518203304384067387474955750326758815542295083498526674852893");

        // argument order: in, out, priv
        let args = vec![
            "prove".to_owned(),
            pkx,
            pky,
            in0_c1x,
            in0_c1y,
            in0_c2x,
            in0_c2y,
            out0,
            secret0,
            skey,
        ];
        sample_dec_circuit::main(args);
    }

    #[test]
    pub fn test_sample_dec_circuit_prove_uninitialized() {
        let pkx =
            &pbixs("2543111965495064707612623550577403881714453669184859408922451773306175031318");
        let pky =
            &pbixs("20927827475527585117296730644692999944545060105133073020125343132211068382185");
        // uninitialized ciphertext
        let in0_c1x = "0";
        let in0_c1y = "0";
        let in0_c2x = "0";
        let in0_c2y = "0";
        let out0 = "0";
        let secret0 = "0";
        let skey = "0";

        // argument order: in, out, priv
        let args: Vec<String> = [
            "prove", pkx, pky, in0_c1x, in0_c1y, in0_c2x, in0_c2y, out0, secret0, skey,
        ]
        .iter()
        .map(|&v| v.to_string())
        .collect();
        sample_dec_circuit::main(args);
    }

    #[test]
    pub fn test_sample_mul_circuit_compile() {
        sample_mul_circuit::main(vec!["compile".to_owned()]);
    }

    #[test]
    pub fn test_sample_mul_circuit_prove() {
        let pkx =
            pbixs("2543111965495064707612623550577403881714453669184859408922451773306175031318");
        let pky =
            pbixs("20927827475527585117296730644692999944545060105133073020125343132211068382185");
        let in0_c1x =
            pbixs("1345914801503869804221332717328097414792076925078931355300970385489312303055");
        let in0_c1y =
            pbixs("3221919363851679888621419552929429977187872757564157365903242129276143826679");
        let in0_c2x =
            pbixs("17378197425436069497126136266495011617394395570683447945973025044739809585373");
        let in0_c2y =
            pbixs("15789009976977544046062803747743295235439704864191175329350822002296637150904");
        let out0_c1x =
            pbixs("1580977511543777394910122699548784426094904736600505129541556064495159060532");
        let out0_c1y =
            pbixs("16190941039609473953318528369093289558337201974880158341123285226900681258492");
        let out0_c2x =
            pbixs("18928854895111284332170004407067674892341217562252934285209587817233013254394");
        let out0_c2y =
            pbixs("8499515539957690392433056598772536511996242730894002020454275332668597388028");

        // argument order: in, out, priv
        let args = vec![
            "prove".to_owned(),
            pkx,
            pky,
            in0_c1x,
            in0_c1y,
            in0_c2x,
            in0_c2y,
            out0_c1x,
            out0_c1y,
            out0_c2x,
            out0_c2y,
        ];
        sample_mul_circuit::main(args);
    }

    #[test]
    pub fn test_sample_rehom_circuit_compile() {
        sample_rehom_circuit::main(vec!["compile".to_owned()]);
    }

    #[test]
    pub fn test_sample_rehom_circuit_prove() {
        let in0_pk1_rec =
            pbixs("6050894705972791558909254891616996674986810630226664089378712055547845936530");
        let in0_pk2_rec =
            pbixs("2098222428782134601768433132230348000710479961151701666623193700891051426734");
        let in0_pk1_me =
            pbixs("20482471737063427267743867277017992027916335915694495884029812150095009538665");
        let in0_pk2_me =
            pbixs("2064971568117740979897173202206849920364978834843195473834248767530206631140");
        let in0_x1x =
            pbixs("17651521056092576389327114851285097543355911207617425985108680402551109362320");
        let in0_x1y =
            pbixs("1979474016615069244406723045280100444351162006271416197677256792620315774762");
        let in0_x2x =
            pbixs("19853863594280103371645806654413426499353488214362048066055209302878076834605");
        let in0_x2y =
            pbixs("14030609827679157150694241338787994123586205285618036032395802092370201236392");
        let in0_b1x =
            pbixs("15288564466537518480361253319568821250986952964904376689834331007000477208583");
        let in0_b1y =
            pbixs("14499221764962898619288300583092505754061076652753865183142104711747753504290");
        let in0_b2x =
            pbixs("4453501154940040952535081235198610167580069962138595581877381781635748887580");
        let in0_b2y =
            pbixs("6003983121454395746745198309971673464065712423681748130677988590915163580200");

        let out0_c1x =
            pbixs("8753696203975329235585687250773552177062674794039810044171026034706968300723");
        let out0_c1y =
            pbixs("5610890170739048469029374457198992252393410759720420099039361376732580293843");
        let out0_c2x =
            pbixs("18367464182747295068302576897950293674363724573836344665957281230085766006943");
        let out0_c2y =
            pbixs("4537651393410589954105259703261476431845750308676046648775851995954139507000");

        let priv_x1 = pbixs("3");
        let priv_x1r =
            pbixs("94118587853295396393805395573620789675238716804156778422292198580847720966");
        let priv_rnd =
            pbixs("860776069412234168463308969065079006977381065448035510613391421262006397808");
        let priv_sk =
            pbixs("1690147589520195547977850797042220126332149894543617606884548383599391228484");

        // argument order: in, out, priv
        let args = vec![
            "prove".to_owned(),
            in0_pk1_rec,
            in0_pk2_rec,
            in0_pk1_me,
            in0_pk2_me,
            in0_x1x.clone(),
            in0_x1y.clone(),
            in0_x2x.clone(),
            in0_x2y.clone(),
            in0_b1x,
            in0_b1y,
            in0_b2x,
            in0_b2y,
            in0_x1x,
            in0_x1y,
            in0_x2x,
            in0_x2y,
            out0_c1x,
            out0_c1y,
            out0_c2x,
            out0_c2y,
            priv_x1.clone(),
            priv_x1r,
            priv_rnd,
            priv_x1,
            priv_sk,
        ];
        sample_rehom_circuit::main(args);
    }
}
