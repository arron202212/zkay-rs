// package org.bouncycastle.crypto.agreement.jpake;

// import java.math.BigInteger;
// import java.security.SecureRandom;

// import org.bouncycastle.crypto.CryptoException;
// import org.bouncycastle.crypto.CryptoServicesRegistrar;
// import org.bouncycastle.crypto.Digest;
// import org.bouncycastle.crypto.digests.SHA256Digest;
// import org.bouncycastle.util.Arrays;

// /**
//  * A participant in a Password Authenticated Key Exchange by Juggling (J-PAKE) exchange.
//  * <p>
//  * The J-PAKE exchange is defined by Feng Hao and Peter Ryan in the paper
//  * <a href="http://grouper.ieee.org/groups/1363/Research/contributions/hao-ryan-2008.pdf">
//  * "Password Authenticated Key Exchange by Juggling, 2008."</a>
//  * <p>
//  * The J-PAKE protocol is symmetric.
//  * There is no notion of a <i>client</i> or <i>server</i>, but rather just two <i>participants</i>.
//  * An instance of {@link JPAKEParticipant} represents one participant, and
//  * is the primary interface for executing the exchange.
//  * <p>
//  * To execute an exchange, construct a {@link JPAKEParticipant} on each end,
//  * and call the following 7 methods
//  * (once and only once, in the given order, for each participant, sending messages between them as described):
//  * <ol>
//  * <li>{@link #createRound1PayloadToSend()} - and send the payload to the other participant</li>
//  * <li>{@link #validateRound1PayloadReceived(JPAKERound1Payload)} - use the payload received from the other participant</li>
//  * <li>{@link #createRound2PayloadToSend()} - and send the payload to the other participant</li>
//  * <li>{@link #validateRound2PayloadReceived(JPAKERound2Payload)} - use the payload received from the other participant</li>
//  * <li>{@link #calculateKeyingMaterial()}</li>
//  * <li>{@link #createRound3PayloadToSend(BigInteger)} - and send the payload to the other participant</li>
//  * <li>{@link #validateRound3PayloadReceived(JPAKERound3Payload, BigInteger)} - use the payload received from the other participant</li>
//  * </ol>
//  * <p>
//  * Each side should derive a session key from the keying material returned by {@link #calculateKeyingMaterial()}.
//  * The caller is responsible for deriving the session key using a secure key derivation function (KDF).
//  * <p>
//  * Round 3 is an optional key confirmation process.
//  * If you do not execute round 3, then there is no assurance that both participants are using the same key.
//  * (i.e. if the participants used different passwords, then their session keys will differ.)
//  * <p>
//  * If the round 3 validation succeeds, then the keys are guaranteed to be the same on both sides.
//  * <p>
//  * The symmetric design can easily support the asymmetric cases when one party initiates the communication.
//  * e.g. Sometimes the round1 payload and round2 payload may be sent in one pass.
//  * Also, in some cases, the key confirmation payload can be sent together with the round2 payload.
//  * These are the trivial techniques to optimize the communication.
//  * <p>
//  * The key confirmation process is implemented as specified in
//  * <a href="http://csrc.nist.gov/publications/nistpubs/800-56A/SP800-56A_Revision1_Mar08-2007.pdf">NIST SP 800-56A Revision 1</a>,
//  * Section 8.2 Unilateral Key Confirmation for Key Agreement Schemes.
//  * <p>
//  * This class is stateful and NOT threadsafe.
//  * Each instance should only be used for ONE complete J-PAKE exchange
//  * (i.e. a new {@link JPAKEParticipant} should be constructed for each new J-PAKE exchange).
//  * <p>
//  */
// public class JPAKEParticipant
// {
//     /*
//      * Possible internal states.  Used for state checking.
//      */

//     public static final int STATE_INITIALIZED = 0;
//     public static final int STATE_ROUND_1_CREATED = 10;
//     public static final int STATE_ROUND_1_VALIDATED = 20;
//     public static final int STATE_ROUND_2_CREATED = 30;
//     public static final int STATE_ROUND_2_VALIDATED = 40;
//     public static final int STATE_KEY_CALCULATED = 50;
//     public static final int STATE_ROUND_3_CREATED = 60;
//     public static final int STATE_ROUND_3_VALIDATED = 70;

//     /**
//      * Unique identifier of this participant.
//      * The two participants in the exchange must NOT share the same id.
//      */
//     private final String participantId;

//     /**
//      * Shared secret.  This only contains the secret between construction
//      * and the call to {@link #calculateKeyingMaterial()}.
//      * <p>
//      * i.e. When {@link #calculateKeyingMaterial()} is called, this buffer overwritten with 0's,
//      * and the field is set to null.
//      * </p>
//      */
//     private char[] password;

