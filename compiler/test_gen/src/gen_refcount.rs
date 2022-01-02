#[cfg(feature = "gen-wasm")]
use crate::helpers::wasm::assert_refcounts;

#[allow(unused_imports)]
use indoc::indoc;

#[allow(unused_imports)]
use roc_std::{RocList, RocStr};

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn str_inc() {
    assert_refcounts!(
        indoc!(
            r#"
                s = Str.concat "A long enough string " "to be heap-allocated"

                [s, s, s]
            "#
        ),
        RocList<RocStr>,
        &[
            3, // s
            1  // result
        ]
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn str_dealloc() {
    assert_refcounts!(
        indoc!(
            r#"
                s = Str.concat "A long enough string " "to be heap-allocated"

                Str.isEmpty s
            "#
        ),
        bool,
        &[0]
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn list_int_inc() {
    assert_refcounts!(
        indoc!(
            r#"
                list = [0x111, 0x222, 0x333]
                [list, list, list]
            "#
        ),
        RocList<RocList<i64>>,
        &[
            3, // list
            1  // result
        ]
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn list_int_dealloc() {
    assert_refcounts!(
        indoc!(
            r#"
                list = [0x111, 0x222, 0x333]
                List.len [list, list, list]
            "#
        ),
        usize,
        &[
            0, // list
            0  // result
        ]
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn list_str_inc() {
    assert_refcounts!(
        indoc!(
            r#"
                s = Str.concat "A long enough string " "to be heap-allocated"
                list = [s, s, s]
                [list, list]
            "#
        ),
        RocList<RocList<RocStr>>,
        &[
            6, // s
            2, // list
            1  // result
        ]
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn list_str_dealloc() {
    assert_refcounts!(
        indoc!(
            r#"
                s = Str.concat "A long enough string " "to be heap-allocated"
                list = [s, s, s]
                List.len [list, list]
            "#
        ),
        usize,
        &[
            0, // s
            0, // list
            0  // result
        ]
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn struct_inc() {
    assert_refcounts!(
        indoc!(
            r#"
                s = Str.concat "A long enough string " "to be heap-allocated"
                r1 = { a: 123, b: s, c: s }
                { y: r1, z: r1 }
            "#
        ),
        [(i64, RocStr, RocStr); 2],
        &[4] // s
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn struct_dealloc() {
    assert_refcounts!(
        indoc!(
            r#"
            s = Str.concat "A long enough string " "to be heap-allocated"
            r1 = { a: 123, b: s, c: s }
            r2 = { x: 456, y: r1, z: r1 }
            r2.x
    "#
        ),
        i64,
        &[0] // s
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn union_nonrecursive_inc() {
    assert_refcounts!(
        indoc!(
            r#"
                TwoOrNone a: [ Two a a, None ]

                s = Str.concat "A long enough string " "to be heap-allocated"

                two : TwoOrNone Str
                two = Two s s

                [two, two]
            "#
        ),
        RocList<([RocStr; 2], i64)>,
        &[
            4, // s
            1  // result list
        ]
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn union_nonrecursive_dec() {
    assert_refcounts!(
        indoc!(
            r#"
                TwoOrNone a: [ Two a a, None ]

                s = Str.concat "A long enough string " "to be heap-allocated"

                two : TwoOrNone Str
                two = Two s s

                when two is
                    Two x _ -> x
                    None -> ""
            "#
        ),
        RocStr,
        &[1] // s
    );
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn union_recursive_inc() {
    assert_refcounts!(
        indoc!(
            r#"
                Expr : [ Sym Str, Add Expr Expr ]

                s = Str.concat "heap_allocated_" "symbol_name"

                e : Expr
                e = Add (Sym s) (Sym s)

                [e, e]
            "#
        ),
        RocStr,
        &[
            4, // s
            1, // Sym
            1, // Sym
            2, // Add
            1  // list
        ]
    );
}
