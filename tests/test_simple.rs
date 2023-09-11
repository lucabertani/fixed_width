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

    assert_eq!("pippo     pippo descrizione   ".to_string(), s);
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

    assert_eq!("pippo     pippo descrizione   25   ".to_string(), s);
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

    assert_eq!("pippo     details1  details2                                                                                  ".to_string(), s);
}

#[test]
fn date_test() {
    #[derive(FixedWidth)]
    struct T {
        #[fixed_width(size = 10)]
        date1: time::Date,

        #[fixed_width(size = 10, date_format = "[year][month]")]
        date2: time::Date,
    }

    let t = T {
        date1: time::macros::date!(2023 - 09 - 11),
        date2: time::macros::date!(2023 - 09 - 11),
    };

    let s: String = t.to_string().unwrap();
    assert_eq!("20230911  202309    ", s);
}

#[test]
fn time_test() {
    #[derive(FixedWidth)]
    struct T {
        #[fixed_width(size = 10)]
        time1: time::Time,
        #[fixed_width(size = 10, time_format = "[hour padding:none]-[minute]-[second]")]
        time2: time::Time,
    }

    let t = T {
        time1: time::macros::time!(13:59),
        time2: time::macros::time!(23:59:59),
    };

    let s: String = t.to_string().unwrap();
    assert_eq!("135900    23-59-59  ", s);
}

#[test]
fn date_time_test() {
    #[derive(FixedWidth)]
    struct T {
        #[fixed_width(size = 20)]
        datetime1: time::PrimitiveDateTime,
    }

    let t = T {
        datetime1: time::macros::datetime!(2023 - 09 - 11 13:59),
    };

    let s: String = t.to_string().unwrap();
    assert_eq!("20230911 135900     ", s);
}

#[test]
fn padding_with() {
    #[derive(FixedWidth)]
    struct T {
        #[fixed_width(size = 10, pad = "*")]
        name: String,
    }

    let t = T {
        name: "pippo".to_string(),
    };

    let s: String = t.to_string().unwrap();
    assert_eq!("pippo*****", s);
}

#[test]
fn padding_right() {
    #[derive(FixedWidth)]
    struct T {
        #[fixed_width(size = 10, pad_left = "false")]
        name: String,
    }

    let t = T {
        name: "pippo".to_string(),
    };

    let s: String = t.to_string().unwrap();
    assert_eq!("     pippo", s);
}

#[test]
fn test_multi() {
    #[derive(FixedWidth)]
    struct T {
        #[fixed_width(size = 10, pad_left = "false", pad = "$")]
        name: String,

        #[fixed_width(
            size = 10,
            date_format = "[year][month]",
            pad_left = "false",
            pad = "*"
        )]
        date1: time::Date,
    }

    let t = T {
        name: "pippo".to_string(),
        date1: time::macros::date!(2023 - 09 - 11),
    };

    let s: String = t.to_string().unwrap();
    assert_eq!("$$$$$pippo****202309", s);
}