//     /**
//      * Digest to use during calculations.
//      */
//     private final Digest digest;

//     /**
//      * Source of secure random data.
//      */
//     private final SecureRandom random;

//     private final BigInteger p;
//     private final BigInteger q;
//     private final BigInteger g;

//     /**
//      * The participantId of the other participant in this exchange.
//      */
//     private String partnerParticipantId;

//     /**
//      * Alice's x1 or Bob's x3.
//      */
//     private BigInteger x1;
//     /**
//      * Alice's x2 or Bob's x4.
//      */
//     private BigInteger x2;
//     /**
//      * Alice's g^x1 or Bob's g^x3.
//      */
//     private BigInteger gx1;
//     /**
//      * Alice's g^x2 or Bob's g^x4.
//      */
//     private BigInteger gx2;
//     /**
//      * Alice's g^x3 or Bob's g^x1.
//      */
//     private BigInteger gx3;
//     /**
//      * Alice's g^x4 or Bob's g^x2.
//      */
//     private BigInteger gx4;
//     /**
//      * Alice's B or Bob's A.
//      */
//     private BigInteger b;

//     /**
//      * The current state.
//      * See the <tt>STATE_*</tt> constants for possible values.
//      */
//     private int state;

//     /**
//      * Convenience constructor for a new {@link JPAKEParticipant} that uses
//      * the {@link JPAKEPrimeOrderGroups#NIST_3072} prime order group,
//      * a SHA-256 digest, and a default {@link SecureRandom} implementation.
//      * <p>
//      * After construction, the {@link #getState() state} will be  {@link #STATE_INITIALIZED}.
//      *
//      * @param participantId unique identifier of this participant.
//      *                      The two participants in the exchange must NOT share the same id.
//      * @param password      shared secret.
//      *                      A defensive copy of this array is made (and cleared once {@link #calculateKeyingMaterial()} is called).
//      *                      Caller should clear the input password as soon as possible.
//      * @throws NullPointerException if any argument is null
//      * @throws IllegalArgumentException if password is empty
//      */
//     public JPAKEParticipant(
//         String participantId,
//         char[] password)
//     {
//         this(
//             participantId,
//             password,
//             JPAKEPrimeOrderGroups.NIST_3072);
//     }


//     /**
//      * Convenience constructor for a new {@link JPAKEParticipant} that uses
//      * a SHA-256 digest and a default {@link SecureRandom} implementation.
//      * <p>
//      * After construction, the {@link #getState() state} will be  {@link #STATE_INITIALIZED}.
//      *
//      * @param participantId unique identifier of this participant.
//      *                      The two participants in the exchange must NOT share the same id.
//      * @param password      shared secret.
//      *                      A defensive copy of this array is made (and cleared once {@link #calculateKeyingMaterial()} is called).
//      *                      Caller should clear the input password as soon as possible.
//      * @param group         prime order group.
//      *                      See {@link JPAKEPrimeOrderGroups} for standard groups
//      * @throws NullPointerException if any argument is null
//      * @throws IllegalArgumentException if password is empty
//      */
//     public JPAKEParticipant(
//         String participantId,
//         char[] password,
//         JPAKEPrimeOrderGroup group)
//     {
//         this(
//             participantId,
//             password,
//             group,
//             new SHA256Digest(),
//             CryptoServicesRegistrar.getSecureRandom());
//     }


//     /**
//      * Construct a new {@link JPAKEParticipant}.
//      * <p>
//      * After construction, the {@link #getState() state} will be  {@link #STATE_INITIALIZED}.
//      *
//      * @param participantId unique identifier of this participant.
//      *                      The two participants in the exchange must NOT share the same id.
//      * @param password      shared secret.
//      *                      A defensive copy of this array is made (and cleared once {@link #calculateKeyingMaterial()} is called).
//      *                      Caller should clear the input password as soon as possible.
//      * @param group         prime order group.
//      *                      See {@link JPAKEPrimeOrderGroups} for standard groups
//      * @param digest        digest to use during zero knowledge proofs and key confirmation (SHA-256 or stronger preferred)
//      * @param random        source of secure random data for x1 and x2, and for the zero knowledge proofs
//      * @throws NullPointerException if any argument is null
//      * @throws IllegalArgumentException if password is empty
//      */
//     public JPAKEParticipant(
//         String participantId,
//         char[] password,
//         JPAKEPrimeOrderGroup group,
//         Digest digest,
//         SecureRandom random)
//     {
//         JPAKEUtil.validateNotNull(participantId, "participantId");
//         JPAKEUtil.validateNotNull(password, "password");
//         JPAKEUtil.validateNotNull(group, "p");
//         JPAKEUtil.validateNotNull(digest, "digest");
//         JPAKEUtil.validateNotNull(random, "random");
//         if (password.length == 0)
//         {
//             throw new IllegalArgumentException("Password must not be empty.");
//         }

