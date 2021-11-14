extern crate time_humanize;

#[cfg(test)]
mod duration {
    use time_humanize::{FromTime, HumanTime};

    #[test]
    fn now() {
        let english = HumanTime::from(0);
        assert_eq!("now", format!("{}", english));
    }

    #[test]
    fn plus_5s() {
        let english = HumanTime::from(5);
        assert_eq!("now", format!("{}", english));
    }

    #[test]
    fn minus_5s() {
        let english = HumanTime::from(-5);
        assert_eq!("now", format!("{}", english));
    }

    #[test]
    fn plus_15s() {
        let english = HumanTime::from(15);
        assert_eq!("in 15 seconds", format!("{}", english));
    }

    #[test]
    fn minus_15s() {
        let english = HumanTime::from(-15);
        assert_eq!("15 seconds ago", format!("{}", english));
    }

    #[test]
    fn plus_95s() {
        let english = HumanTime::from(95);
        assert_eq!("in 2 minutes", format!("{}", english));
    }

    #[test]
    fn minus_95s() {
        let english = HumanTime::from(-95);
        assert_eq!("2 minutes ago", format!("{}", english));
    }

    #[test]
    fn plus_125s() {
        let english = HumanTime::from(125);
        assert_eq!("in 2 minutes", format!("{}", english));
    }

    #[test]
    fn minus_125s() {
        let english = HumanTime::from(-125);
        assert_eq!("2 minutes ago", format!("{}", english));
    }

    #[test]
    fn plus_31m() {
        let english = HumanTime::from_minutes(31);
        assert_eq!("in 31 minutes", format!("{}", english));
    }

    #[test]
    fn minus_31m() {
        let english = HumanTime::from(60 * -31);
        assert_eq!("31 minutes ago", format!("{}", english));
    }

    #[test]
    fn plus_45m() {
        let english = HumanTime::from(60 * 45);
        assert_eq!("in 45 minutes", format!("{}", english));
    }

    #[test]
    fn minus_45m() {
        let english = HumanTime::from(60 * -45);
        assert_eq!("45 minutes ago", format!("{}", english));
    }

    #[test]
    fn plus_46m() {
        let english = HumanTime::from(60 * 46);
        assert_eq!("in an hour", format!("{}", english));
    }

    #[test]
    fn minus_46m() {
        let english = HumanTime::from(60 * -46);
        assert_eq!("an hour ago", format!("{}", english));
    }

    #[test]
    fn plus_1h() {
        let english = HumanTime::from(60 * 60);
        assert_eq!("in an hour", format!("{}", english));
    }

    #[test]
    fn minus_1h() {
        let english = HumanTime::from(-60 * 60);
        assert_eq!("an hour ago", format!("{}", english));
    }

    #[test]
    fn plus_12h() {
        let english = HumanTime::from(60 * 60 * 12);
        assert_eq!("in 12 hours", format!("{}", english));
    }

    #[test]
    fn minus_12h() {
        let english = HumanTime::from(60 * 60 * -12);
        assert_eq!("12 hours ago", format!("{}", english));
    }

    #[test]
    fn plus_23h() {
        let english = HumanTime::from(60 * 60 * 23);
        assert_eq!("in a day", format!("{}", english));
    }

    #[test]
    fn minus_23h() {
        let english = HumanTime::from(60 * 60 * -23);
        assert_eq!("a day ago", format!("{}", english));
    }

    #[test]
    fn plus_26h() {
        let english = HumanTime::from(60 * 60 * 26);
        assert_eq!("in a day", format!("{}", english));
    }

    #[test]
    fn minus_26h() {
        let english = HumanTime::from(60 * 60 * -26);
        assert_eq!("a day ago", format!("{}", english));
    }

    #[test]
    fn plus_1d() {
        let english = HumanTime::from(24 * 60 * 60);
        assert_eq!("in a day", format!("{}", english));
    }

    #[test]
    fn minus_1d() {
        let english = HumanTime::from(-24 * 60 * 60);
        assert_eq!("a day ago", format!("{}", english));
    }

    #[test]
    fn plus_2d() {
        let english = HumanTime::from(24 * 60 * 60 * 2);
        assert_eq!("in 2 days", format!("{}", english));
    }

    #[test]
    fn minus_2d() {
        let english = HumanTime::from(24 * 60 * 60 * -2);
        assert_eq!("2 days ago", format!("{}", english));
    }

