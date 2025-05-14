

pub struct SampleCircuitTest {

    
    pub   testSampleEncCircuitCompile() {
        SampleEncCircuit.main(vec![String::default();] {"compile"});
    }

    
    pub   testSampleEncCircuitProve() {
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
let pkx = BigInteger::new("2543111965495064707612623550577403881714453669184859408922451773306175031318").toString(16);
let pky = BigInteger::new("20927827475527585117296730644692999944545060105133073020125343132211068382185").toString(16);
let out0_r = BigInteger::new("4992017890738015216991440853823451346783754228142718316135811893930821210517").toString(16);
let out0_c1x = BigInteger::new("17990166387038654353532224054392704246273066434684370089496246721960255371329").toString(16);
let out0_c1y = BigInteger::new("15866190370882469414665095798958204707796441173247149326160843221134574846694").toString(16);
let out0_c2x = BigInteger::new("20611619168289996179170076826255394452844088446249762902489426332728314449540").toString(16);
let out0_c2y = BigInteger::new("15977019707513990678856869992098745075741339619245698210811867116749537641408").toString(16);
let in0_c1x = BigInteger::new("20000451794290380375914691798920385097103434955980148521154607378788339649411").toString(16);
let in0_c1y = BigInteger::new("3379688933589504078077257631396507733503572474143535438012650064116108361323").toString(16);
let in0_c2x = BigInteger::new("17692342451347357823507390319100928261770955547170665908868317402407559496644").toString(16);
let in0_c2y = BigInteger::new("10685998684618216791975894032544668032271032005273052481243516059128881465545").toString(16);
let out1_c1x = BigInteger::new("18885199402227818148211810144232318738102042906622969713112212912459159846007").toString(16);
let out1_c1y = BigInteger::new("11125071952177567933017599368067887482603292954302203070407920687516147981132").toString(16);
let out1_c2x = BigInteger::new("20036470080915178878390944667725801469044803295396841663384258912114611255016").toString(16);
let out1_c2y = BigInteger::new("18986185709423663075397883577572338596028661172318034324882291197251276265727").toString(16);

        // argument order: in, out, priv
let args = vec![String::default();]{"prove", pkx, pky, in0_c1x, in0_c1y, in0_c2x, in0_c2y,
                out0_c1x, out0_c1y, out0_c2x, out0_c2y, out1_c1x, out1_c1y, out1_c2x, out1_c2y, out0_r};
        SampleEncCircuit.main(args);
    }

    
    pub   testSampleDecCircuitCompile() {
        SampleDecCircuit.main(vec![String::default();] {"compile"});
    }

    
    pub   testSampleDecCircuitProve() {
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

let pkx = BigInteger::new("2543111965495064707612623550577403881714453669184859408922451773306175031318").toString(16);
let pky = BigInteger::new("20927827475527585117296730644692999944545060105133073020125343132211068382185").toString(16);
let in0_c1x = BigInteger::new("17990166387038654353532224054392704246273066434684370089496246721960255371329").toString(16);
let in0_c1y = BigInteger::new("15866190370882469414665095798958204707796441173247149326160843221134574846694").toString(16);
let in0_c2x = BigInteger::new("13578016172019942326633412365679613147103709674318008979748420035774874659858").toString(16);
let in0_c2y = BigInteger::new("15995926508900361671313404296634773295236345482179714831868518062689263430374").toString(16);
let out0 = BigInteger::new("42").toString(16);
let secret0 = BigInteger::new("42").toString(16);
let skey = BigInteger::new("448344687855328518203304384067387474955750326758815542295083498526674852893").toString(16);

        // argument order: in, out, priv
let args = vec![String::default();]{"prove", pkx, pky, in0_c1x, in0_c1y, in0_c2x, in0_c2y, out0, secret0, skey};
        SampleDecCircuit.main(args);
    }

    
    pub   testSampleDecCircuitProveUninitialized() {
let pkx = BigInteger::new("2543111965495064707612623550577403881714453669184859408922451773306175031318").toString(16);
let pky = BigInteger::new("20927827475527585117296730644692999944545060105133073020125343132211068382185").toString(16);
        // uninitialized ciphertext
let in0_c1x = BigInteger::new("0").toString(16);
let in0_c1y = BigInteger::new("0").toString(16);
let in0_c2x = BigInteger::new("0").toString(16);
let in0_c2y = BigInteger::new("0").toString(16);
let out0 = BigInteger::new("0").toString(16);
let secret0 = BigInteger::new("0").toString(16);
let skey = BigInteger::new("0").toString(16);

        // argument order: in, out, priv
let args = vec![String::default();]{"prove", pkx, pky, in0_c1x, in0_c1y, in0_c2x, in0_c2y, out0, secret0, skey};
        SampleDecCircuit.main(args);
    }

    
    pub   testSampleMulCircuitCompile() {
        SampleMulCircuit.main(vec![String::default();] {"compile"});
    }

    
    pub   testSampleMulCircuitProve() {
let pkx = BigInteger::new("2543111965495064707612623550577403881714453669184859408922451773306175031318").toString(16);
let pky = BigInteger::new("20927827475527585117296730644692999944545060105133073020125343132211068382185").toString(16);
let in0_c1x = BigInteger::new("1345914801503869804221332717328097414792076925078931355300970385489312303055").toString(16);
let in0_c1y = BigInteger::new("3221919363851679888621419552929429977187872757564157365903242129276143826679").toString(16);
let in0_c2x = BigInteger::new("17378197425436069497126136266495011617394395570683447945973025044739809585373").toString(16);
let in0_c2y = BigInteger::new("15789009976977544046062803747743295235439704864191175329350822002296637150904").toString(16);
let out0_c1x = BigInteger::new("1580977511543777394910122699548784426094904736600505129541556064495159060532").toString(16);
        let out0_c1y = BigInteger::new("16190941039609473953318528369093289558337201974880158341123285226900681258492").toString(16);
        let out0_c2x = BigInteger::new("18928854895111284332170004407067674892341217562252934285209587817233013254394").toString(16);
        let out0_c2y = BigInteger::new("8499515539957690392433056598772536511996242730894002020454275332668597388028").toString(16);

        // argument order: in, out, priv
        let args = vec![String::default();]{"prove", pkx, pky, in0_c1x, in0_c1y, in0_c2x, in0_c2y, out0_c1x, out0_c1y, out0_c2x, out0_c2y};
        SampleMulCircuit.main(args);
    }

    
    pub   testSampleRehomCircuitCompile() {
        SampleRehomCircuit.main(vec![String::default();] {"compile"});
    }

    
    pub   testSampleRehomCircuitProve() {
        let in0_pk1_rec = BigInteger::new("6050894705972791558909254891616996674986810630226664089378712055547845936530").toString(16);
        let in0_pk2_rec = BigInteger::new("2098222428782134601768433132230348000710479961151701666623193700891051426734").toString(16);
        let in0_pk1_me = BigInteger::new("20482471737063427267743867277017992027916335915694495884029812150095009538665").toString(16);
        let in0_pk2_me = BigInteger::new("2064971568117740979897173202206849920364978834843195473834248767530206631140").toString(16);
        let in0_x1x = BigInteger::new("17651521056092576389327114851285097543355911207617425985108680402551109362320").toString(16);
        let in0_x1y = BigInteger::new("1979474016615069244406723045280100444351162006271416197677256792620315774762").toString(16);
        let in0_x2x = BigInteger::new("19853863594280103371645806654413426499353488214362048066055209302878076834605").toString(16);
        let in0_x2y = BigInteger::new("14030609827679157150694241338787994123586205285618036032395802092370201236392").toString(16);
        let in0_b1x = BigInteger::new("15288564466537518480361253319568821250986952964904376689834331007000477208583").toString(16);
        let in0_b1y = BigInteger::new("14499221764962898619288300583092505754061076652753865183142104711747753504290").toString(16);
        let in0_b2x = BigInteger::new("4453501154940040952535081235198610167580069962138595581877381781635748887580").toString(16);
        let in0_b2y = BigInteger::new("6003983121454395746745198309971673464065712423681748130677988590915163580200").toString(16);

        let out0_c1x = BigInteger::new("8753696203975329235585687250773552177062674794039810044171026034706968300723").toString(16);
        let out0_c1y = BigInteger::new("5610890170739048469029374457198992252393410759720420099039361376732580293843").toString(16);
        let out0_c2x = BigInteger::new("18367464182747295068302576897950293674363724573836344665957281230085766006943").toString(16);
        let out0_c2y = BigInteger::new("4537651393410589954105259703261476431845750308676046648775851995954139507000").toString(16);

        let priv_x1 = BigInteger::new("3").toString(16);
        let priv_x1r = BigInteger::new("94118587853295396393805395573620789675238716804156778422292198580847720966").toString(16);
        let priv_rnd = BigInteger::new("860776069412234168463308969065079006977381065448035510613391421262006397808").toString(16);
        let priv_sk = BigInteger::new("1690147589520195547977850797042220126332149894543617606884548383599391228484").toString(16);

        // argument order: in, out, priv
        let args = vec![String::default();]{"prove", in0_pk1_rec, in0_pk2_rec, in0_pk1_me, in0_pk2_me, in0_x1x, in0_x1y, in0_x2x, in0_x2y, in0_b1x, in0_b1y, in0_b2x, in0_b2y, in0_x1x, in0_x1y, in0_x2x, in0_x2y, out0_c1x, out0_c1y, out0_c2x, out0_c2y, priv_x1, priv_x1r, priv_rnd, priv_x1, priv_sk};
        SampleRehomCircuit.main(args);
    }
}
