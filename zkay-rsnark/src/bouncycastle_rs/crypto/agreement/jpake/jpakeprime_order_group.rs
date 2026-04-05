// package org.bouncycastle.crypto.agreement.jpake;

// import java.math.BigInteger;

// /**
//  * A pre-computed prime order group for use during a J-PAKE exchange.
//  * <p>
//  * Typically a Schnorr group is used.  In general, J-PAKE can use any prime order group
//  * that is suitable for public key cryptography, including elliptic curve cryptography.
//  * <p>
//  * See {@link JPAKEPrimeOrderGroups} for convenient standard groups.
//  * <p>
//  * NIST <a href="http://csrc.nist.gov/groups/ST/toolkit/documents/Examples/DSA2_All.pdf">publishes</a>
//  * many groups that can be used for the desired level of security.
//  */
// public class JPAKEPrimeOrderGroup
// {
//     private final BigInteger p;
//     private final BigInteger q;
//     private final BigInteger g;

//     /**
//      * Constructs a new {@link JPAKEPrimeOrderGroup}.
//      * <p>
//      * In general, you should use one of the pre-approved groups from
//      * {@link JPAKEPrimeOrderGroups}, rather than manually constructing one.
//      * <p>
//      * The following basic checks are performed:
//      * <ul>
//      * <li>p-1 must be evenly divisible by q</li>
//      * <li>g must be in [2, p-1]</li>
//      * <li>g^q mod p must equal 1</li>
//      * <li>p must be prime (within reasonably certainty)</li>
//      * <li>q must be prime (within reasonably certainty)</li>
//      * </ul>
//      * <p>
//      * The prime checks are performed using {@link BigInteger#isProbablePrime(int)},
//      * and are therefore subject to the same probability guarantees.
//      * <p>
//      * These checks prevent trivial mistakes.
//      * However, due to the small uncertainties if p and q are not prime,
//      * advanced attacks are not prevented.
//      * Use it at your own risk.
//      *
//      * @throws NullPointerException if any argument is null
//      * @throws IllegalArgumentException if any of the above validations fail
//      */
//     public JPAKEPrimeOrderGroup(BigInteger p, BigInteger q, BigInteger g)
//     {
//         /*
//          * Don't skip the checks on user-specified groups.
//          */
//         this(p, q, g, false);
//     }

//     /**
//      * Internal package-private constructor used by the pre-approved
//      * groups in {@link JPAKEPrimeOrderGroups}.
//      * These pre-approved groups can avoid the expensive checks.
//      */
//     JPAKEPrimeOrderGroup(BigInteger p, BigInteger q, BigInteger g, boolean skipChecks)
//     {
//         JPAKEUtil.validateNotNull(p, "p");
//         JPAKEUtil.validateNotNull(q, "q");
//         JPAKEUtil.validateNotNull(g, "g");

//         if (!skipChecks)
//         {
//             if (!p.subtract(JPAKEUtil.ONE).mod(q).equals(JPAKEUtil.ZERO))
//             {
//                 throw new IllegalArgumentException("p-1 must be evenly divisible by q");
//             }
//             if (g.compareTo(BigInteger.valueOf(2)) == -1 || g.compareTo(p.subtract(JPAKEUtil.ONE)) == 1)
//             {
//                 throw new IllegalArgumentException("g must be in [2, p-1]");
//             }
//             if (!g.modPow(q, p).equals(JPAKEUtil.ONE))
//             {
//                 throw new IllegalArgumentException("g^q mod p must equal 1");
//             }
//             /*
//              * Note that these checks do not guarantee that p and q are prime.
//              * We just have reasonable certainty that they are prime.
//              */
//             if (!p.isProbablePrime(20))
//             {
//                 throw new IllegalArgumentException("p must be prime");
//             }
//             if (!q.isProbablePrime(20))
//             {
//                 throw new IllegalArgumentException("q must be prime");
//             }
//         }

//         this.p = p;
//         this.q = q;
//         this.g = g;
//     }

//     public BigInteger getP()
//     {
//         return p;
//     }

//     public BigInteger getQ()
//     {
//         return q;
//     }

//     public BigInteger getG()
//     {
//         return g;
//     }

// }
use num_bigint::BigInt;
use num_traits::{One, Zero, ToPrimitive};
use num_primes::Verification; // 用于 Probable Prime 测试

#[derive(Clone, Debug)]
pub struct JpakePrimeOrderGroup {
    p: BigInt,
    q: BigInt,
    g: BigInt,
}

impl JpakePrimeOrderGroup {
    /// 构造一个新的 JPAKE 质数阶群 (包含完整校验)
    pub fn new(p: BigInt, q: BigInt, g: BigInt) -> Result<Self, String> {
        Self::new_internal(p, q, g, false)
    }

    /// 内部构造函数，对应 Java 的 package-private 构造器
    pub(crate) fn new_internal(p: BigInt, q: BigInt, g: BigInt, skip_checks: bool) -> Result<Self, String> {
        if !skip_checks {
            // 1. p-1 必须能被 q 整除
            let p_minus_1 = &p - BigInt::one();
            if !(&p_minus_1 % &q).is_zero() {
                return Err("p-1 must be evenly divisible by q".to_string());
            }

            // 2. g 必须在 [2, p-1] 范围内
            let two = BigInt::from(2);
            if g < two || g > p_minus_1 {
                return Err("g must be in [2, p-1]".to_string());
            }

            // 3. g^q mod p 必须等于 1
            if !g.modpow(&q, &p).is_one() {
                return Err("g^q mod p must equal 1".to_string());
            }

            // 4. p 必须是质数 (合理确定性)
            // num-primes 的 Verification::is_prime 使用了类似 isProbablePrime 的测试
            if !Verification::is_prime(&p) {
                return Err("p must be prime".to_string());
            }

            // 5. q 必须是质数 (合理确定性)
            if !Verification::is_prime(&q) {
                return Err("q must be prime".to_string());
            }
        }

        Ok(Self { p, q, g })
    }

    // Getter 方法
    pub fn p(&self) -> &BigInt { &self.p }
    pub fn q(&self) -> &BigInt { &self.q }
    pub fn g(&self) -> &BigInt { &self.g }
}
