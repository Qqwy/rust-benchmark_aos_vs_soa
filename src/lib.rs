#![feature(core_intrinsics)]
#![feature(allocator_api)]

use bumpalo::Bump;

pub fn partition_range_aos<T0, T1, T2, T3, T4>(
    aos: &[(T0, T1, T2, T3, T4)],
    lower_needle: &(&T0, &T1, &T2, &T3, &T4),
    higher_needle: &(&T0, &T1, &T2, &T3, &T4),
) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    unsafe {
        let lower = aos[..].partition_point(|(ref e0, ref e1, ref e2, ref e3, ref e4)| {
            (e0, e1, e2, e3, e4).cmp(lower_needle).is_lt()
        });
        let higher = lower
            + aos.get_unchecked(lower..).partition_point(
                |(ref e0, ref e1, ref e2, ref e3, ref e4)| {
                    (e0, e1, e2, e3, e4).cmp(higher_needle).is_lt()
                },
            );
        (lower, higher)
    }
}

pub fn partition_range_aos_segmented<T0, T1, T2, T3, T4>(
    aos: &[(T0, T1, T2, T3, T4)],
    lower_needle: &(&T0, &T1, &T2, &T3, &T4),
    higher_needle: &(&T0, &T1, &T2, &T3, &T4),
) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    unsafe {
        let lower = aos[..].partition_point(|(ref e0, _, _, _, _)| e0.cmp(lower_needle.0).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..)
                .partition_point(|(ref e0, _, _, _, _)| e0.cmp(higher_needle.0).is_lt());

        let lower = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, ref e1, _, _, _)| e1.cmp(lower_needle.1).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, ref e1, _, _, _)| e1.cmp(higher_needle.1).is_lt());

        let lower = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, ref e2, _, _)| e2.cmp(lower_needle.2).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, ref e2, _, _)| e2.cmp(higher_needle.2).is_lt());

        let lower = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, _, ref e3, _)| e3.cmp(lower_needle.3).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, _, ref e3, _)| e3.cmp(higher_needle.3).is_lt());

        let lower = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, _, _, ref e4)| e4.cmp(lower_needle.4).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, _, _, ref e4)| e4.cmp(higher_needle.4).is_lt());

        (lower, higher)
    }
}
pub fn partition_range_aos_segmented_alt<T0, T1, T2, T3, T4>(
    aos: &[(T0, T1, T2, T3, T4)],
    lower_needle: &(&T0, &T1, &T2, &T3, &T4),
    higher_needle: &(&T0, &T1, &T2, &T3, &T4),
) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    unsafe {
        let lower = aos[..].partition_point(|(ref e0, _, _, _, _)| e0.cmp(lower_needle.0).is_lt());

        let lower = lower
            + aos
                .get_unchecked(lower..)
                .partition_point(|(_, ref e1, _, _, _)| e1.cmp(lower_needle.1).is_lt());

        let lower = lower
            + aos
                .get_unchecked(lower..)
                .partition_point(|(_, _, ref e2, _, _)| e2.cmp(lower_needle.2).is_lt());

        let lower = lower
            + aos
                .get_unchecked(lower..)
                .partition_point(|(_, _, _, ref e3, _)| e3.cmp(lower_needle.3).is_lt());

        let lower = lower
            + aos
                .get_unchecked(lower..)
                .partition_point(|(_, _, _, _, ref e4)| e4.cmp(lower_needle.4).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..)
                .partition_point(|(ref e0, _, _, _, _)| e0.cmp(higher_needle.0).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, ref e1, _, _, _)| e1.cmp(higher_needle.1).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, ref e2, _, _)| e2.cmp(higher_needle.2).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, _, ref e3, _)| e3.cmp(higher_needle.3).is_lt());

        let higher = lower
            + aos
                .get_unchecked(lower..higher)
                .partition_point(|(_, _, _, _, ref e4)| e4.cmp(higher_needle.4).is_lt());

        (lower, higher)
    }
}

