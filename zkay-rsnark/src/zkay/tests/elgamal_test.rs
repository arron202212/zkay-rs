#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        eval::circuit_evaluator::CircuitEvaluator,
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
    zkay::{
        typed_wire::TypedWire, zkay_baby_jub_jub_gadget::JubJubPoint,
        zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget,
        zkay_elgamal_dec_gadget::ZkayElgamalDecGadget,
        zkay_elgamal_enc_gadget::ZkayElgamalEncGadget,
        zkay_elgamal_rerand_gadget::ZkayElgamalRerandGadget,
    },
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;
#[cfg(test)]
mod test {
    #[inline]
    fn pbi(bs: &str) -> BigInteger {
        Util::parse_big_int(bs)
    }
    use super::*;
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct AffinePoint {
        pub x: BigInteger,
        pub y: BigInteger,
    }
    impl AffinePoint {
        pub fn new(x: BigInteger, y: BigInteger) -> Self {
            Self { x, y }
        }

        pub fn asConstJubJub(&self, generator: &RcCell<CircuitGenerator>) -> JubJubPoint {
            let wx = CircuitGenerator::create_constant_wire(generator.clone(), &self.x, &None);
            let wy = CircuitGenerator::create_constant_wire(generator.clone(), &self.y, &None);
            JubJubPoint::new(wx, wy)
        }
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct ElgamalEncCircuitGenerator {
        plain: BigInteger,
        random: BigInteger,
        pk: AffinePoint,
    }
    impl ElgamalEncCircuitGenerator {
        pub fn new(
            name: &str,
            plain: BigInteger,
            random: BigInteger,
            pk: AffinePoint,
        ) -> CircuitGeneratorExtend<Self> {
            CircuitGeneratorExtend::<Self>::new(name, Self { plain, random, pk })
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<ElgamalEncCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<ElgamalEncCircuitGenerator> {
        fn build_circuit(&mut self) {
            let randomness =
                CircuitGenerator::create_constant_wire(self.cg(), &self.t.random, &None);
            let randomnessBits = randomness.get_bit_wiresi(self.t.random.bits(), &None);
            let message = CircuitGenerator::create_constant_wire(self.cg(), &self.t.plain, &None);
            let messageBits = message.get_bit_wiresi(32, &None);
            let generator = self.cg.clone();
            let gadget = ZkayElgamalEncGadget::new(
                messageBits.as_array().clone(),
                self.t.pk.asConstJubJub(&generator),
                randomnessBits.as_array().clone(),
                self.cg(),
            );
            CircuitGenerator::make_output_array(
                self.cg(),
                gadget.get_output_wires(),
                &Some("cipher".to_owned()),
            );
        }

        fn generate_sample_input(&self, _evaluator: &mut CircuitEvaluator) {}
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct ElgamalRerandCircuitGenerator {
        c1: AffinePoint,
        c2: AffinePoint,
        random: BigInteger,
        pk: AffinePoint,
    }

    impl ElgamalRerandCircuitGenerator {
        pub fn new(
            name: &str,
            c1: AffinePoint,
            c2: AffinePoint,
            pk: AffinePoint,
            random: BigInteger,
        ) -> CircuitGeneratorExtend<Self> {
            CircuitGeneratorExtend::<Self>::new(name, Self { c1, c2, random, pk })
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<ElgamalRerandCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<ElgamalRerandCircuitGenerator> {
        fn build_circuit(&mut self) {
            let randomness =
                CircuitGenerator::create_constant_wire(self.cg(), &self.t.random, &None);
            let randomnessBits = randomness.get_bit_wiresi(self.t.random.bits(), &None);
            let generator = self.cg.clone();
            let gadget = ZkayElgamalRerandGadget::new(
                self.t.c1.asConstJubJub(&generator),
                self.t.c2.asConstJubJub(&generator),
                self.t.pk.asConstJubJub(&generator),
                randomnessBits.as_array().clone(),
                self.cg(),
            );
            CircuitGenerator::make_output_array(
                self.cg(),
                gadget.get_output_wires(),
                &Some("rerand_cipher".to_owned()),
            );
        }

        fn generate_sample_input(&self, _evaluator: &mut CircuitEvaluator) {}
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct ElgamalDecCircuitGenerator {
        msg: BigInteger,
        pk: AffinePoint,
        sk: BigInteger,
        c1: AffinePoint,
        c2: AffinePoint,
    }

    impl ElgamalDecCircuitGenerator {
        pub fn new(
            name: &str,
            pk: AffinePoint,
            sk: BigInteger,
            c1: AffinePoint,
            c2: AffinePoint,
            msg: BigInteger,
        ) -> CircuitGeneratorExtend<Self> {
            CircuitGeneratorExtend::<Self>::new(
                name,
                Self {
                    msg,
                    pk,
                    sk,
                    c1,
                    c2,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<ElgamalDecCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<ElgamalDecCircuitGenerator> {
        fn build_circuit(&mut self) {
            let secretKey = CircuitGenerator::create_constant_wire(self.cg(), &self.t.sk, &None);
            let skBits = secretKey.get_bit_wiresi(self.t.sk.bits(), &None);
            let msgWire = CircuitGenerator::create_constant_wire(self.cg(), &self.t.msg, &None);
            let generator = self.cg.clone();
            let gadget = ZkayElgamalDecGadget::new(
                self.t.pk.asConstJubJub(&generator),
                skBits.as_array().clone(),
                self.t.c1.asConstJubJub(&generator),
                self.t.c2.asConstJubJub(&generator),
                msgWire,
                self.cg(),
            );
            CircuitGenerator::make_output_array(
                self.cg(),
                gadget.get_output_wires(),
                &Some("dummy output".to_owned()),
            );
        }

        fn generate_sample_input(&self, _evaluator: &mut CircuitEvaluator) {}
    }

    fn oneInputTest(
        plain: BigInteger,
        random: BigInteger,
        random2: BigInteger,
        sk: BigInteger,
        pk: AffinePoint,
        c1Expected: AffinePoint,
        c2Expected: AffinePoint,
        r1Expected: AffinePoint,
        r2Expected: AffinePoint,
    ) {
        let mut cgen =
            ElgamalEncCircuitGenerator::new("test_enc", plain.clone(), random, pk.clone());
        cgen.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("test_enc", &cgen.cg);
        evaluator.evaluate(&cgen.cg);
        let c1x = evaluator.get_wire_value(cgen.get_out_wires()[0].as_ref().unwrap());
        let c1y = evaluator.get_wire_value(cgen.get_out_wires()[1].as_ref().unwrap());
        let c2x = evaluator.get_wire_value(cgen.get_out_wires()[2].as_ref().unwrap());
        let c2y = evaluator.get_wire_value(cgen.get_out_wires()[3].as_ref().unwrap());
        assert_eq!(c1Expected.x, c1x);
        assert_eq!(c1Expected.y, c1y);
        assert_eq!(c2Expected.x, c2x);
        assert_eq!(c2Expected.y, c2y);

        let mut cgen = ElgamalDecCircuitGenerator::new(
            "test_dec",
            pk.clone(),
            sk.clone(),
            c1Expected.clone(),
            c2Expected.clone(),
            plain,
        );
        cgen.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("test_dec", &cgen.cg);
        evaluator.evaluate(&cgen.cg);
        let one = evaluator.get_wire_value(cgen.get_out_wires()[0].as_ref().unwrap());
        assert_eq!(Util::one(), one);

        let mut rgen =
            ElgamalRerandCircuitGenerator::new("test_rerand", c1Expected, c2Expected, pk, random2);
        rgen.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("test_rerand", &rgen.cg);
        evaluator.evaluate(&rgen.cg);
        let r1x = evaluator.get_wire_value(rgen.get_out_wires()[0].as_ref().unwrap());
        let r1y = evaluator.get_wire_value(rgen.get_out_wires()[1].as_ref().unwrap());
        let r2x = evaluator.get_wire_value(rgen.get_out_wires()[2].as_ref().unwrap());
        let r2y = evaluator.get_wire_value(rgen.get_out_wires()[3].as_ref().unwrap());
        assert_eq!(r1Expected.x, r1x);
        assert_eq!(r1Expected.y, r1y);
        assert_eq!(r2Expected.x, r2x);
        assert_eq!(r2Expected.y, r2y);
    }

    // * SAGE SCRIPT TO GENERATE TEST CASES

    //        p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
    //        Fp = GF(p)

    //        MONT_A = 168698
    //        MONT_B = 168700

    //        WEIERSTRASS_A2 = Fp(MONT_A) / Fp(MONT_B)
    //        WEIERSTRASS_A4 = Fp(1) / (Fp(MONT_B) * Fp(MONT_B))

    //        E = EllipticCurve(Fp, [0, WEIERSTRASS_A2, 0, WEIERSTRASS_A4, 0])

    //        as_edwards = lambda x, y: ((Fp(MONT_B)*x)/(Fp(MONT_B)*y), ((Fp(MONT_B)*x)-1)/((Fp(MONT_B)*x)+1))
    //        as_weierstrass = lambda x, y: ((1+y)/((1-y) * Fp(MONT_B)), (1+y)/((1-y)*x*Fp(MONT_B)))

    //        # Generator in Edwards form
    //        Gx = Fp(11904062828411472290643689191857696496057424932476499415469791423656658550213)
    //        Gy = Fp(9356450144216313082194365820021861619676443907964402770398322487858544118183)

    //        # Generator in Weierstrass form
    //        (Gu, Gv) = as_weierstrass(Gx, Gy)
    //        G = E(Gu, Gv)

    //        def ElGamalPk(rand):
    //            return G*rand

    //        def ElGamalEmbed(msg):
    //            return G*msg

    //        def ElGamalEnc(pk, msg, rand):
    //            s = pk*rand
    //            c1 = G*rand
    //            c2 = msg + s
    //            return (c1, c2)

    //        def ElGamalRerand(c1, c2, pk, rand):
    //            z = ElGamalEmbed(0)
    //            (z1, z2) = ElGamalEnc(pk, z, rand)
    //            return (z1 + c1, z2 + c2)

    //        def Run(sk, msg, rand, rand2):
    //                pk = ElGamalPk(sk)
    //                emb = ElGamalEmbed(msg)
    //                (c1, c2) = ElGamalEnc(pk, emb, rand)
    //                (d1, d2) = ElGamalRerand(c1, c2, pk, rand2)

    //                (pkx, pky) = as_edwards(pk[0], pk[1])
    //                (c1x, c1y) = as_edwards(c1[0], c1[1])
    //                (c2x, c2y) = as_edwards(c2[0], c2[1])
    //                (d1x, d1y) = as_edwards(d1[0], d1[1])
    //                (d2x, d2y) = as_edwards(d2[0], d2[1])
    //                print('BigInteger plain = BigInteger.parse_bytes(b"%s",10).unwrap();' % msg)
    //                print('BigInteger random = BigInteger.parse_bytes(b"%s",10).unwrap();' % rand)
    //                print('BigInteger random2 = BigInteger.parse_bytes(b"%s",10).unwrap();' % rand2)
    //                print('BigInteger sk = BigInteger.parse_bytes(b"%s",10).unwrap();' % sk)
    //                print('BigInteger pkx = BigInteger.parse_bytes(b"%s",10).unwrap();' % pkx)
    //                print('BigInteger pky = BigInteger.parse_bytes(b"%s",10).unwrap();' % pky)
    //                print('BigInteger c1x_exp = BigInteger.parse_bytes(b"%s",10).unwrap();' % c1x)
    //                print('BigInteger c1y_exp = BigInteger.parse_bytes(b"%s",10).unwrap();' % c1y)
    //                print('BigInteger c2x_exp = BigInteger.parse_bytes(b"%s",10).unwrap();' % c2x)
    //                print('BigInteger c2y_exp = BigInteger.parse_bytes(b"%s",10).unwrap();' % c2y)
    //                print('BigInteger r1x_exp = BigInteger.parse_bytes(b"%s",10).unwrap();' % d1x)
    //                print('BigInteger r1y_exp = BigInteger.parse_bytes(b"%s",10).unwrap();' % d1y)
    //                print('BigInteger r2x_exp = BigInteger.parse_bytes(b"%s",10).unwrap();' % d2x)
    //                print('BigInteger r2y_exp = BigInteger.parse_bytes(b"%s",10).unwrap();' % d2y)
    //                print('')

    //        Run(193884008695, 42, 405309899802, 498372940021)
    //        Run(399850902903, 439864, 450983970634, 1293840028489)
    //        Run(303897902911, 29479828, 11053400909823, 2818211)
    //        Run(879404942393, 20503, 40394702098873424340, 1199860398278648324)
    //        Run(409693890709893623, 9973, 400939876470980734, 980387209578)
    //        Run(943434980730874900974038, 3092, 304047020868704, 29059219019893)
    //        Run(40909374909834, 11, 9438929848, 472788712)
    //        Run(1047249, 309904, 2249, 187498091987891)
    //        Run(448344687855328518203304384067387474955750326758815542295083498526674852893, 42, 4992017890738015216991440853823451346783754228142718316135811893930821210517, 39278167679809198687982907130870918672986098198762678158021231)

    #[test]
    pub fn testElgamal1() {
        let plain = BigInteger::from(42);
        let random = BigInteger::from(405309899802i64);
        let random2 = BigInteger::from(498372940021i64);
        let sk = BigInteger::from(193884008695i64);
        let pkx =
            pbi("16805734088130288896486560435301001274867494983860633470885993193318772284256");
        let pky =
            pbi("12162439373882959082081494184542429855888325538638041876957263568830191647503");
        let c1x_exp =
            pbi("11968954241083294479582021735246320153591640350554672643229194688283746268751");
        let c1y_exp =
            pbi("17725843468231767283529061723550512784133895105007547043315490343601022890819");
        let c2x_exp =
            pbi("14203017384855711456240284283576262759333751248327439118405672500504849522290");
        let c2y_exp =
            pbi("20209776676192040223587478743432669760403295009110800013515437438556993692901");
        let r1x_exp =
            pbi("13591348693066294607093547701467815955182961658265372222056978378224264955118");
        let r1y_exp =
            pbi("224496693684666279083264478158697965533005482392940254861497379468968617265");
        let r2x_exp =
            pbi("10099626854765102435685973265870013378646709910580992014866316035367552182675");
        let r2y_exp =
            pbi("14767943092180306325317567029873935159218010704312689008185444061546749553058");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
    #[test]
    pub fn testElgamal2() {
        let plain = BigInteger::from(439864);
        let random = BigInteger::from(450983970634i64);
        let random2 = BigInteger::from(1293840028489i64);
        let sk = BigInteger::from(399850902903i64);
        let pkx =
            pbi("10779867656770035784341593210643876194947544727395589637798068397910380874725");
        let pky =
            pbi("10710250165934448718080245412425852632776460303399969324127728070645358476210");
        let c1x_exp =
            pbi("21217098875190065545745711937037122650118596372225419155354220102137118082248");
        let c1y_exp =
            pbi("8596071183490377685362568529945549465632153223890855646524023565071032562107");
        let c2x_exp =
            pbi("12243154004977744181331269362343083310985310016493155403556248989647435379337");
        let c2y_exp =
            pbi("5519301039601602428047143906992557429812524647117609489079159221144713724256");
        let r1x_exp =
            pbi("12879341210277729652562065130333613991137793795439148105389860506010063832764");
        let r1y_exp =
            pbi("12028008901381051327638773292171283584285939209840487219206955741588933923683");
        let r2x_exp =
            pbi("21348363880076528954413108099703096613495992044195524899352374409593437815681");
        let r2y_exp =
            pbi("448225545923890529546465107524885423214165045321928302012946805889055497548");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
    #[test]
    pub fn testElgamal3() {
        let plain = BigInteger::from(29479828);
        let random = BigInteger::from(11053400909823i64);
        let random2 = BigInteger::from(2818211);
        let sk = BigInteger::from(303897902911i64);
        let pkx =
            pbi("6414992512248574902260727978938771599371076631007732970498629309935423025541");
        let pky =
            pbi("5588797317393153831727440400622613249402810496821055368006297877884731592188");
        let c1x_exp =
            pbi("8457880476600111688234391562428843907438067884739990468648711671328170249897");
        let c1y_exp =
            pbi("5513193275811000218852876613945594356630692965732869074432709923308086384141");
        let c2x_exp =
            pbi("18871471165123797022765192830051533784387329326555711754062027748705980592258");
        let c2y_exp =
            pbi("2960859843097508915587155523192075278657656986058747365068999681758189942574");
        let r1x_exp =
            pbi("5366029516069172231732392874784911967837619433091056142690918344258949461784");
        let r1y_exp =
            pbi("15522512818540701465276714745444492880212853838854710379826233848880406457659");
        let r2x_exp =
            pbi("20641775747486861600659362415793944030784330174792501848914914142661365683768");
        let r2y_exp =
            pbi("4050578578711337375872799728115034683479047059868613096904707326437389065410");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
    #[test]
    pub fn testElgamal4() {
        let plain = BigInteger::from(20503);
        let random = BigInteger::from(40394702098873424340i128);
        let random2 = BigInteger::from(1199860398278648324i128);
        let sk = BigInteger::from(879404942393i64);
        let pkx =
            pbi("12387118419063114351013801589244952825991461324644293362309293502203205557028");
        let pky =
            pbi("12115395333617340639899571997042008699641933696177211723946595143553517655022");
        let c1x_exp =
            pbi("8470974253563601832011440733676763727170463193150013886940174894973160268113");
        let c1y_exp =
            pbi("11451437979815532596520424453163860534423134767934210095904011136004726209298");
        let c2x_exp =
            pbi("3755451285204548243386923793338922452126300087029724835994171785286681386647");
        let c2y_exp =
            pbi("5647640334301816276800781755737747998337525435601524546545647915251655431126");
        let r1x_exp =
            pbi("19189418561911229092629541870381728693454153202408672369439535900592352563832");
        let r1y_exp =
            pbi("6190221119147372470564485675966744098041498927494365664238329939235766355806");
        let r2x_exp =
            pbi("20606498248708222575429594795830500486712281647481596625185438753439188883374");
        let r2y_exp =
            pbi("16572497279801942880250856433861727900257767071582946132942024691743685883868");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
    #[test]
    pub fn testElgamal5() {
        let plain = BigInteger::from(9973);
        let random = BigInteger::from(400939876470980734i64);
        let random2 = BigInteger::from(980387209578i64);
        let sk = BigInteger::from(409693890709893623i64);
        let pkx =
            pbi("19038786034365121129737447326845215547071528710647939313908355725905191188995");
        let pky =
            pbi("2214248829964940682725033718946556328772607342640796638058055582396213081489");
        let c1x_exp =
            pbi("4049645432003817379994226545412987321416789229476686170128957164758871401279");
        let c1y_exp =
            pbi("16222213389691959124184899327364928149053913263183689276193684274178358008847");
        let c2x_exp =
            pbi("20622976335254791707752271712848997733998271931456734369112350069849260350570");
        let c2y_exp =
            pbi("18512314847286550940159097003907528453978422823733935044908448485364066867711");
        let r1x_exp =
            pbi("18012047336332553077224720034396440234969675366649853620277921699711776290087");
        let r1y_exp =
            pbi("4548928749379657739820459290800365447344909635123275019182600575083243815395");
        let r2x_exp =
            pbi("1463706360529125936803497998062819315280830785086437985047610791857684501217");
        let r2y_exp =
            pbi("19260145642157386527783785376105740564939772326492185963463823034939637900510");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
    #[test]
    pub fn testElgamal6() {
        let plain = BigInteger::from(3092);
        let random = BigInteger::from(304047020868704i64);
        let random2 = BigInteger::from(29059219019893i64);
        let sk = pbi("943434980730874900974038");
        let pkx =
            pbi("11537936820602925819401558832551213707370271036894418664399992536929137441385");
        let pky =
            pbi("21341107817615984362450388042180099428636742794610654263474204384582578901535");
        let c1x_exp =
            pbi("5759977009078653474075225079238017700911800551924115686420736271126581950794");
        let c1y_exp =
            pbi("19803546030374265878743382701240403271716532910167764659132971083286486432920");
        let c2x_exp =
            pbi("13163571290961645931573447250398485715074921372484044328064084837570242392677");
        let c2y_exp =
            pbi("2561391748738501878805425385302883053224206298569352883147194368919207812616");
        let r1x_exp =
            pbi("3395078398739021110672219647886266968195610965478540927835578721265826113829");
        let r1y_exp =
            pbi("3226633820835478993953835109420755648719417700859759750351066585603811804967");
        let r2x_exp =
            pbi("9019585129196677535142255820209900579037484179808220251923228715078249551317");
        let r2y_exp =
            pbi("1217462091073599572419023941043993348899274045225302280909460520543019198569");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
    #[test]
    pub fn testElgamal7() {
        let plain = BigInteger::from(11);
        let random = BigInteger::from(9438929848i64);
        let random2 = BigInteger::from(472788712);
        let sk = BigInteger::from(40909374909834i64);
        let pkx =
            pbi("18963601429601260488925336533212077133253656490980222624829298073185383062394");
        let pky =
            pbi("10955396660032392970784549789530638666297323493863859953055999819584497853280");
        let c1x_exp =
            pbi("1585437441439177712931180855793556731169186271301451803103671783184926099707");
        let c1y_exp =
            pbi("17238669393035514721193643357894128432464531731096710478456257855369920548914");
        let c2x_exp =
            pbi("1905207801382404175680710222856135239447406509352907340030501059581465963296");
        let c2y_exp =
            pbi("20283410046728803419736841039385114962006738871621806761375631312392012049538");
        let r1x_exp =
            pbi("1172658417426784028905024343050664961751271991789496194901660929685910153502");
        let r1y_exp =
            pbi("17841298193398572201135170415806132380874126713992220099435734043652191436231");
        let r2x_exp =
            pbi("7665114463165580774659286388392015419508027373404639766757305403638822493955");
        let r2y_exp =
            pbi("20283399076363492534661102577978890614478101704954633485201009460012598302984");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
    #[test]
    pub fn testElgamal8() {
        let plain = BigInteger::from(309904);
        let random = BigInteger::from(2249);
        let random2 = BigInteger::from(187498091987891i64);
        let sk = BigInteger::from(1047249);
        let pkx =
            pbi("18796243199533119758484912853892319178237479744292136482258313307214080406845");
        let pky =
            pbi("12562816211385016374219058391715927349499041836379377424804413924517388503535");
        let c1x_exp =
            pbi("1093180272049918847371658916991447949076205903414878489417833675168297761329");
        let c1y_exp =
            pbi("13652001713064310312737185590474813760724236299822572903882767064490757672145");
        let c2x_exp =
            pbi("10233072806856007905263356274253594443764592402456777832406280451546479173285");
        let c2y_exp =
            pbi("15828131619625847918230665900694350637473057051841970861137734958423235339878");
        let r1x_exp =
            pbi("6913306664985054426548553351911704413655199598352597631955117153023351855134");
        let r1y_exp =
            pbi("11053589110183477810314966941682935316922509679588844947720220513312942073284");
        let r2x_exp =
            pbi("11926108613321274783962330168347934941286508322677303443866831105482343220833");
        let r2y_exp =
            pbi("20884432215848566718856711647507988451300507966194327106115486784790475250127");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
    #[test]
    pub fn testElgamal9() {
        let plain = BigInteger::from(42);
        let random =
            pbi("4992017890738015216991440853823451346783754228142718316135811893930821210517");
        let random2 = pbi("39278167679809198687982907130870918672986098198762678158021231");
        let sk = pbi("448344687855328518203304384067387474955750326758815542295083498526674852893");
        let pkx =
            pbi("2543111965495064707612623550577403881714453669184859408922451773306175031318");
        let pky =
            pbi("20927827475527585117296730644692999944545060105133073020125343132211068382185");
        let c1x_exp =
            pbi("17990166387038654353532224054392704246273066434684370089496246721960255371329");
        let c1y_exp =
            pbi("15866190370882469414665095798958204707796441173247149326160843221134574846694");
        let c2x_exp =
            pbi("13578016172019942326633412365679613147103709674318008979748420035774874659858");
        let c2y_exp =
            pbi("15995926508900361671313404296634773295236345482179714831868518062689263430374");
        let r1x_exp =
            pbi("18784552725438955691676194299236851361347814005105892890816896300567057713848");
        let r1y_exp =
            pbi("19693165835882985893423572117981608192865028744064689668605666703143190897862");
        let r2x_exp =
            pbi("2530344813076577056814169669700763620340945156800181207832024608219434629412");
        let r2y_exp =
            pbi("10093888871955407903732269877335284565715256278559408224374937460596986224178");

        oneInputTest(
            plain,
            random,
            random2,
            sk,
            AffinePoint::new(pkx, pky),
            AffinePoint::new(c1x_exp, c1y_exp),
            AffinePoint::new(c2x_exp, c2y_exp),
            AffinePoint::new(r1x_exp, r1y_exp),
            AffinePoint::new(r2x_exp, r2y_exp),
        );
    }
}
