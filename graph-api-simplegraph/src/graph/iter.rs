use std::ops::Range;

pub(crate) enum RangeOrNoneIterator<T, Incr> {
    None {
        exhausted: bool,
    },
    Range {
        current: T,
        range: Range<T>,
        incr: Incr,
    },
}

impl<T, Incr> RangeOrNoneIterator<T, Incr>
where
    T: Copy,
    Incr: Fn(T) -> T,
{
    pub(crate) fn new(range: Option<Range<T>>, incr: Incr) -> RangeOrNoneIterator<T, Incr> {
        match range {
            None => RangeOrNoneIterator::None { exhausted: false },
            Some(r) => RangeOrNoneIterator::Range {
                current: r.start,
                range: r,
                incr,
            },
        }
    }
}

impl<T, Incr> Iterator for RangeOrNoneIterator<T, Incr>
where
    T: Copy + Clone + Ord,
    Incr: Fn(T) -> T,
{
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RangeOrNoneIterator::None { exhausted } => {
                if *exhausted {
                    None
                } else {
                    *exhausted = true;
                    Some(None)
                }
            }
            RangeOrNoneIterator::Range {
                current,
                range,
                incr,
            } => {
                if *current < range.end {
                    let current_copy = *current;
                    *current = incr(current_copy);
                    Some(Some(current_copy))
                } else {
                    None
                }
            }
        }
    }
}
