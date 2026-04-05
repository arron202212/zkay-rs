// package org.bouncycastle.crypto.agreement.jpake;

// import java.math.BigInteger;

// import org.bouncycastle.util.Arrays;

// /**
//  * The payload sent/received during the second round of a J-PAKE exchange.
//  * <p>
//  * Each {@link JPAKEParticipant} creates and sends an instance
//  * of this payload to the other {@link JPAKEParticipant}.
//  * The payload to send should be created via
//  * {@link JPAKEParticipant#createRound2PayloadToSend()}
//  * <p>
//  * Each {@link JPAKEParticipant} must also validate the payload
//  * received from the other {@link JPAKEParticipant}.
//  * The received payload should be validated via
//  * {@link JPAKEParticipant#validateRound2PayloadReceived(JPAKERound2Payload)}
//  */
// public class JPAKERound2Payload
// {
//     /**
//      * The id of the {@link JPAKEParticipant} who created/sent this payload.
//      */
//     private final String participantId;

//     /**
//      * The value of A, as computed during round 2.
//      */
//     private final BigInteger a;

//     /**
//      * The zero knowledge proof for x2 * s.
//      * <p>
//      * This is a two element array, containing {g^v, r} for x2 * s.
//      * </p>
//      */
//     private final BigInteger[] knowledgeProofForX2s;

//     public JPAKERound2Payload(
//         String participantId,
//         BigInteger a,
//         BigInteger[] knowledgeProofForX2s)
//     {
//         JPAKEUtil.validateNotNull(participantId, "participantId");
//         JPAKEUtil.validateNotNull(a, "a");
//         JPAKEUtil.validateNotNull(knowledgeProofForX2s, "knowledgeProofForX2s");

//         this.participantId = participantId;
//         this.a = a;
//         this.knowledgeProofForX2s = Arrays.copyOf(knowledgeProofForX2s, knowledgeProofForX2s.length);
//     }

//     public String getParticipantId()
//     {
//         return participantId;
//     }

//     public BigInteger getA()
//     {
//         return a;
//     }

//     public BigInteger[] getKnowledgeProofForX2s()
//     {
//         return Arrays.copyOf(knowledgeProofForX2s, knowledgeProofForX2s.length);
//     }

// }


use num_bigint::BigInt;

/// 對應 Java 的 JPAKERound2Payload
#[derive(Clone, Debug)]
pub struct JpakeRound2Payload {
    /// 發送者的參與者 ID
    pub participant_id: String,
    /// 在 Round 2 計算出的 A 值 (A = ga^(x2*s))
    pub a: BigInt,
    /// x2 * s 的零知識證明 {g^v, r}
    pub knowledge_proof_for_x2s: JpakeZkp,
}

impl JpakeRound2Payload {
    pub fn new(
        participant_id: String,
        a: BigInt,
        kp_x2s: JpakeZkp,
    ) -> Self {
        Self {
            participant_id,
            a,
            knowledge_proof_for_x2s: kp_x2s,
        }
    }
}
