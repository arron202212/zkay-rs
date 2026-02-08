// int main(int argc, char **argv) {
// 	::testing::InitGoogleTest(&argc, argv);
// 	return RUN_ALL_TESTS();
// }
use ffec::common::double::Double;

#[macro_export]
macro_rules! dbl_vec {
    () => { Vec::<Double>::new() };
    ( $( $x:expr ),* ) => {
      {
          let mut temp_vec = Vec::<Double>::new();
          $(
              temp_vec.push( Double::from( $x ) );
          )*
          temp_vec
      }
  };
}
