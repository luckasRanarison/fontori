use std::cmp::Ordering;

pub type Tag = u32;

pub const CMAP: u32 = 1668112752;
pub const GLYF: u32 = 1735162214;
pub const HEAD: u32 = 1751474532;
pub const HHEA: u32 = 1751672161;
pub const HMTX: u32 = 1752003704;
pub const LOCA: u32 = 1819239265;
pub const MAXP: u32 = 1835104368;
pub const NAME: u32 = 1851878757;
pub const POST: u32 = 1886352244;

pub const REQUIRED_TAGS: [Tag; 9] = [CMAP, GLYF, HEAD, HHEA, HMTX, LOCA, MAXP, NAME, POST];

fn tag_priority(tag: Tag) -> u8 {
    match tag {
        HEAD => 1,
        MAXP => 2,
        LOCA => 3,
        GLYF => 4,
        HHEA => 5,
        HMTX => 6,
        CMAP => 7,
        NAME => 8,
        POST => 9,
        _ => 255,
    }
}

pub fn compare_tags(a: Tag, b: Tag) -> Ordering {
    let a_priority = tag_priority(a);
    let b_priority = tag_priority(b);
    a_priority.cmp(&b_priority)
}
