macro_rules! serde_roundtrip {
        ( $testfn:tt, $x:expr, $ty:ty ) =>
            ( #[test]
                fn $testfn () {
                let obj : $ty = $x;
                println!("obj original {:?}", obj);
                let obj_json = serde_json::to_string(&obj).unwrap();
                println!("obj {}", obj_json);
                let obj_recovered: $ty = serde_json::from_str(obj_json.as_str()).unwrap();
                println!("obj recovered {:?}", obj_recovered);

                assert_eq!(obj, obj_recovered);
            }
            );
}