//         this.participantId = participantId;
        
//         /*
//          * Create a defensive copy so as to fully encapsulate the password.
//          * 
//          * This array will contain the password for the lifetime of this
//          * participant BEFORE {@link #calculateKeyingMaterial()} is called.
//          * 
//          * i.e. When {@link #calculateKeyingMaterial()} is called, the array will be cleared
//          * in order to remove the password from memory.
//          * 
//          * The caller is responsible for clearing the original password array
//          * given as input to this constructor.
//          */
//         this.password = Arrays.copyOf(password, password.length);

//         this.p = group.getP();
//         this.q = group.getQ();
//         this.g = group.getG();

//         this.digest = digest;
//         this.random = random;

//         this.state = STATE_INITIALIZED;
//     }

//     /**
//      * Gets the current state of this participant.
//      * See the <tt>STATE_*</tt> constants for possible values.
//      */
//     public int getState()
//     {
//         return this.state;
//     }

//     /**
//      * Creates and returns the payload to send to the other participant during round 1.
//      * <p>
//      * After execution, the {@link #getState() state} will be  {@link #STATE_ROUND_1_CREATED}.
//      */
//     public JPAKERound1Payload createRound1PayloadToSend()
//     {
//         if (this.state >= STATE_ROUND_1_CREATED)
//         {
//             throw new IllegalStateException("Round1 payload already created for " + participantId);
//         }

//         this.x1 = JPAKEUtil.generateX1(q, random);
//         this.x2 = JPAKEUtil.generateX2(q, random);

//         this.gx1 = JPAKEUtil.calculateGx(p, g, x1);
//         this.gx2 = JPAKEUtil.calculateGx(p, g, x2);
//         BigInteger[] knowledgeProofForX1 = JPAKEUtil.calculateZeroKnowledgeProof(p, q, g, gx1, x1, participantId, digest, random);
//         BigInteger[] knowledgeProofForX2 = JPAKEUtil.calculateZeroKnowledgeProof(p, q, g, gx2, x2, participantId, digest, random);

//         this.state = STATE_ROUND_1_CREATED;

//         return new JPAKERound1Payload(participantId, gx1, gx2, knowledgeProofForX1, knowledgeProofForX2);
//     }

//     /**
//      * Validates the payload received from the other participant during round 1.
//      * <p>
//      * Must be called prior to {@link #createRound2PayloadToSend()}.
//      * <p>
//      * After execution, the {@link #getState() state} will be  {@link #STATE_ROUND_1_VALIDATED}.
//      *
//      * @throws CryptoException if validation fails.
//      * @throws IllegalStateException if called multiple times.
//      */
//     public void validateRound1PayloadReceived(JPAKERound1Payload round1PayloadReceived)
//         throws CryptoException
//     {
//         if (this.state >= STATE_ROUND_1_VALIDATED)
//         {
//             throw new IllegalStateException("Validation already attempted for round1 payload for" + participantId);
//         }
//         this.partnerParticipantId = round1PayloadReceived.getParticipantId();
//         this.gx3 = round1PayloadReceived.getGx1();
//         this.gx4 = round1PayloadReceived.getGx2();

//         BigInteger[] knowledgeProofForX3 = round1PayloadReceived.getKnowledgeProofForX1();
//         BigInteger[] knowledgeProofForX4 = round1PayloadReceived.getKnowledgeProofForX2();

//         JPAKEUtil.validateParticipantIdsDiffer(participantId, round1PayloadReceived.getParticipantId());
//         JPAKEUtil.validateGx4(gx4);
//         JPAKEUtil.validateZeroKnowledgeProof(p, q, g, gx3, knowledgeProofForX3, round1PayloadReceived.getParticipantId(), digest);
//         JPAKEUtil.validateZeroKnowledgeProof(p, q, g, gx4, knowledgeProofForX4, round1PayloadReceived.getParticipantId(), digest);

//         this.state = STATE_ROUND_1_VALIDATED;
//     }

