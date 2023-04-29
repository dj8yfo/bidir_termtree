#![allow(clippy::branches_sharing_code)]

#[cfg(test)]
mod tests;

use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::rc::Rc;
use std::fmt::Write;

pub trait Direction {
    
}

pub struct Up;
pub struct Down;

impl Direction for Up {
    
}
impl Direction for Down {
    
}


/// a simple recursive type which is able to render its
/// components in a tree-like format
#[derive(Debug, Clone)]
pub struct Tree<D: Display, V: Direction> {
    pub root: D,
    pub leaves: Vec<Tree<D, V>>,
    multiline: bool,
    glyphs: GlyphPalette,
    _dir: PhantomData<V>,
}

impl<D: Display, V: Direction> Tree<D, V> {
    pub fn new(root: D) -> Self {
        Tree {
            root,
            leaves: Vec::new(),
            multiline: false,
            glyphs: GlyphPalette::new(),
            _dir: PhantomData,
        }
    }

    pub fn with_leaves(mut self, leaves: impl IntoIterator<Item = impl Into<Tree<D, V>>>) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }

    /// Ensure all lines for `root` are indented
    pub fn with_multiline(mut self, yes: bool) -> Self {
        self.multiline = yes;
        self
    }

    /// Customize the rendering of this node
    pub fn with_glyphs(mut self, glyphs: GlyphPalette) -> Self {
        self.glyphs = glyphs;
        self
    }
}

impl<D: Display, V: Direction> Tree<D, V> {
    /// Ensure all lines for `root` are indented
    pub fn set_multiline(&mut self, yes: bool) -> &mut Self {
        self.multiline = yes;
        self
    }

    /// Customize the rendering of this node
    pub fn set_glyphs(&mut self, glyphs: GlyphPalette) -> &mut Self {
        self.glyphs = glyphs;
        self
    }
}

impl<D: Display, V: Direction> Tree<D, V> {
    pub fn push(&mut self, leaf: impl Into<Tree<D, V>>) -> &mut Self {
        self.leaves.push(leaf.into());
        self
    }
}

impl<D: Display, V: Direction> From<D> for Tree<D, V> {
    fn from(inner: D) -> Self {
        Self::new(inner)
    }
}

impl<D: Display, V: Direction> Extend<D> for Tree<D, V> {
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        self.leaves.extend(iter.into_iter().map(Into::into))
    }
}

impl<D: Display, V: Direction> Extend<Tree<D, V>> for Tree<D, V> {
    fn extend<T: IntoIterator<Item = Tree<D, V>>>(&mut self, iter: T) {
        self.leaves.extend(iter)
    }
}

fn print_leaf_down<D: Display>(
    leaf: &Tree<D, Down>,
    last: bool,
    f: &mut fmt::Formatter,
    spaces: &Rc<Vec<bool>>,
    self_glyphs: &GlyphPalette,
) -> fmt::Result{
    let mut prefix = (
        if last {
            leaf.glyphs.last_item
        } else {
            leaf.glyphs.middle_item
        },
        leaf.glyphs.item_indent,
    );

    if leaf.multiline {
        let rest_prefix = (
            if last {
                leaf.glyphs.last_skip
            } else {
                leaf.glyphs.middle_skip
            },
            leaf.glyphs.skip_indent,
        );
        debug_assert_eq!(prefix.0.chars().count(), rest_prefix.0.chars().count());
        debug_assert_eq!(prefix.1.chars().count(), rest_prefix.1.chars().count());

        let root = if f.alternate() {
            format!("{:#}", leaf.root)
        } else {
            format!("{:}", leaf.root)
        };
        for line in root.lines() {
            // print single line
            for s in spaces.as_slice() {
                if *s {
                    self_glyphs.last_skip.fmt(f)?;
                    self_glyphs.skip_indent.fmt(f)?;
                } else {
                    self_glyphs.middle_skip.fmt(f)?;
                    self_glyphs.skip_indent.fmt(f)?;
                }
            }
            prefix.0.fmt(f)?;
            prefix.1.fmt(f)?;
            line.fmt(f)?;
            writeln!(f)?;
            prefix = rest_prefix;
        }
    } else {
        // print single line
        for s in spaces.as_slice() {
            if *s {
                self_glyphs.last_skip.fmt(f)?;
                self_glyphs.skip_indent.fmt(f)?;
            } else {
                self_glyphs.middle_skip.fmt(f)?;
                self_glyphs.skip_indent.fmt(f)?;
            }
        }
        prefix.0.fmt(f)?;
        prefix.1.fmt(f)?;
        leaf.root.fmt(f)?; // Pass along `f.alternate()`
        writeln!(f)?;
    }
    Ok(())
}

