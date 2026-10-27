use std::ops::Range;

use similar::{DiffOp, algorithms::{Capture, Replace, myers}};

use super::{Block, join_words, words};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    pub base: Range<usize>,
    pub proposed: Range<usize>,
}

pub fn ops<T: PartialEq>(a: &[T], b: &[T]) -> Vec<DiffOp> {
    let mut hook = Replace::new(Capture::new());
    let _ = myers::diff(&mut hook, a, 0..a.len(), b, 0..b.len());
    hook.into_inner().into_ops()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Clean,
    Applied,
    Conflict,
}

#[derive(Debug, Clone)]
pub struct Planned {
    pub hunk: Hunk,
    pub current: Range<usize>,
    pub placement: Placement,
    pub merged: Option<Vec<Block>>,
}

impl Planned {
    pub fn can_apply(&self) -> bool {
        matches!(self.placement, Placement::Clean | Placement::Conflict)
    }

    pub fn replacement<'a>(&'a self, proposed: &'a [Block]) -> &'a [Block] {
        self.merged.as_deref().unwrap_or(&proposed[self.hunk.proposed.clone()])
    }
}

struct Anchored {
    hunk: Hunk,
    current: Range<usize>,
    placement: Placement,
}

fn matched_indices<T: PartialEq>(base: &[T], other: &[T]) -> Vec<Option<usize>> {
    let mut matched = vec![None; base.len()];
    for op in ops(base, other) {
        if let DiffOp::Equal { old_index, new_index, len } = op {
            for offset in 0..len {
                matched[old_index + offset] = Some(new_index + offset);
            }
        }
    }
    matched
}

fn anchored<T: PartialEq>(base: &[T], proposed: &[T], current: &[T]) -> Vec<Anchored> {
    let in_proposed = matched_indices(base, proposed);
    let in_current = matched_indices(base, current);
    let mut anchors: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];
    for (index, (proposed, current)) in in_proposed.iter().zip(&in_current).enumerate() {
        if let (Some(proposed), Some(current)) = (proposed, current) {
            anchors.push((index + 1, proposed + 1, current + 1));
        }
    }
    anchors.push((base.len() + 1, proposed.len() + 1, current.len() + 1));

    anchors
        .windows(2)
        .filter_map(|pair| {
            let ((base_start, proposed_start, current_start), (base_end, proposed_end, current_end)) = (pair[0], pair[1]);
            let hunk = Hunk { base: base_start..base_end - 1, proposed: proposed_start..proposed_end - 1 };
            let current_range = current_start..current_end - 1;
            let original = &base[hunk.base.clone()];
            let theirs = &proposed[hunk.proposed.clone()];
            let yours = &current[current_range.clone()];
            if theirs == original {
                return None;
            }
            let placement = if yours == theirs {
                Placement::Applied
            } else if yours == original {
                Placement::Clean
            } else {
                Placement::Conflict
            };
            Some(Anchored { hunk, current: current_range, placement })
        })
        .collect()
}

fn merge_words(base: &[Block], proposed: &[Block], current: &[Block]) -> Option<Vec<Block>> {
    let ([base], [proposed], [current]) = (base, proposed, current) else {
        return None;
    };
    if !base.same_text_kind(proposed) || !base.same_text_kind(current) {
        return None;
    }
    let (base_words, proposed_words, current_words) = (words(base.inline_content()?), words(proposed.inline_content()?), words(current.inline_content()?));
    let hunks = anchored(&base_words, &proposed_words, &current_words);
    if hunks.iter().any(|hunk| hunk.placement == Placement::Conflict) {
        return None;
    }
    let mut merged = current_words;
    for hunk in hunks.into_iter().rev().filter(|hunk| hunk.placement == Placement::Clean) {
        merged.splice(hunk.current, proposed_words[hunk.hunk.proposed].iter().cloned());
    }
    Some(vec![current.with_inline_content(join_words(merged))])
}

pub fn plan(base: &[Block], proposed: &[Block], current: &[Block]) -> Vec<Planned> {
    anchored(base, proposed, current)
        .into_iter()
        .map(|Anchored { hunk, current: range, placement }| {
            let merged = (placement == Placement::Conflict)
                .then(|| merge_words(&base[hunk.base.clone()], &proposed[hunk.proposed.clone()], &current[range.clone()]))
                .flatten();
            let placement = match &merged {
                Some(merged) if merged.as_slice() == &current[range.clone()] => Placement::Applied,
                Some(_) => Placement::Clean,
                None => placement,
            };
            Planned { hunk, current: range, placement, merged }
        })
        .collect()
}

pub fn apply(current: &[Block], proposed: &[Block], planned: &[Planned], selected: impl Fn(usize) -> bool) -> Vec<Block> {
    let mut result = current.to_vec();
    let mut chosen: Vec<&Planned> = planned.iter().enumerate().filter(|(index, item)| selected(*index) && item.can_apply()).map(|(_, item)| item).collect();
    chosen.sort_by(|a, b| b.current.start.cmp(&a.current.start).then(b.current.end.cmp(&a.current.end)));
    for item in chosen {
        result.splice(item.current.clone(), item.replacement(proposed).iter().cloned());
    }
    result
}