//     /**
//      * Creates and returns the payload to send to the other participant during round 2.
//      * <p>
//      * {@link #validateRound1PayloadReceived(JPAKERound1Payload)} must be called prior to this method.
//      * <p>
//      * After execution, the {@link #getState() state} will be  {@link #STATE_ROUND_2_CREATED}.
//      *
//      * @throws IllegalStateException if called prior to {@link #validateRound1PayloadReceived(JPAKERound1Payload)}, or multiple times
//      */
//     public JPAKERound2Payload createRound2PayloadToSend()
//     {
//         if (this.state >= STATE_ROUND_2_CREATED)
//         {
//             throw new IllegalStateException("Round2 payload already created for " + this.participantId);
//         }
//         if (this.state < STATE_ROUND_1_VALIDATED)
//         {
//             throw new IllegalStateException("Round1 payload must be validated prior to creating Round2 payload for " + this.participantId);
//         }
//         BigInteger gA = JPAKEUtil.calculateGA(p, gx1, gx3, gx4);
//         BigInteger s = JPAKEUtil.calculateS(password);
//         BigInteger x2s = JPAKEUtil.calculateX2s(q, x2, s);
//         BigInteger A = JPAKEUtil.calculateA(p, q, gA, x2s);
//         BigInteger[] knowledgeProofForX2s = JPAKEUtil.calculateZeroKnowledgeProof(p, q, gA, A, x2s, participantId, digest, random);

//         this.state = STATE_ROUND_2_CREATED;

//         return new JPAKERound2Payload(participantId, A, knowledgeProofForX2s);
//     }

//     /**
//      * Validates the payload received from the other participant during round 2.
//      * <p>
//      * Note that this DOES NOT detect a non-common password.
//      * The only indication of a non-common password is through derivation
//      * of different keys (which can be detected explicitly by executing round 3 and round 4)
//      * <p>
//      * Must be called prior to {@link #calculateKeyingMaterial()}.
//      * <p>
//      * After execution, the {@link #getState() state} will be  {@link #STATE_ROUND_2_VALIDATED}.
//      *
//      * @throws CryptoException if validation fails.
//      * @throws IllegalStateException if called prior to {@link #validateRound1PayloadReceived(JPAKERound1Payload)}, or multiple times
//      */
//     public void validateRound2PayloadReceived(JPAKERound2Payload round2PayloadReceived)
//         throws CryptoException
//     {
//         if (this.state >= STATE_ROUND_2_VALIDATED)
//         {
//             throw new IllegalStateException("Validation already attempted for round2 payload for" + participantId);
//         }
//         if (this.state < STATE_ROUND_1_VALIDATED)
//         {
//             throw new IllegalStateException("Round1 payload must be validated prior to validating Round2 payload for " + this.participantId);
//         }
//         BigInteger gB = JPAKEUtil.calculateGA(p, gx3, gx1, gx2);
//         this.b = round2PayloadReceived.getA();
//         BigInteger[] knowledgeProofForX4s = round2PayloadReceived.getKnowledgeProofForX2s();

//         JPAKEUtil.validateParticipantIdsDiffer(participantId, round2PayloadReceived.getParticipantId());
//         JPAKEUtil.validateParticipantIdsEqual(this.partnerParticipantId, round2PayloadReceived.getParticipantId());
//         JPAKEUtil.validateGa(gB);
//         JPAKEUtil.validateZeroKnowledgeProof(p, q, gB, b, knowledgeProofForX4s, round2PayloadReceived.getParticipantId(), digest);

//         this.state = STATE_ROUND_2_VALIDATED;
//     }

//     /**
//      * Calculates and returns the key material.
//      * A session key must be derived from this key material using a secure key derivation function (KDF).
//      * The KDF used to derive the key is handled externally (i.e. not by {@link JPAKEParticipant}).
//      * <p>
//      * The keying material will be identical for each participant if and only if
//      * each participant's password is the same.  i.e. If the participants do not
//      * share the same password, then each participant will derive a different key.
//      * Therefore, if you immediately start using a key derived from
//      * the keying material, then you must handle detection of incorrect keys.
//      * If you want to handle this detection explicitly, you can optionally perform
//      * rounds 3 and 4.  See {@link JPAKEParticipant} for details on how to execute
//      * rounds 3 and 4.
//      * <p>
//      * The keying material will be in the range <tt>[0, p-1]</tt>.
//      * <p>
//      * {@link #validateRound2PayloadReceived(JPAKERound2Payload)} must be called prior to this method.
//      * <p>
//      * As a side effect, the internal {@link #password} array is cleared, since it is no longer needed.
//      * <p>
//      * After execution, the {@link #getState() state} will be  {@link #STATE_KEY_CALCULATED}.
//      *
//      * @throws IllegalStateException if called prior to {@link #validateRound2PayloadReceived(JPAKERound2Payload)},
//      * or if called multiple times.
//      */
//     public BigInteger calculateKeyingMaterial()
//     {
//         if (this.state >= STATE_KEY_CALCULATED)
//         {
//             throw new IllegalStateException("Key already calculated for " + participantId);
//         }
//         if (this.state < STATE_ROUND_2_VALIDATED)
//         {
//             throw new IllegalStateException("Round2 payload must be validated prior to creating key for " + participantId);
//         }
//         BigInteger s = JPAKEUtil.calculateS(password);
        
