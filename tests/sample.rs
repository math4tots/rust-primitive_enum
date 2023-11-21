#[macro_use]
extern crate primitive_enum;

#[cfg(test)]
mod tests {

    primitive_enum! {
    /// Some comments about 'MyEnum'
    MyEnum u16 ;
        A,
        B,

        /// Some special comments about variant C
        C,
        D = 500,
        E,
    }

    #[test]
    fn test_enum_list() {
        use MyEnum::*;

        assert_eq!(MyEnum::list(), &[A, B, C, D, E]);

        for x in MyEnum::list() {
            assert_eq!(MyEnum::from((*x) as u16), Some(*x));
        }
    }

    #[test]
    fn test_enum_from() {
        use MyEnum::*;

        assert_eq!(MyEnum::from(0), Some(A));
        assert_eq!(MyEnum::from(1), Some(B));
        assert_eq!(MyEnum::from(2), Some(C));
        assert_eq!(MyEnum::from(3), None);
        assert_eq!(MyEnum::from(4), None);
        assert_eq!(MyEnum::from(500), Some(D));
        assert_eq!(MyEnum::from(501), Some(E));
        assert_eq!(MyEnum::from(502), None);
    }

    #[test]
    fn test_enum_from_name() {
        use MyEnum::*;

        assert_eq!(MyEnum::from_name("A"), Some(A));
        assert_eq!(MyEnum::from_name("B"), Some(B));
        assert_eq!(MyEnum::from_name("C"), Some(C));
        assert_eq!(MyEnum::from_name("D"), Some(D));
        assert_eq!(MyEnum::from_name("E"), Some(E));
        assert_eq!(MyEnum::from_name("X"), None);
        assert_eq!(MyEnum::from_name("asdf"), None);
    }

    primitive_enum! { MarkerType u32 ;
        Markercone = 0,
        Markercylinder = 1,
        Markerarrow = 2,
        Markerarrowflat = 3,
        Markerflag = 4,
        Markerringflag = 5,
        Markerring = 6,
        Markerplane = 7,
        Markerbikelogo1 = 8,
        Markerbikelogo2 = 9,
        Markernum0 = 10,
        Markernum1 = 11,
        Markernum2 = 12,
        Markernum3 = 13,
        Markernum4 = 14,
        Markernum5 = 15,
        Markernum6 = 16,
        Markernum7 = 17,
        Markernum8 = 18,
        Markernum9 = 19,
        Markerchevron1 = 20,
        Markerchevron2 = 21,
        Markerchevron3 = 22,
        Markerringflat = 23,
        Markerlap = 24,
        Markerhalo = 25,
        Markerhalopoint = 26,
        Markerhalorotate = 27,
        Markersphere = 28,
        Markermoney = 29,
        Markerlines = 30,
        Markerbeast = 31,
        Markerquestionmark = 32,
        Markertransformplane = 33,
        Markertransformhelicopter = 34,
        Markertransformboat = 35,
        Markertransformcar = 36,
        Markertransformbike = 37,
        Markertransformpushbike = 38,
        Markertransformtruck = 39,
        Markertransformparachute = 40,
        Markertransformthruster = 41,
        Markerwarp = 42,
        Markerboxes = 43,
        Markerpitlane = 44,
    }

