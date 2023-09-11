use fixed_width::FixedWidth;
use fixed_width_derive::FixedWidth;

#[test]
fn simple_test() {
    #[derive(FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        name: String,
        #[fixed_width(size = 20)]
        description: String,
    }

    let t = Test {
        name: "pippo".to_string(),
        description: "pippo descrizione".to_string(),
    };
    let s: String = t.to_string().unwrap();

    assert_eq!("     pippo   pippo descrizione".to_string(), s);
}

#[test]
fn simple_test2() {
    #[derive(FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        name: String,
        #[fixed_width(size = 20)]
        description: String,
        #[fixed_width(size = 5)]
        age: u32,
    }

    let t = Test {
        name: "pippo".to_string(),
        description: "pippo descrizione".to_string(),
        age: 25,
    };
    let s: String = t.to_string().unwrap();

    assert_eq!("     pippo   pippo descrizione   25".to_string(), s);
}

#[test]
fn list_test() {
    #[derive(FixedWidth)]
    struct Master {
        #[fixed_width(size = 10)]
        name: String,

        #[fixed_width(size = 100)]
        details: Vec<Detail>,
    }

    #[derive(FixedWidth)]
    struct Detail {
        #[fixed_width(size = 10)]
        detail: String,
    }

    let t = Master {
        name: "pippo".to_string(),
        details: vec![
            Detail {
                detail: "details1".to_string(),
            },
            Detail {
                detail: "details2".to_string(),
            },
        ],
    };
    let s: String = t.to_string().unwrap();

    assert_eq!("     pippo                                                                                  details1  details2".to_string(), s);
}