//         /*
//          * Clear the password array from memory, since we don't need it anymore.
//          * 
//          * Also set the field to null as a flag to indicate that the key has already been calculated.
//          */
//         Arrays.fill(password, (char)0);
//         this.password = null;

//         BigInteger keyingMaterial = JPAKEUtil.calculateKeyingMaterial(p, q, gx4, x2, s, b);
        
//         /*
//          * Clear the ephemeral private key fields as well.
//          * Note that we're relying on the garbage collector to do its job to clean these up.
//          * The old objects will hang around in memory until the garbage collector destroys them.
//          * 
//          * If the ephemeral private keys x1 and x2 are leaked,
//          * the attacker might be able to brute-force the password.
//          */
//         this.x1 = null;
//         this.x2 = null;
//         this.b = null;
        
//         /*
//          * Do not clear gx* yet, since those are needed by round 3.
//          */

//         this.state = STATE_KEY_CALCULATED;

//         return keyingMaterial;
//     }


//     /**
//      * Creates and returns the payload to send to the other participant during round 3.
//      * <p>
//      * See {@link JPAKEParticipant} for more details on round 3.
//      * <p>
//      * After execution, the {@link #getState() state} will be  {@link #STATE_ROUND_3_CREATED}.
//      *
//      * @param keyingMaterial The keying material as returned from {@link #calculateKeyingMaterial()}.
//      * @throws IllegalStateException if called prior to {@link #calculateKeyingMaterial()}, or multiple times
//      */
//     public JPAKERound3Payload createRound3PayloadToSend(BigInteger keyingMaterial)
//     {
//         if (this.state >= STATE_ROUND_3_CREATED)
//         {
//             throw new IllegalStateException("Round3 payload already created for " + this.participantId);
//         }
//         if (this.state < STATE_KEY_CALCULATED)
//         {
//             throw new IllegalStateException("Keying material must be calculated prior to creating Round3 payload for " + this.participantId);
//         }

//         BigInteger macTag = JPAKEUtil.calculateMacTag(
//             this.participantId,
//             this.partnerParticipantId,
//             this.gx1,
//             this.gx2,
//             this.gx3,
//             this.gx4,
//             keyingMaterial,
//             this.digest);

//         this.state = STATE_ROUND_3_CREATED;

//         return new JPAKERound3Payload(participantId, macTag);
//     }

//     /**
//      * Validates the payload received from the other participant during round 3.
//      * <p>
//      * See {@link JPAKEParticipant} for more details on round 3.
//      * <p>
//      * After execution, the {@link #getState() state} will be {@link #STATE_ROUND_3_VALIDATED}.
//      *
//      * @param round3PayloadReceived The round 3 payload received from the other participant.
//      * @param keyingMaterial The keying material as returned from {@link #calculateKeyingMaterial()}.
//      * @throws CryptoException if validation fails.
//      * @throws IllegalStateException if called prior to {@link #calculateKeyingMaterial()}, or multiple times
//      */
//     public void validateRound3PayloadReceived(JPAKERound3Payload round3PayloadReceived, BigInteger keyingMaterial)
//         throws CryptoException
//     {
//         if (this.state >= STATE_ROUND_3_VALIDATED)
//         {
//             throw new IllegalStateException("Validation already attempted for round3 payload for" + participantId);
//         }
//         if (this.state < STATE_KEY_CALCULATED)
//         {
//             throw new IllegalStateException("Keying material must be calculated validated prior to validating Round3 payload for " + this.participantId);
//         }
//         JPAKEUtil.validateParticipantIdsDiffer(participantId, round3PayloadReceived.getParticipantId());
//         JPAKEUtil.validateParticipantIdsEqual(this.partnerParticipantId, round3PayloadReceived.getParticipantId());

