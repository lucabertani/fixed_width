use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use fixed_width::FixedWidth;
use fixed_width_derive::FixedWidth;

#[test]
fn test_various() {
    let bd: BigDecimal = BigDecimal::from_f64(24.25).unwrap();

    println!("bd: {bd}");

    println!("bd digits: {}", bd.digits());

    let i: i64 = bd.to_i64().unwrap();
    println!("bd integer: {}", i);
    let f = bd - i;
    println!("bd decimals: {}", f);
}

#[test]
fn bigdecimal_test() {
    #[derive(FixedWidth)]
    struct Test {
        #[fixed_width(size = 10, decimals = 3, pad_left = true)]
        number: BigDecimal,
    }

    let t = Test {
        number: BigDecimal::from_f64(23.45).unwrap(),
    };
    let s: String = t.to_string().unwrap();

    assert_eq!("     2345+".to_string(), s);
}