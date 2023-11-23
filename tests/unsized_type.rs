use disjoint_impls::disjoint_impls;

pub trait Dispatch {
    type Group: ?Sized;
}

pub enum GroupA {}
impl Dispatch for String {
    type Group = GroupA;
}
impl<T> Dispatch for Vec<T> {
    type Group = GroupA;
}

pub struct GroupB(str);
impl Dispatch for &str {
    type Group = GroupB;
}
impl Dispatch for u32 {
    type Group = GroupB;
}

disjoint_impls! {
    pub trait Kita {
        fn kita(&self) -> String;
    }

    impl<T: Dispatch<Group = GroupA>> Kita for T {
        fn kita(&self) -> String {
            "Blanket A".to_owned()
        }
    }

    impl<T: Dispatch<Group = GroupB> + ?Sized> Kita for T {
        fn kita(&self) -> String {
            "Blanket B".to_owned()
        }
    }
}

/*
pub trait Kita {
    fn kita(&self) -> String;
}
const _: () = {
    // NOTE: Marker types are not used so they can always be ?Sized
    pub trait _Kita0<T0: ?Sized> {
        fn kita(&self) -> String;
    }
    impl<T0: Dispatch<Group = GroupA>> _Kita0<GroupA> for T0 {
        fn kita(&self) -> String {
            "Blanket A".to_owned()
        }
    }
    impl<T0: Dispatch<Group = GroupB> + ?Sized> _Kita0<GroupB> for T0 {
        fn kita(&self) -> String {
            "Blanket B".to_owned()
        }
    }
    impl<T0> Kita for T0 where T0: Dispatch, Self: _Kita0<<T0 as Dispatch>::Group> {
        fn kita(&self) -> String {
            <Self as _Kita0<<T0 as Dispatch>::Group>>::kita(self)
        }
    }
};
*/

#[test]
fn unsized_type() {
    assert_eq!("Blanket A", String::new().kita());
    assert_eq!("Blanket A", Vec::<u8>::new().kita());
    assert_eq!("Blanket B", 0_u32.kita());
    assert_eq!("Blanket B", "kita".kita());
}
