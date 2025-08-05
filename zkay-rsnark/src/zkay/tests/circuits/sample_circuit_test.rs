#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn testSampleEncCircuitCompile() {
        sample_enc_circuit::main(vec!["compile".to_owned()]);
    }

    #[test]
    pub fn testSampleEncCircuitProve() {
        /*
        zk__out0_cipher = Enc(3, glob_key_Elgamal__owner, zk__out0_cipher_R)
        zk__in0_cipher_val = <42>
        zk__out1_cipher = <42 + 3>

        'glob_key_Elgamal__owner' = in[0:2]
        '_zk__foo.0.zk__in0_cipher_val' = in[2:6]
        '_zk__foo.0.zk__out0_cipher' = out[6:10]
        '_zk__foo.0.zk__out1_cipher' = out[10:14]
        '_zk__foo.0.zk__out0_cipher_R' = priv[0:1]
         */
        let pkx = BigInteger
            .parse_bytes(
                b"2543111965495064707612623550577403881714453669184859408922451773306175031318",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let pky = BigInteger
            .parse_bytes(
                b"20927827475527585117296730644692999944545060105133073020125343132211068382185",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_r = BigInteger
            .parse_bytes(
                b"4992017890738015216991440853823451346783754228142718316135811893930821210517",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c1x = BigInteger
            .parse_bytes(
                b"17990166387038654353532224054392704246273066434684370089496246721960255371329",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c1y = BigInteger
            .parse_bytes(
                b"15866190370882469414665095798958204707796441173247149326160843221134574846694",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c2x = BigInteger
            .parse_bytes(
                b"20611619168289996179170076826255394452844088446249762902489426332728314449540",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c2y = BigInteger
            .parse_bytes(
                b"15977019707513990678856869992098745075741339619245698210811867116749537641408",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c1x = BigInteger
            .parse_bytes(
                b"20000451794290380375914691798920385097103434955980148521154607378788339649411",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c1y = BigInteger
            .parse_bytes(
                b"3379688933589504078077257631396507733503572474143535438012650064116108361323",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c2x = BigInteger
            .parse_bytes(
                b"17692342451347357823507390319100928261770955547170665908868317402407559496644",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c2y = BigInteger
            .parse_bytes(
                b"10685998684618216791975894032544668032271032005273052481243516059128881465545",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out1_c1x = BigInteger
            .parse_bytes(
                b"18885199402227818148211810144232318738102042906622969713112212912459159846007",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out1_c1y = BigInteger
            .parse_bytes(
                b"11125071952177567933017599368067887482603292954302203070407920687516147981132",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out1_c2x = BigInteger
            .parse_bytes(
                b"20036470080915178878390944667725801469044803295396841663384258912114611255016",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out1_c2y = BigInteger
            .parse_bytes(
                b"18986185709423663075397883577572338596028661172318034324882291197251276265727",
                10,
            )
            .unwrap()
            .to_str_radix(16);

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
    pub fn testSampleDecCircuitCompile() {
        sample_dec_circuit::main(vec!["compile".to_owned()]);
    }

    #[test]
    pub fn testSampleDecCircuitProve() {
        /*
        zk__in0_cipher_val = Enc(42, glob_key_Elgamal__me, ...)
        secret0_plain_val = 42
        zk__out0_plain_val = 42
        zk__in0_cipher_val_R = (secret key of me)

        'glob_key_Elgamal__me' = in[0:2]
        '_zk__bar.0.zk__in0_cipher_val' = in[2:6]
        '_zk__bar.0.zk__out0_plain_val' = out[6:7]
        '_zk__bar.0.secret0_plain_val' = priv[0:1]
        '_zk__bar.0.zk__in0_cipher_val_R' = priv[1:2]
         */

        let pkx = BigInteger
            .parse_bytes(
                b"2543111965495064707612623550577403881714453669184859408922451773306175031318",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let pky = BigInteger
            .parse_bytes(
                b"20927827475527585117296730644692999944545060105133073020125343132211068382185",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c1x = BigInteger
            .parse_bytes(
                b"17990166387038654353532224054392704246273066434684370089496246721960255371329",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c1y = BigInteger
            .parse_bytes(
                b"15866190370882469414665095798958204707796441173247149326160843221134574846694",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c2x = BigInteger
            .parse_bytes(
                b"13578016172019942326633412365679613147103709674318008979748420035774874659858",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c2y = BigInteger
            .parse_bytes(
                b"15995926508900361671313404296634773295236345482179714831868518062689263430374",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0 = BigInteger.parse_bytes(b"42", 10).unwrap().to_str_radix(16);
        let secret0 = BigInteger.parse_bytes(b"42", 10).unwrap().to_str_radix(16);
        let skey = BigInteger
            .parse_bytes(
                b"448344687855328518203304384067387474955750326758815542295083498526674852893",
                10,
            )
            .unwrap()
            .to_str_radix(16);

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
    pub fn testSampleDecCircuitProveUninitialized() {
        let pkx = BigInteger
            .parse_bytes(
                b"2543111965495064707612623550577403881714453669184859408922451773306175031318",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let pky = BigInteger
            .parse_bytes(
                b"20927827475527585117296730644692999944545060105133073020125343132211068382185",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        // uninitialized ciphertext
        let in0_c1x = BigInteger.parse_bytes(b"0", 10).unwrap().to_str_radix(16);
        let in0_c1y = BigInteger.parse_bytes(b"0", 10).unwrap().to_str_radix(16);
        let in0_c2x = BigInteger.parse_bytes(b"0", 10).unwrap().to_str_radix(16);
        let in0_c2y = BigInteger.parse_bytes(b"0", 10).unwrap().to_str_radix(16);
        let out0 = BigInteger.parse_bytes(b"0", 10).unwrap().to_str_radix(16);
        let secret0 = BigInteger.parse_bytes(b"0", 10).unwrap().to_str_radix(16);
        let skey = BigInteger.parse_bytes(b"0", 10).unwrap().to_str_radix(16);

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
    pub fn testSampleMulCircuitCompile() {
        sample_mul_circuit::main(vec!["compile".to_owned()]);
    }

    #[test]
    pub fn testSampleMulCircuitProve() {
        let pkx = BigInteger
            .parse_bytes(
                b"2543111965495064707612623550577403881714453669184859408922451773306175031318",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let pky = BigInteger
            .parse_bytes(
                b"20927827475527585117296730644692999944545060105133073020125343132211068382185",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c1x = BigInteger
            .parse_bytes(
                b"1345914801503869804221332717328097414792076925078931355300970385489312303055",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c1y = BigInteger
            .parse_bytes(
                b"3221919363851679888621419552929429977187872757564157365903242129276143826679",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c2x = BigInteger
            .parse_bytes(
                b"17378197425436069497126136266495011617394395570683447945973025044739809585373",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_c2y = BigInteger
            .parse_bytes(
                b"15789009976977544046062803747743295235439704864191175329350822002296637150904",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c1x = BigInteger
            .parse_bytes(
                b"1580977511543777394910122699548784426094904736600505129541556064495159060532",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c1y = BigInteger
            .parse_bytes(
                b"16190941039609473953318528369093289558337201974880158341123285226900681258492",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c2x = BigInteger
            .parse_bytes(
                b"18928854895111284332170004407067674892341217562252934285209587817233013254394",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c2y = BigInteger
            .parse_bytes(
                b"8499515539957690392433056598772536511996242730894002020454275332668597388028",
                10,
            )
            .unwrap()
            .to_str_radix(16);

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
    pub fn testSampleRehomCircuitCompile() {
        sample_rehom_circuit::main(vec!["compile".to_owned()]);
    }

    #[test]
    pub fn testSampleRehomCircuitProve() {
        let in0_pk1_rec = BigInteger
            .parse_bytes(
                b"6050894705972791558909254891616996674986810630226664089378712055547845936530",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_pk2_rec = BigInteger
            .parse_bytes(
                b"2098222428782134601768433132230348000710479961151701666623193700891051426734",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_pk1_me = BigInteger
            .parse_bytes(
                b"20482471737063427267743867277017992027916335915694495884029812150095009538665",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_pk2_me = BigInteger
            .parse_bytes(
                b"2064971568117740979897173202206849920364978834843195473834248767530206631140",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_x1x = BigInteger
            .parse_bytes(
                b"17651521056092576389327114851285097543355911207617425985108680402551109362320",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_x1y = BigInteger
            .parse_bytes(
                b"1979474016615069244406723045280100444351162006271416197677256792620315774762",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_x2x = BigInteger
            .parse_bytes(
                b"19853863594280103371645806654413426499353488214362048066055209302878076834605",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_x2y = BigInteger
            .parse_bytes(
                b"14030609827679157150694241338787994123586205285618036032395802092370201236392",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_b1x = BigInteger
            .parse_bytes(
                b"15288564466537518480361253319568821250986952964904376689834331007000477208583",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_b1y = BigInteger
            .parse_bytes(
                b"14499221764962898619288300583092505754061076652753865183142104711747753504290",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_b2x = BigInteger
            .parse_bytes(
                b"4453501154940040952535081235198610167580069962138595581877381781635748887580",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let in0_b2y = BigInteger
            .parse_bytes(
                b"6003983121454395746745198309971673464065712423681748130677988590915163580200",
                10,
            )
            .unwrap()
            .to_str_radix(16);

        let out0_c1x = BigInteger
            .parse_bytes(
                b"8753696203975329235585687250773552177062674794039810044171026034706968300723",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c1y = BigInteger
            .parse_bytes(
                b"5610890170739048469029374457198992252393410759720420099039361376732580293843",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c2x = BigInteger
            .parse_bytes(
                b"18367464182747295068302576897950293674363724573836344665957281230085766006943",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let out0_c2y = BigInteger
            .parse_bytes(
                b"4537651393410589954105259703261476431845750308676046648775851995954139507000",
                10,
            )
            .unwrap()
            .to_str_radix(16);

        let priv_x1 = BigInteger.parse_bytes(b"3", 10).unwrap().to_str_radix(16);
        let priv_x1r = BigInteger
            .parse_bytes(
                b"94118587853295396393805395573620789675238716804156778422292198580847720966",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let priv_rnd = BigInteger
            .parse_bytes(
                b"860776069412234168463308969065079006977381065448035510613391421262006397808",
                10,
            )
            .unwrap()
            .to_str_radix(16);
        let priv_sk = BigInteger
            .parse_bytes(
                b"1690147589520195547977850797042220126332149894543617606884548383599391228484",
                10,
            )
            .unwrap()
            .to_str_radix(16);

        // argument order: in, out, priv
        let args = vec![
            "prove".to_owned(),
            in0_pk1_rec,
            in0_pk2_rec,
            in0_pk1_me,
            in0_pk2_me,
            in0_x1x,
            in0_x1y,
            in0_x2x,
            in0_x2y,
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
            priv_x1,
            priv_x1r,
            priv_rnd,
            priv_x1,
            priv_sk,
        ];
        sample_rehom_circuit::main(args);
    }
}