    // #[test]
    // fn plus_6d_13h() {
    //     let english = (HumanTime::from(24 * 60 * 60 * 6) + HumanTime::from(60 * 60 * 13));
    //     assert_eq!("in a week", format!("{}", english));
    // }

    // #[test]
    // fn minus_6d_13h() {
    //     let english = (HumanTime::from(24 * 60 * 60 * -6) + HumanTime::from(60 * 60 * -13));
    //     assert_eq!("a week ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_7d() {
    //     let english = HumanTime::from(24 * 60 * 60 * 7);
    //     assert_eq!("in a week", format!("{}", english));
    // }

    // #[test]
    // fn minus_7d() {
    //     let english = HumanTime::from(24 * 60 * 60 * -7);
    //     assert_eq!("a week ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_10d() {
    //     let english = HumanTime::from(24 * 60 * 60 * 10);
    //     assert_eq!("in a week", format!("{}", english));
    // }

    // #[test]
    // fn minus_10d() {
    //     let english = HumanTime::from(24 * 60 * 60 * -10);
    //     assert_eq!("a week ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_11d() {
    //     let english = HumanTime::from(24 * 60 * 60 * 11);
    //     assert_eq!("in 2 weeks", format!("{}", english));
    // }

    // #[test]
    // fn minus_11d() {
    //     let english = HumanTime::from(24 * 60 * 60 * -11);
    //     assert_eq!("2 weeks ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_4w() {
    //     let english = Duration::weeks(4);
    //     assert_eq!("in 4 weeks", format!("{}", english));
    // }

    // #[test]
    // fn minus_4w() {
    //     let english = Duration::weeks(-4);
    //     assert_eq!("4 weeks ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_30d() {
    //     let english = HumanTime::from(24 * 60 * 60 * 30);
    //     assert_eq!("in a month", format!("{}", english));
    // }

    // #[test]
    // fn minus_30d() {
    //     let english = HumanTime::from(24 * 60 * 60 * -30);
    //     assert_eq!("a month ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_45d() {
    //     let english = HumanTime::from(24 * 60 * 60 * 45);
    //     assert_eq!("in a month", format!("{}", english));
    // }

    // #[test]
    // fn minus_45d() {
    //     let english = HumanTime::from(24 * 60 * 60 * -45);
    //     assert_eq!("a month ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_46d() {
    //     let english = HumanTime::from(24 * 60 * 60 * 46);
    //     assert_eq!("in 2 months", format!("{}", english));
    // }

    // #[test]
    // fn minus_46d() {
    //     let english = HumanTime::from(24 * 60 * 60 * -46);
    //     assert_eq!("2 months ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_24w() {
    //     let english = Duration::weeks(24);
    //     assert_eq!("in 5 months", format!("{}", english));
    // }

    // #[test]
    // fn minus_24w() {
    //     let english = Duration::weeks(-24);
    //     assert_eq!("5 months ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_26w() {
    //     let english = Duration::weeks(26);
    //     assert_eq!("in 6 months", format!("{}", english));
    // }

    // #[test]
    // fn minus_26w() {
    //     let english = Duration::weeks(-26);
    //     assert_eq!("6 months ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_50w() {
    //     let english = Duration::weeks(50);
    //     assert_eq!("in a year", format!("{}", english));
    // }

    // #[test]
    // fn minus_50w() {
    //     let english = Duration::weeks(-50);
    //     assert_eq!("a year ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_100w() {
    //     let english = Duration::weeks(100);
    //     assert_eq!("in 2 years", format!("{}", english));
    // }

    // #[test]
    // fn minus_100w() {
    //     let english = Duration::weeks(-100);
    //     assert_eq!("2 years ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_120w() {
    //     let english = Duration::weeks(120);
    //     assert_eq!("in 2 years", format!("{}", english));
    // }

    // #[test]
    // fn minus_120w() {
    //     let english = Duration::weeks(-120);
    //     assert_eq!("2 years ago", format!("{}", english));
    // }

    // #[test]
    // fn plus_200w() {
    //     let english = Duration::weeks(200);
    //     assert_eq!("in 3 years", format!("{}", english));
    // }

    // #[test]
    // fn minus_200w() {
    //     let english = Duration::weeks(-200);
    //     assert_eq!("3 years ago", format!("{}", english));
    // }
    // }
}