pub fn partition_range_soa<T0, T1, T2, T3, T4>(
    soa: &Soa<T0, T1, T2, T3, T4>,
    lower_needle: &(&T0, &T1, &T2, &T3, &T4),
    higher_needle: &(&T0, &T1, &T2, &T3, &T4),
) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    unsafe {
        let lower = soa.0[..].partition_point(|e0| e0.cmp(lower_needle.0).is_lt());

        let higher = lower
            + soa
                .0
                .get_unchecked(lower..)
                .partition_point(|e0| e0.cmp(higher_needle.0).is_lt());

        let lower = lower
            + soa
                .1
                .get_unchecked(lower..higher)
                .partition_point(|e1| e1.cmp(lower_needle.1).is_lt());

        let higher = lower
            + soa
                .1
                .get_unchecked(lower..higher)
                .partition_point(|e1| e1.cmp(higher_needle.1).is_lt());

        let lower = lower
            + soa
                .2
                .get_unchecked(lower..higher)
                .partition_point(|e2| e2.cmp(lower_needle.2).is_lt());

        let higher = lower
            + soa
                .2
                .get_unchecked(lower..higher)
                .partition_point(|e2| e2.cmp(higher_needle.2).is_lt());

        let lower = lower
            + soa
                .3
                .get_unchecked(lower..higher)
                .partition_point(|e3| e3.cmp(lower_needle.3).is_lt());

        let higher = lower
            + soa
                .3
                .get_unchecked(lower..higher)
                .partition_point(|e3| e3.cmp(higher_needle.3).is_lt());

        let lower = lower
            + soa
                .4
                .get_unchecked(lower..higher)
                .partition_point(|e4| e4.cmp(lower_needle.4).is_lt());

        let higher = lower
            + soa
                .4
                .get_unchecked(lower..higher)
                .partition_point(|e4| e4.cmp(higher_needle.4).is_lt());

        (lower, higher)
    }
}

pub fn partition_range_soa_alt<T0, T1, T2, T3, T4>(
    soa: &Soa<'_, T0, T1, T2, T3, T4>,
    lower_needle: &(&T0, &T1, &T2, &T3, &T4),
    higher_needle: &(&T0, &T1, &T2, &T3, &T4),
) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    unsafe {
        let lower = soa.0[..].partition_point(|e0| e0.cmp(lower_needle.0).is_lt());

        let lower = lower
            + soa
                .1
                .get_unchecked(lower..)
                .partition_point(|e1| e1.cmp(lower_needle.1).is_lt());

        let lower = lower
            + soa
                .2
                .get_unchecked(lower..)
                .partition_point(|e2| e2.cmp(lower_needle.2).is_lt());

        let lower = lower
            + soa
                .3
                .get_unchecked(lower..)
                .partition_point(|e3| e3.cmp(lower_needle.3).is_lt());

        let lower = lower
            + soa
                .4
                .get_unchecked(lower..)
                .partition_point(|e4| e4.cmp(lower_needle.4).is_lt());

        let higher = lower
            + soa
                .0
                .get_unchecked(lower..)
                .partition_point(|e0| e0.cmp(higher_needle.0).is_lt());

        let higher = lower
            + soa
                .1
                .get_unchecked(lower..higher)
                .partition_point(|e1| e1.cmp(higher_needle.1).is_lt());

        let higher = lower
            + soa
                .2
                .get_unchecked(lower..higher)
                .partition_point(|e2| e2.cmp(higher_needle.2).is_lt());

        let higher = lower
            + soa
                .3
                .get_unchecked(lower..higher)
                .partition_point(|e3| e3.cmp(higher_needle.3).is_lt());

        let higher = lower
            + soa
                .4
                .get_unchecked(lower..higher)
                .partition_point(|e4| e4.cmp(higher_needle.4).is_lt());

        (lower, higher)
    }
}

pub struct Soa<'a, T0, T1, T2, T3, T4>(Vec<T0, &'a Bump>, Vec<T1, &'a Bump>, Vec<T2, &'a Bump>, Vec<T3, &'a Bump>, Vec<T4, &'a Bump>);

pub fn expected_soa_size<T0, T1, T2, T3, T4>(slice: &[(T0, T1, T2, T3, T4)]) -> usize {
    use core::mem::size_of;
    let len = slice.len();
    len * size_of::<T0>() + len * size_of::<T1>() + len * size_of::<T2>() + len * size_of::<T3>() + len * size_of::<T4>()
}

pub fn aos_to_soa<'a, T0, T1, T2, T3, T4>(bump: &'a Bump,
    aos: Vec<(T0, T1, T2, T3, T4)>,
) -> Soa<'a, T0, T1, T2, T3, T4> {
    let len = aos.len();
    let mut soa = (
        Vec::with_capacity_in(len, bump),
        Vec::with_capacity_in(len, bump),
        Vec::with_capacity_in(len, bump),
        Vec::with_capacity_in(len, bump),
        Vec::with_capacity_in(len, bump),
    );

    for (e0, e1, e2, e3, e4) in aos {
        soa.0.push(e0);
        soa.1.push(e1);
        soa.2.push(e2);
        soa.3.push(e3);
        soa.4.push(e4);
    }
    Soa(soa.0, soa.1, soa.2, soa.3, soa.4)
}