//         JPAKEUtil.validateMacTag(
//             this.participantId,
//             this.partnerParticipantId,
//             this.gx1,
//             this.gx2,
//             this.gx3,
//             this.gx4,
//             keyingMaterial,
//             this.digest,
//             round3PayloadReceived.getMacTag());
        
        
//         /*
//          * Clear the rest of the fields.
//          */
//         this.gx1 = null;
//         this.gx2 = null;
//         this.gx3 = null;
//         this.gx4 = null;

//         this.state = STATE_ROUND_3_VALIDATED;
//     }

// }

use num_bigint::{BigInt, RandBigInt};
use rand::{CryptoRng, RngCore};
use sha2::{Sha256, Digest};
use zeroize::{Zeroize, Zeroizing};

/// J-PAKE 参与者状态枚举
#[derive(Debug, PartialEq)]
pub enum State {
    Initialized,
    Round1Created,
    Round1Validated,
    Round2Created,
    Round2Validated,
    KeyCalculated,
    Round3Created,
    Round3Validated,
}

pub struct JpakeParticipant {
    participant_id: String,
    partner_participant_id: Option<String>,
    password: Zeroizing<String>, // 使用 Zeroizing 确保内存中的密码在使用后被擦除
    
    // 算法参数 (p, q, g)
    p: BigInt,
    q: BigInt,
    g: BigInt,

    // 内部状态
    state: State,
    
    // 密钥交换中间变量
    x1: Option<BigInt>,
    x2: Option<BigInt>,
    gx1: Option<BigInt>,
    gx2: Option<BigInt>,
    gx3: Option<BigInt>,
    gx4: Option<BigInt>,
    b: Option<BigInt>,
}
impl JpakeParticipant {
    pub fn new(
        participant_id: &str,
        password: &str,
        p: BigInt,
        q: BigInt,
        g: BigInt,
    ) -> Result<Self, &'static str> {
        if password.is_empty() {
            return Err("Password cannot be empty");
        }

        Ok(Self {
            participant_id: participant_id.to_string(),
            partner_participant_id: None,
            password: Zeroizing::new(password.to_string()),
            p,
            q,
            g,
            state: State::Initialized,
            x1: None,
            x2: None,
            gx1: None,
            gx2: None,
            gx3: None,
            gx4: None,
            b: None,
        })
    }

    /// 创建第一轮负载（模拟 Java 的 createRound1PayloadToSend）
    pub fn create_round_1_payload<R: RngCore + CryptoRng>(&mut self, rng: &mut R) -> Result<(), &'static str> {
        if self.state != State::Initialized {
            return Err("Invalid state for Round 1");
        }

        // 生成随机数 x1, x2 (0 < x < q)
        let x1 = rng.gen_bigint_range(&BigInt::from(1), &self.q);
        let x2 = rng.gen_bigint_range(&BigInt::from(1), &self.q);

        // 计算 g^x1 和 g^x2
        self.gx1 = Some(self.g.modpow(&x1, &self.p));
        self.gx2 = Some(self.g.modpow(&x2, &self.p));
        
        self.x1 = Some(x1);
        self.x2 = Some(x2);
        
        self.state = State::Round1Created;
        Ok(())
    }
}
impl JpakeParticipant {
    /// 将密码哈希为大整数 s (用于后续模运算)
    fn get_password_hash(&self) -> BigInt {
        let mut hasher = Sha256::new();
        hasher.update(self.password.as_bytes());
        let result = hasher.finalize();
        // 将哈希值转为正的大整数，并对 q 取模确保在群范围内
        BigInt::from_bytes_be(num_bigint::Sign::Plus, &result) % &self.q
    }
}
impl JpakeParticipant {
    pub fn create_round_2_payload(&mut self) -> Result<BigInt, &'static str> {
        // 验证状态：必须已经验证了对方的 Round 1 负载
        if self.state != State::Round1Validated {
            return Err("必须先验证对方的 Round 1 负载才能进行 Round 2");
        }

        // 提取 Round 1 的中间值
        // gx1, gx2 是自己的；gx3, gx4 是对方的
        let (gx1, gx2, gx3) = match (&self.gx1, &self.gx2, &self.gx3) {
            (Some(v1), Some(v2), Some(v3)) => (v1, v2, v3),
            _ => return Err("缺失中间计算变量"),
        };

        let x2 = self.x2.as_ref().ok_or("缺失私钥 x2")?;
        let s = self.get_password_hash();

        // 计算 A = g^((x1 + x3 + x4) * x2 * s) [简化公式，具体依协议版本]
        // 标准 J-PAKE 公式通常为：
        // 新生成器 G = (g^x1 * g^x3 * g^x4)
        // 这里的逻辑通常是：b = (g^{x1+x3+x4})^{x2 * s}
        
