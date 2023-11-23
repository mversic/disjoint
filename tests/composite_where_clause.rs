use disjoint_impls::disjoint_impls;

pub trait Dispatch {
    type Group;
}

pub enum GroupA {}
impl Dispatch for Option<String> {
    type Group = GroupA;
}
impl<T> Dispatch for Option<Vec<T>> {
    type Group = GroupA;
}

pub enum GroupB {}
impl Dispatch for Option<i32> {
    type Group = GroupB;
}
impl Dispatch for Option<u32> {
    type Group = GroupB;
}

disjoint_impls! {
    pub trait Kita {
        const NAME: &'static str;
    }

    impl<T> Kita for T where Option<T>: Dispatch<Group = GroupA> {
        const NAME: &'static str = "Blanket A";
    }
    impl<T> Kita for T where Option<T>: Dispatch<Group = GroupB> {
        const NAME: &'static str = "Blanket B";
    }
}

/*
pub trait Kita {
    const NAME: &'static str;
}

const _: () = {
    trait _Kita<T0: ?Sized> {
        const NAME: &'static str;
    }

    impl<T0> _Kita<GroupA> for T0 where Option<T0>: Dispatch<Group = GroupA> {
        const NAME: &'static str = "Blanket A";
    }
    impl<T0> _Kita<GroupB> for T0 where Option<T0>: Dispatch<Group = GroupB> {
        const NAME: &'static str = "Blanket B";
    }

    impl<T0> Kita for T0 where Option<T0>: Dispatch, Self: _Kita<<Option<T0> as Dispatch>::Group> {
        const NAME: &'static str = <Self as _Kita<<Option<T0> as Dispatch>::Group>>::NAME;
    }
};
*/

#[test]
fn composite_where_clause() {
    assert_eq!("Blanket A", <String as Kita>::NAME);
    assert_eq!("Blanket A", <Vec::<u32> as Kita>::NAME);
    assert_eq!("Blanket B", <u32 as Kita>::NAME);
    assert_eq!("Blanket B", <i32 as Kita>::NAME);
}
