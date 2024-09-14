use enum_arena::*;

#[derive(Clone, Debug, Arena)]
#[arena_id = "FooArena"]
#[ref_id = "FooRef"]
#[mut_ref_id = "FooMutRef"]
struct Foo {
    a: u64,
    b: u64,
}

#[test]
fn test_foo_arena() {
    let arena = FooArena::new(16);
    let mut refs = vec![];

    for i in 0..1024u64 {
        let mut ref_mut = arena.alloc_mut(Foo { a: i, b: i });

        ref_mut.b += 1;

        refs.push(ref_mut.freeze());
    }

    for (i, r) in refs.into_iter().enumerate() {
        assert_eq!(r.a, i as u64);
        assert_eq!(r.b, i as u64 + 1);
    }
    assert_eq!(arena.len(), 1024);
    assert_eq!(arena.capacity(), 16);
}