        // 1. 计算指数: (x2 * s) mod q
        let exp = (x2 * &s) % &self.q;
        
        // 2. 计算基数: (gx1 * gx3 * gx4) mod p
        // 注意：gx4 是对方在 Round 1 发来的第二个公钥
        let gx4 = self.gx4.as_ref().ok_or("缺失对方的 gx4")?;
        let base = (gx1 * gx3 * gx4) % &self.p;

        // 3. 计算结果 b = base^exp mod p
        let b = base.modpow(&exp, &self.p);
        self.b = Some(b.clone());

        self.state = State::Round2Created;
        
        // 返回 b 供发送，实际协议还需要附带对应的 ZKP 证明
        Ok(b)
    }
}
impl JpakeParticipant {
    pub fn validate_round_2_payload(&mut self, partner_b: BigInt) -> Result<(), &'static str> {
        if self.state != State::Round2Created {
            return Err("状态错误：需先创建自己的 Round 2 负载");
        }

        // 验证 partner_b 是否在合法范围内 (1 < b < p-1)
        if partner_b <= BigInt::from(1) || partner_b >= (&self.p - 1) {
            return Err("非法的 Round 2 负载值");
        }

        // 存储对方的 B
        // 如果是 Alice，这就是对方 Bob 的 B；反之亦然。
        // 这里后续会用于计算最终会话密钥 (Session Key)
        // self.partner_b = Some(partner_b); 

        self.state = State::Round2Validated;
        Ok(())
    }
}
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

impl JpakeParticipant {
    /// 创建 Round 3 负载
    /// keying_material: 前几轮计算得出的共享密钥材料
    pub fn create_round_3_payload(&mut self, keying_material: &BigInt) -> Result<JpakeRound3Payload, &'static str> {
        if self.state != State::KeyCalculated {
            return Err("必须先计算出密钥材料才能进行 Round 3");
        }

        // 计算 MAC 标签
        let mac_tag = self.calculate_mac_tag(
            &self.participant_id,
            self.partner_participant_id.as_ref().ok_or("未知合作伙伴 ID")?,
            keying_material
        );

        self.state = State::Round3Created;

        Ok(JpakeRound3Payload::new(self.participant_id.clone(), mac_tag))
    }

    /// 模拟 JPAKEUtil.calculateMacTag
    fn calculate_mac_tag(&self, from_id: &str, to_id: &str, key: &BigInt) -> BigInt {
        let mut hasher = Sha256::new();
        
        // 拼接认证数据：KC_1_U = HMAC(K, "KC_1_U" || ID_U || ID_V || gx1 || gx2 || gx3 || gx4)
        // 注意：实际实现需严格遵循 NIST SP 800-56A 定义的拼接顺序
        hasher.update(key.to_bytes_be().1);
        hasher.update(from_id.as_bytes());
        hasher.update(to_id.as_bytes());
        
        // 将哈希结果转回 BigInt
        BigInt::from_bytes_be(num_bigint::Sign::Plus, &hasher.finalize())
    }
}
impl JpakeParticipant {
    pub fn validate_round_3_payload(
        &mut self, 
        payload: JpakeRound3Payload, 
        keying_material: &BigInt
    ) -> Result<(), &'static str> {
        if self.state != State::Round3Created && self.state != State::KeyCalculated {
            return Err("无效的状态");
        }

        // 计算期望的 MAC 标签（注意 ID 顺序与发送方相反）
        let expected_mac_tag = self.calculate_mac_tag(
            &payload.participant_id, // 对方 ID
            &self.participant_id,    // 自己 ID
            keying_material
        );

        // 安全比较（防止侧信道攻击）
        if payload.mac_tag != expected_mac_tag {
            return Err("MAC 标签验证失败：密钥不一致或密码错误");
        }

        self.state = State::Round3Validated;
        Ok(())
    }
}
impl JpakeParticipant {
    pub fn create_round_1_payload_to_send<D: digest::Digest, R: rand::RngCore + rand::CryptoRng>(
        &mut self,
        rng: &mut R,
    ) -> Result<JpakeRound1Payload, &'static str> {
        // 1. 生成隨機數 x1, x2
        let x1 = JpakeUtil::generate_x1(&self.q, rng);
        let x2 = JpakeUtil::generate_x2(&self.q, rng);

        // 2. 計算 g^x1, g^x2
        let gx1 = JpakeUtil::calculate_gx(&self.p, &self.g, &x1);
        let gx2 = JpakeUtil::calculate_gx(&self.p, &self.g, &x2);

