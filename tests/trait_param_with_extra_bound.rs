use disjoint_impls::disjoint_impls;

pub trait Dispatch {
    type Group;
}

trait A {}

pub enum GroupA {}
impl A for u16 {}
impl Dispatch for u16 {
    type Group = GroupA;
}
impl A for i16 {}
impl Dispatch for i16 {
    type Group = GroupA;
}

pub enum GroupB {}
impl Dispatch for i32 {
    type Group = GroupB;
}
impl Dispatch for u32 {
    type Group = GroupB;
}

disjoint_impls! {
    pub trait Kita<U: From<u8> = u32> {
        const NAME: &'static str;
    }

    impl<U: Dispatch<Group = GroupA> + A, T> Kita<U> for T where U: From<u8> {
        const NAME: &'static str = "Blanket A";
    }
    impl<U: Dispatch<Group = GroupB> + From<u8>, T> Kita<U> for T {
        const NAME: &'static str = "Blanket B";
    }
}

/*
pub trait Kita<U: From<u8> = u32> {
    const NAME: &'static str;
}

const _: () = {
    pub trait _Kita0<_1: ?Sized, U: From<u8> = u32> {
        const NAME: &'static str;
    }

    impl<U: Dispatch<Group = GroupA> + A, _1> _Kita0<GroupA, U> for _1 where U: From<u8> {
        const NAME: &'static str = "Blanket A";
    }
    impl<U: Dispatch<Group = GroupB> + From<u8>, _1> _Kita0<GroupB, U> for _1 {
        const NAME: &'static str = "Blanket B";
    }

    impl<U: From<u8>, _1> Kita<U> for _1 where U: Dispatch, Self: _Kita0<<U as Dispatch>::Group, U> {
        const NAME: &'static str = <Self as _Kita0<<U as Dispatch>::Group, U>>::NAME;
    }
};
*/

#[test]
fn trait_param_with_extra_bound() {
    assert_eq!("Blanket A", <u32 as Kita<u16>>::NAME);
    assert_eq!("Blanket A", <u32 as Kita<i16>>::NAME);
    assert_eq!("Blanket B", <u32 as Kita<u32>>::NAME);
    assert_eq!("Blanket B", <u32 as Kita<i32>>::NAME);
}