impl<D: Display> Display for Tree<D, Down> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.root.fmt(f)?; // Pass along `f.alternate()`
        writeln!(f)?;
        let mut queue = DisplauQueue::new();
        let no_space = Rc::new(Vec::new());

        enqueue_leaves(&mut queue, self, no_space);

        while let Some((last, leaf, spaces)) = queue.pop_front() {
            print_leaf_down(leaf, last, f, &spaces, &self.glyphs)?;
            // recurse
            if !leaf.leaves.is_empty() {

                let s: &Vec<bool> = &spaces;
                let mut child_spaces = s.clone();
                child_spaces.push(last);
                let child_spaces = Rc::new(child_spaces);

                enqueue_leaves(&mut queue, leaf, child_spaces);
            }
        }
        Ok(())
    }
}

fn print_leaf_up<D: Display>(
    leaf: &Tree<D, Up>,
    last: bool,
    ask_fmt: &mut fmt::Formatter, 
    spaces: &Rc<Vec<bool>>,
    self_glyphs: &GlyphPalette,
) -> Result<String, std::fmt::Error>{
    let mut result = String::new();
    let mut prefix = (
        if last {
            leaf.glyphs.last_item_up
        } else {
            leaf.glyphs.middle_item
        },
        leaf.glyphs.item_indent,
    );

    if leaf.multiline {
        let rest_prefix = (
            leaf.glyphs.middle_skip,
            leaf.glyphs.skip_indent,
        );
        debug_assert_eq!(prefix.0.chars().count(), rest_prefix.0.chars().count());
        debug_assert_eq!(prefix.1.chars().count(), rest_prefix.1.chars().count());

        let root = if ask_fmt.alternate() {
            format!("{:#}", leaf.root)
        } else {
            format!("{:}", leaf.root)
        };
        for line in root.lines() {
            // print single line
            for s in spaces.as_slice() {
                if *s {
                    write!(&mut result, "{}", format!("{}", self_glyphs.last_skip))?;
                    write!(&mut result, "{}", format!("{}", self_glyphs.skip_indent))?;
                } else {
                    write!(&mut result, "{}", format!("{}", self_glyphs.middle_skip))?;
                    write!(&mut result, "{}", format!("{}", self_glyphs.skip_indent))?;
                }
            }
            write!(&mut result, "{}", format!("{}", prefix.0))?;
            write!(&mut result, "{}", format!("{}", prefix.1))?;
            write!(&mut result, "{}", format!("{}", line))?;
            writeln!(&mut result)?;
            prefix = rest_prefix;
        }
    } else {
        // print single line
        for s in spaces.as_slice() {
            if *s {
                write!(&mut result, "{}", format!("{}", self_glyphs.last_skip))?;
                write!(&mut result, "{}", format!("{}", self_glyphs.skip_indent))?;
            } else {
                write!(&mut result, "{}", format!("{}", self_glyphs.middle_skip))?;
                write!(&mut result, "{}", format!("{}", self_glyphs.skip_indent))?;
            }
        }

        write!(&mut result, "{}", format!("{}", prefix.0))?;
        write!(&mut result, "{}", format!("{}", prefix.1))?;
        write!(&mut result, "{}", format!("{}", leaf.root))?;
        writeln!(&mut result)?;

    }
    Ok(result)
}

impl<D: Display> Display for Tree<D, Up> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut queue = DisplauQueue::new();
        let no_space = Rc::new(Vec::new());

        let mut deque = VecDeque::new();
        let first = format!("{}\n", self.root);
        deque.push_back(first);

        enqueue_leaves(&mut queue, self.clone(), no_space);

        while let Some((last, leaf, spaces)) = queue.pop_front() {
            let leaf_printed = print_leaf_up(leaf, last, f, &spaces, &self.glyphs)?;
            deque.push_back(leaf_printed);
            // recurse
            if !leaf.leaves.is_empty() {

                let s: &Vec<bool> = &spaces;
                let mut child_spaces = s.clone();
                child_spaces.push(last);
                let child_spaces = Rc::new(child_spaces);

                enqueue_leaves(&mut queue, leaf, child_spaces);
            }
        }

        while let Some(write) = deque.pop_back() {
            write!(f, "{}", write)?;
            
        }
        Ok(())
    }
}

type DisplauQueue<'t, D, V> = VecDeque<(bool, &'t Tree<D, V>, Rc<Vec<bool>>)>;

fn enqueue_leaves<'t, D: Display, V: Direction>(
    queue: &mut DisplauQueue<'t, D, V>,
    parent: &'t Tree<D, V>,
    spaces: Rc<Vec<bool>>,
) {
    for (i, leaf) in parent.leaves.iter().rev().enumerate() {
        let last = i == 0;
        queue.push_front((last, leaf, spaces.clone()));
    }
}



#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GlyphPalette {
    pub middle_item: &'static str,
    pub last_item: &'static str,
    pub last_item_up: &'static str,
    pub item_indent: &'static str,

    pub middle_skip: &'static str,
    pub last_skip: &'static str,
    pub skip_indent: &'static str,
}

impl GlyphPalette {
    pub const fn new() -> Self {
        Self {
            middle_item: "├",
            last_item: "└",
            last_item_up: "┌",
            item_indent: "── ",

            middle_skip: "│",
            last_skip: " ",
            skip_indent: "   ",
        }
    }
}

impl Default for GlyphPalette {
    fn default() -> Self {
        Self::new()
    }
}