        // 3. 計算 ZKP
        let (gv1, r1) = JpakeUtil::calculate_zero_knowledge_proof::<D, R>(
            &self.p, &self.q, &self.g, &gx1, &x1, &self.participant_id, rng
        );
        let (gv2, r2) = JpakeUtil::calculate_zero_knowledge_proof::<D, R>(
            &self.p, &self.q, &self.g, &gx2, &x2, &self.participant_id, rng
        );

        // 保存狀態
        self.x1 = Some(x1);
        self.x2 = Some(x2);
        self.gx1 = Some(gx1.clone());
        self.gx2 = Some(gx2.clone());
        self.state = State::Round1Created;

        Ok(JpakeRound1Payload::new(
            self.participant_id.clone(),
            gx1,
            gx2,
            JpakeZkp::new(gv1, r1),
            JpakeZkp::new(gv2, r2),
        ))
    }
}
impl JpakeParticipant {
    pub fn create_round_2_payload_to_send<D: digest::Digest, R: rand::RngCore + rand::CryptoRng>(
        &mut self,
        rng: &mut R,
    ) -> Result<JpakeRound2Payload, &'static str> {
        // 1. 狀態檢查
        if self.state != State::Round1Validated {
            return Err("必須先驗證 Round 1 負載");
        }

        // 2. 獲取必要參數
        let p = &self.p;
        let q = &self.q;
        let x2 = self.x2.as_ref().ok_or("缺失 x2")?;
        let gx1 = self.gx1.as_ref().ok_or("缺失 gx1")?;
        let gx3 = self.gx3.as_ref().ok_or("缺失 gx3")?;
        let gx4 = self.gx4.as_ref().ok_or("缺失 gx4")?;

        // 3. 計算 ga = g^x1 * g^x3 * g^x4 mod p
        let ga = JpakeUtil::calculate_ga(p, gx1, gx3, gx4);
        JpakeUtil::validate_ga(&ga)?; // 驗證 ga != 1

        // 4. 計算 s (密碼哈希) 和 x2s = (x2 * s) mod q
        let s = self.get_password_hash();
        let x2s = JpakeUtil::calculate_x2s(q, x2, &s);

        // 5. 計算 A = ga^(x2s) mod p
        let a = JpakeUtil::calculate_a(p, &ga, &x2s);

        // 6. 生成針對 x2s 的零知識證明
        // 注意：這裡的基數是 ga，而不是標準的 g
        let (gv, r) = JpakeUtil::calculate_zero_knowledge_proof::<D, R>(
            p, q, &ga, &a, &x2s, &self.participant_id, rng
        );

        self.state = State::Round2Created;

        Ok(JpakeRound2Payload::new(
            self.participant_id.clone(),
            a,
            JpakeZkp::new(gv, r),
        ))
    }
}
impl JpakeParticipant {
    /// 創建 Round 3 負載 (對應 createRound3PayloadToSend)
    /// keying_material: 前幾輪計算得出的共享密鑰材料
    pub fn create_round_3_payload_to_send(
        &mut self,
        keying_material: &BigInt,
    ) -> Result<JpakeRound3Payload, &'static str> {
        // 1. 狀態檢查：必須已計算出密鑰
        if self.state != State::KeyCalculated {
            return Err("必須先計算密鑰材料才能進行 Round 3");
        }

        // 2. 使用 Util 計算自己的 MacTag
        let partner_id = self.partner_participant_id.as_ref().ok_or("未知合作夥伴 ID")?;
        let mac_tag = self.calculate_mac_tag(
            &self.participant_id, 
            partner_id, 
            keying_material
        );

        self.state = State::Round3Created;

        Ok(JpakeRound3Payload::new(self.participant_id.clone(), mac_tag))
    }

    /// 驗證收到的 Round 3 負載 (對應 validateRound3PayloadReceived)
    pub fn validate_round_3_payload_received(
        &mut self,
        payload: JpakeRound3Payload,
        keying_material: &BigInt,
    ) -> Result<(), &'static str> {
        // 1. 狀態檢查
        if self.state != State::Round3Created && self.state != State::KeyCalculated {
            return Err("無效的驗證狀態");
        }

        // 2. 計算預期的 MacTag (注意 ID 順序：對方發過來的，所以 from 是對方)
        let expected_mac_tag = self.calculate_mac_tag(
            &payload.participant_id,
            &self.participant_id,
            keying_material
        );

        // 3. 比較標籤
        if payload.mac_tag != expected_mac_tag {
            return Err("密鑰確認失敗：MacTag 不匹配，可能密碼錯誤或遭到攻擊");
        }

        self.state = State::Round3Validated;
        Ok(())
    }
}
