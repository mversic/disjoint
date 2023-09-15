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

disjoint::impls! {
    pub trait Kita {
        const NAME: &'static str;
    }

    impl<T: IntoIterator<Item = U>, U: Dispatch<Group = GroupA>> Kita for T {
        const NAME: &'static str = "Blanket A";
    }
    impl<T: IntoIterator<Item = U>, U: Dispatch<Group = GroupB>> Kita for T {
        const NAME: &'static str = "Blanket B";
    }
}

/*
pub trait Kita {
    const NAME: &'static str;
}

const _: () = {
    trait _Kita<T0> {
        const NAME: &'static str;
    }

    impl<T0: Dispatch<Group = GroupA>> _Kita<GroupA> for T0 {
        const NAME: &'static str = "Blanket A";
    }
    impl<T0: Dispatch<Group = GroupB>> _Kita<GroupB> for T0 {
        const NAME: &'static str = "Blanket B";
    }

    impl<T0: Dispatch> Kita for T0 where Self: _Kita<<T0 as Dispatch>::Group> {
        const NAME: &'static str = <Self as _Kita<<T0 as Dispatch>::Group>>::NAME;
    }
};
*/

#[test]
fn main() {
    assert_eq!("Blanket A", String::NAME);
    assert_eq!("Blanket A", Vec::<u32>::NAME);
    assert_eq!("Blanket B", u32::NAME);
    assert_eq!("Blanket B", i32::NAME);
}
