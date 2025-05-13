
/**
 * shares big integer constants
 *
 */
pub struct BigIntStorage {
	
	  bigIntegerSet:ConcurrentMap<BigInteger, BigInteger>,
	  instance:BigIntStorage,
}
impl BigIntStorage{
	 BigIntStorage(){
		bigIntegerSet = new ConcurrentHashMap<BigInteger, BigInteger>();
	}
	
	pub   BigIntStorage getInstance(){
		if instance == null{
			instance = BigIntStorage::new();
		}
		return instance;
	}
	
	pub  BigInteger getBigInteger(x:BigInteger ){
		bigIntegerSet.putIfAbsent(x, x);
	    return bigIntegerSet.get(x);
	}
}