pub fn partition_range_aos_on_soa<T0, T1, T2, T3, T4>(
    soa: &Soa<'_, T0, T1, T2, T3, T4>,
    lower_needle: &(&T0, &T1, &T2, &T3, &T4),
    higher_needle: &(&T0, &T1, &T2, &T3, &T4),
) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    let lower = soa.partition_point(|(e0, e1, e2, e3, e4)| {
        (*e0, *e1, *e2, *e3, *e4).cmp(lower_needle).is_lt()
    });
    let higher = soa.partition_point(|(e0, e1, e2, e3, e4)| {
        (*e0, *e1, *e2, *e3, *e4).cmp(higher_needle).is_lt()
    });
    (lower, higher)
}

pub fn partition_range_aos_on_soa2<T0, T1, T2, T3, T4>(
    soa: &Soa<'_, T0, T1, T2, T3, T4>,
    lower_needle: &(&T0, &T1, &T2, &T3, &T4),
    higher_needle: &(&T0, &T1, &T2, &T3, &T4),
) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    let lower_needle_rest = (lower_needle.1, lower_needle.2, lower_needle.3, lower_needle.4);
    let higher_needle_rest = (higher_needle.1, higher_needle.2, higher_needle.3, higher_needle.4);
    let lower = soa.partition_point2(
        |e0| {
            e0.cmp(lower_needle.0)
        },
        |(e1, e2, e3, e4)| {
            (*e1, *e2, *e3, *e4).cmp(&lower_needle_rest)
    });
    let higher = soa.partition_point2(
        |e0| {
            e0.cmp(higher_needle.0)
        },
        |(e1, e2, e3, e4)| {
            (*e1, *e2, *e3, *e4).cmp(&higher_needle_rest)
    });
    (lower, higher)
}


use core::cmp::Ordering::{Less, Greater};
impl<'a, T0, T1, T2, T3, T4> Soa<'a, T0, T1, T2, T3, T4> {
    // Adapted from core::slice::
    #[must_use]
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&(&T0, &T1, &T2, &T3, &T4)) -> bool,
    {
        self.binary_search_by(|x| if pred(x) { Less } else { Greater }).unwrap_or_else(|i| i)
    }

    // Adapted from core::slice::
    #[inline]
    pub fn binary_search_by<'b, F>(&'b self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&(&T0, &T1, &T2, &T3, &T4)) -> core::cmp::Ordering,
    {
        // INVARIANTS:
        // - 0 <= left <= left + size = right <= self.len()
        // - f returns Less for everything in self[..left]
        // - f returns Greater for everything in self[right..]
        let mut size = self.0.len();
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;

            // SAFETY: the while condition means `size` is strictly positive, so
            // `size/2 < size`.  Thus `left + size/2 < left + size`, which
            // coupled with the `left + size <= self.len()` invariant means
            // we have `left + size/2 < self.len()`, and this is in-bounds.
            let item = unsafe { (self.0.get_unchecked(mid), self.1.get_unchecked(mid), self.2.get_unchecked(mid), self.3.get_unchecked(mid), self.4.get_unchecked(mid)) };
            let cmp = f(&item);

            // The reason why we use if/else control flow rather than match
            // is because match reorders comparison operations, which is perf sensitive.
            // This is x86 asm for u8: https://rust.godbolt.org/z/8Y8Pra.
            if cmp == Less {
                left = mid + 1;
            } else if cmp == Greater {
                right = mid;
            } else {
                // SAFETY: same as the `get_unchecked` above
                unsafe { core::intrinsics::assume(mid < self.0.len()) };
                return Ok(mid);
            }

            size = right - left;
        }

        // SAFETY: directly true from the overall invariant.
        // Note that this is `<=`, unlike the assume in the `Ok` path.
        unsafe { core::intrinsics::assume(left <= self.0.len()) };
        Err(left)
    }
    // Adapted from core::slice::
    #[must_use]
    pub fn partition_point2<P, P2>(&self, mut pred1: P, mut pred2: P2) -> usize
    where
        P: FnMut(&T0) -> core::cmp::Ordering,
        P2: FnMut(&(&T1, &T2, &T3, &T4)) -> core::cmp::Ordering,
    {
        // self.binary_search_by(|x| if pred(x) { Less } else { Greater }).unwrap_or_else(|i| i)
        self.binary_search_by2(|one| pred1(one), |rest| pred2(rest)).unwrap_or_else(|i| i)
    }

    // Adapted from core::slice::
    #[inline]
    pub fn binary_search_by2<'b, F, F2>(&'b self, mut f: F, mut f2: F2) -> Result<usize, usize>
    where
        F: FnMut(&T0) -> core::cmp::Ordering,
        F2: FnMut(&(&T1, &T2, &T3, &T4)) -> core::cmp::Ordering,
    {
        // INVARIANTS:
        // - 0 <= left <= left + size = right <= self.len()
        // - f returns Less for everything in self[..left]
        // - f returns Greater for everything in self[right..]
        let mut size = self.0.len();
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;

            // SAFETY: the while condition means `size` is strictly positive, so
            // `size/2 < size`.  Thus `left + size/2 < left + size`, which
            // coupled with the `left + size <= self.len()` invariant means
            // we have `left + size/2 < self.len()`, and this is in-bounds.
            let cmp = f(unsafe { self.0.get_unchecked(mid)});
            let cmp = cmp.then_with(|| {
                let rest = unsafe { (self.1.get_unchecked(mid), self.2.get_unchecked(mid), self.3.get_unchecked(mid), self.4.get_unchecked(mid)) };
                f2(&rest)
            });

            // The reason why we use if/else control flow rather than match
            // is because match reorders comparison operations, which is perf sensitive.
            // This is x86 asm for u8: https://rust.godbolt.org/z/8Y8Pra.
            if cmp == Less {
                left = mid + 1;
            } else if cmp == Greater {
                right = mid;
            } else {
                // SAFETY: same as the `get_unchecked` above
                unsafe { core::intrinsics::assume(mid < self.0.len()) };
                return Ok(mid);
            }

            size = right - left;
        }

        // SAFETY: directly true from the overall invariant.
        // Note that this is `<=`, unlike the assume in the `Ok` path.
        unsafe { core::intrinsics::assume(left <= self.0.len()) };
        Err(left)
    }
}


