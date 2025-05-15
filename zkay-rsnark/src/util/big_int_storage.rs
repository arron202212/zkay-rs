use std::sync::OnceLock;
static instance: OnceLock<BigIntStorage> = OnceLock::new();

/**
 * shares big integer constants
 *
 */
pub struct BigIntStorage {
    bigIntegerSet: HashMap<BigInteger, BigInteger>,
}
impl BigIntStorage {
    pub fn new() -> Self {
        instance.get_or_init(|| BigIntStorage::new());
        Self {
            bigIntegerSet: HashMap::new(),
        }
    }

    pub fn getBigInteger(x: BigInteger) -> BigInteger {
        bigIntegerSet.entry(x).or_insert(x);
        bigIntegerSet.get(x).unwrap().clone()
    }
}
