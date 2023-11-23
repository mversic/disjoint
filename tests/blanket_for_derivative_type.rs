use disjoint_impls::disjoint_impls;

pub trait Dispatch {
    type Group;
}

pub enum GroupA {}
impl Dispatch for String {
    type Group = GroupA;
}
impl<T> Dispatch for Vec<T> {
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
    pub trait Kita {
        const NAME: &'static str;
    }

    impl<T: Dispatch<Group = GroupA>> Kita for Option<T> {
        const NAME: &'static str = "Blanket A";
    }
    impl<T: Dispatch<Group = GroupB>> Kita for Option<T> {
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

    impl<T0: Dispatch<Group = GroupA>> _Kita<GroupA> for Option<T0> {
        const NAME: &'static str = "Blanket A";
    }
    impl<T0: Dispatch<Group = GroupB>> _Kita<GroupB> for Option<T0> {
        const NAME: &'static str = "Blanket B";
    }

    impl<T0> Kita for Option<T0> where T0: Dispatch, Self: _Kita<<T0 as Dispatch>::Group> {
        const NAME: &'static str = <Self as _Kita<<T0 as Dispatch>::Group>>::NAME;
    }
};
*/

#[test]
fn blanket_for_derivative_type() {
    assert_eq!("Blanket A", Option::<String>::NAME);
    assert_eq!("Blanket A", Option::<Vec::<u32>>::NAME);
    assert_eq!("Blanket B", Option::<u32>::NAME);
    assert_eq!("Blanket B", Option::<i32>::NAME);
}