    #[test]
    fn test_big_enum() {
        assert_eq!(MarkerType::Markercone as u16, 0);
        assert_eq!(MarkerType::Markercylinder as u16, 1);
        assert_eq!(MarkerType::Markerarrow as u16, 2);
        assert_eq!(MarkerType::Markerarrowflat as u16, 3);
        assert_eq!(MarkerType::Markerflag as u16, 4);
        assert_eq!(MarkerType::Markerringflag as u16, 5);
        assert_eq!(MarkerType::Markerring as u16, 6);
        assert_eq!(MarkerType::Markerplane as u16, 7);
        assert_eq!(MarkerType::Markerbikelogo1 as u16, 8);
        assert_eq!(MarkerType::Markerbikelogo2 as u16, 9);
        assert_eq!(MarkerType::Markernum0 as u16, 10);
        assert_eq!(MarkerType::Markernum1 as u16, 11);
        assert_eq!(MarkerType::Markernum2 as u16, 12);
        assert_eq!(MarkerType::Markernum3 as u16, 13);
        assert_eq!(MarkerType::Markernum4 as u16, 14);
        assert_eq!(MarkerType::Markernum5 as u16, 15);
        assert_eq!(MarkerType::Markernum6 as u16, 16);
        assert_eq!(MarkerType::Markernum7 as u16, 17);
        assert_eq!(MarkerType::Markernum8 as u16, 18);
        assert_eq!(MarkerType::Markernum9 as u16, 19);
        assert_eq!(MarkerType::Markerchevron1 as u16, 20);
        assert_eq!(MarkerType::Markerchevron2 as u16, 21);
        assert_eq!(MarkerType::Markerchevron3 as u16, 22);
        assert_eq!(MarkerType::Markerringflat as u16, 23);
        assert_eq!(MarkerType::Markerlap as u16, 24);
        assert_eq!(MarkerType::Markerhalo as u16, 25);
        assert_eq!(MarkerType::Markerhalopoint as u16, 26);
        assert_eq!(MarkerType::Markerhalorotate as u16, 27);
        assert_eq!(MarkerType::Markersphere as u16, 28);
        assert_eq!(MarkerType::Markermoney as u16, 29);
        assert_eq!(MarkerType::Markerlines as u16, 30);
        assert_eq!(MarkerType::Markerbeast as u16, 31);
        assert_eq!(MarkerType::Markerquestionmark as u16, 32);
        assert_eq!(MarkerType::Markertransformplane as u16, 33);
        assert_eq!(MarkerType::Markertransformhelicopter as u16, 34);
        assert_eq!(MarkerType::Markertransformboat as u16, 35);
        assert_eq!(MarkerType::Markertransformcar as u16, 36);
        assert_eq!(MarkerType::Markertransformbike as u16, 37);
        assert_eq!(MarkerType::Markertransformpushbike as u16, 38);
        assert_eq!(MarkerType::Markertransformtruck as u16, 39);
        assert_eq!(MarkerType::Markertransformparachute as u16, 40);
        assert_eq!(MarkerType::Markertransformthruster as u16, 41);
        assert_eq!(MarkerType::Markerwarp as u16, 42);
        assert_eq!(MarkerType::Markerboxes as u16, 43);
        assert_eq!(MarkerType::Markerpitlane as u16, 44);

        assert_eq!(
            MarkerType::from_name("Markercone"),
            Some(MarkerType::Markercone)
        );
        assert_eq!(
            MarkerType::from_name("Markercylinder"),
            Some(MarkerType::Markercylinder)
        );
        assert_eq!(
            MarkerType::from_name("Markerarrow"),
            Some(MarkerType::Markerarrow)
        );
        assert_eq!(
            MarkerType::from_name("Markerarrowflat"),
            Some(MarkerType::Markerarrowflat)
        );
        assert_eq!(
            MarkerType::from_name("Markerflag"),
            Some(MarkerType::Markerflag)
        );
        assert_eq!(
            MarkerType::from_name("Markerringflag"),
            Some(MarkerType::Markerringflag)
        );
        assert_eq!(
            MarkerType::from_name("Markerring"),
            Some(MarkerType::Markerring)
        );
        assert_eq!(
            MarkerType::from_name("Markerplane"),
            Some(MarkerType::Markerplane)
        );
        assert_eq!(
            MarkerType::from_name("Markerbikelogo1"),
            Some(MarkerType::Markerbikelogo1)
        );
        assert_eq!(
            MarkerType::from_name("Markerbikelogo2"),
            Some(MarkerType::Markerbikelogo2)
        );
        assert_eq!(
            MarkerType::from_name("Markernum0"),
            Some(MarkerType::Markernum0)
        );
        assert_eq!(
            MarkerType::from_name("Markernum1"),
            Some(MarkerType::Markernum1)
        );
        assert_eq!(
            MarkerType::from_name("Markernum2"),
            Some(MarkerType::Markernum2)
        );
        assert_eq!(
            MarkerType::from_name("Markernum3"),
            Some(MarkerType::Markernum3)
        );
        assert_eq!(
            MarkerType::from_name("Markernum4"),
            Some(MarkerType::Markernum4)
        );
        assert_eq!(
            MarkerType::from_name("Markernum5"),
            Some(MarkerType::Markernum5)
        );
        assert_eq!(
            MarkerType::from_name("Markernum6"),
            Some(MarkerType::Markernum6)
        );
        assert_eq!(
            MarkerType::from_name("Markernum7"),
            Some(MarkerType::Markernum7)
        );
        assert_eq!(
            MarkerType::from_name("Markernum8"),
            Some(MarkerType::Markernum8)
        );
        assert_eq!(
            MarkerType::from_name("Markernum9"),
            Some(MarkerType::Markernum9)
        );
        assert_eq!(
            MarkerType::from_name("Markerchevron1"),
            Some(MarkerType::Markerchevron1)
        );
        assert_eq!(
            MarkerType::from_name("Markerchevron2"),
            Some(MarkerType::Markerchevron2)
        );
        assert_eq!(
            MarkerType::from_name("Markerchevron3"),
            Some(MarkerType::Markerchevron3)
        );
        assert_eq!(
            MarkerType::from_name("Markerringflat"),
            Some(MarkerType::Markerringflat)
        );
        assert_eq!(
            MarkerType::from_name("Markerlap"),
            Some(MarkerType::Markerlap)
        );
        assert_eq!(
            MarkerType::from_name("Markerhalo"),
            Some(MarkerType::Markerhalo)
        );
        assert_eq!(
            MarkerType::from_name("Markerhalopoint"),
            Some(MarkerType::Markerhalopoint)
        );
        assert_eq!(
            MarkerType::from_name("Markerhalorotate"),
            Some(MarkerType::Markerhalorotate)
        );
        assert_eq!(
            MarkerType::from_name("Markersphere"),
            Some(MarkerType::Markersphere)
        );
        assert_eq!(
            MarkerType::from_name("Markermoney"),
            Some(MarkerType::Markermoney)
        );
        assert_eq!(
            MarkerType::from_name("Markerlines"),
            Some(MarkerType::Markerlines)
        );
        assert_eq!(
            MarkerType::from_name("Markerbeast"),
            Some(MarkerType::Markerbeast)
        );
        assert_eq!(
            MarkerType::from_name("Markerquestionmark"),
            Some(MarkerType::Markerquestionmark)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformplane"),
            Some(MarkerType::Markertransformplane)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformhelicopter"),
            Some(MarkerType::Markertransformhelicopter)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformboat"),
            Some(MarkerType::Markertransformboat)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformcar"),
            Some(MarkerType::Markertransformcar)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformbike"),
            Some(MarkerType::Markertransformbike)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformpushbike"),
            Some(MarkerType::Markertransformpushbike)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformtruck"),
            Some(MarkerType::Markertransformtruck)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformparachute"),
            Some(MarkerType::Markertransformparachute)
        );
        assert_eq!(
            MarkerType::from_name("Markertransformthruster"),
            Some(MarkerType::Markertransformthruster)
        );
        assert_eq!(
            MarkerType::from_name("Markerwarp"),
            Some(MarkerType::Markerwarp)
        );
        assert_eq!(
            MarkerType::from_name("Markerboxes"),
            Some(MarkerType::Markerboxes)
        );
        assert_eq!(
            MarkerType::from_name("Markerpitlane"),
            Some(MarkerType::Markerpitlane)
        );
    }

    primitive_enum! { MarkerType2 u32 ;
        A, // 0
        B, // 1
        C, // 2
        D, // 3
        E, // 4
        #[default]
        F, // 5
        G, // 6
    }

    #[test]
    fn test_enum_default() {
        assert_eq!(MarkerType2::default(), MarkerType2::F);
        assert_eq!(MarkerType2::from(0), Some(MarkerType2::A));
    }
}