pub fn partition_range_soa_example(
    soa: &Soa<'_, u64, u32, u128, u64, bool>,
    lower_needle: &(&u64, &u32, &u128, &u64, &bool),
    higher_needle: &(&u64, &u32, &u128, &u64, &bool),
) -> (usize, usize) {
    partition_range_soa(soa, lower_needle, higher_needle)
}

pub fn partition_range_soa_alt_example(
    soa: &Soa<'_, u64, u32, u128, u64, bool>,
    lower_needle: &(&u64, &u32, &u128, &u64, &bool),
    higher_needle: &(&u64, &u32, &u128, &u64, &bool),
) -> (usize, usize) {
    partition_range_soa_alt(soa, lower_needle, higher_needle)
}

pub fn partition_range_aos_example(
    vec: &Vec<(u64, u32, u128, u64, bool)>,
    lower_needle: &(&u64, &u32, &u128, &u64, &bool),
    higher_needle: &(&u64, &u32, &u128, &u64, &bool),
) -> (usize, usize) {
    partition_range_aos(vec, lower_needle, higher_needle)
}

pub fn partition_range_aos_segmented_example(
    vec: &Vec<(u64, u32, u128, u64, bool)>,
    lower_needle: &(&u64, &u32, &u128, &u64, &bool),
    higher_needle: &(&u64, &u32, &u128, &u64, &bool),
) -> (usize, usize) {
    partition_range_aos_segmented(vec, lower_needle, higher_needle)
}


pub fn partition_range_soa_complex_example(
    soa: &Soa<'_, u64, u32, String, u64, bool>,
    lower_needle: &(&u64, &u32, &String, &u64, &bool),
    higher_needle: &(&u64, &u32, &String, &u64, &bool),
) -> (usize, usize) {
    partition_range_soa(soa, lower_needle, higher_needle)
}

pub fn partition_range_soa_alt_complex_example(
    soa: &Soa<'_, u64, u32, String, u64, bool>,
    lower_needle: &(&u64, &u32, &String, &u64, &bool),
    higher_needle: &(&u64, &u32, &String, &u64, &bool),
) -> (usize, usize) {
    partition_range_soa_alt(soa, lower_needle, higher_needle)
}

pub fn partition_range_aos_complex_example(
    vec: &Vec<(u64, u32, String, u64, bool)>,
    lower_needle: &(&u64, &u32, &String, &u64, &bool),
    higher_needle: &(&u64, &u32, &String, &u64, &bool),
) -> (usize, usize) {
    partition_range_aos(vec, lower_needle, higher_needle)
}

pub fn partition_range_aos_segmented_complex_example(
    vec: &Vec<(u64, u32, String, u64, bool)>,
    lower_needle: &(&u64, &u32, &String, &u64, &bool),
    higher_needle: &(&u64, &u32, &String, &u64, &bool),
) -> (usize, usize) {
    partition_range_aos_segmented(vec, lower_needle, higher_needle)
}
