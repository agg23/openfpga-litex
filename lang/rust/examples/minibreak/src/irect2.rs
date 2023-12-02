// 2D rectangle class/helper methods based on glam ivec2
// Notes:
//    0,0 is top left
//    For all rectangle classes, br is "non-inclusive":
//    The rectangle [[10,10],[12,12]] includes [10,10], [11,11] but not [12,12]

#![allow(dead_code)]

use glam::IVec2;

// Vector helpers

// Is "at" within vect "size" rooted at 0,0?
pub fn ivec2_within(size: IVec2, at: IVec2) -> bool {
    IVec2::ZERO.cmple(at).all() && size.cmpgt(at).all()
}

// Is vector left (less than or equal, less than, less than, greater than or
// equal, greater than) vector right on all axes?
pub fn ivec2_le(left: IVec2, right: IVec2) -> bool {
    left.cmple(right).all()
}
pub fn ivec2_lt(left: IVec2, right: IVec2) -> bool {
    // Unused
    left.cmplt(right).all()
}
pub fn ivec2_ge(left: IVec2, right: IVec2) -> bool {
    left.cmpge(right).all()
}
pub fn ivec2_gt(left: IVec2, right: IVec2) -> bool {
    left.cmpgt(right).all()
}

// Rectangle class
#[derive(Debug, Clone, Copy)]
pub struct IRect2 {
    pub ul: IVec2, // Upper Left
    pub br: IVec2, // Bottom Right (non-inclusive)
}

impl IRect2 {
    pub fn new(ul: IVec2, br: IVec2) -> Self {
        Self { ul, br }
    }

    pub fn new_centered(center: IVec2, size: IVec2) -> Self {
        let br = center + size / 2; // Bias placement toward upper-left
        let ul = br - size;
        Self { ul, br }
    }

    // Arg vector is contained in rectangle
    pub fn within(&self, test: IVec2) -> bool {
        ivec2_le(self.ul, test) && ivec2_gt(self.br, test)
    }

    // Arg rectangle overlaps this one by at least one pixel
    pub fn intersect(&self, test: IRect2) -> bool {
        // Will misbehave on 0-size rects
        self.within(test.ul) || {
            let in_br = test.br + IVec2::NEG_ONE; // For testing within the point just inside must be in
            self.within(in_br) || // All 4 corners
            self.within(IVec2::new(test.ul.x, in_br.y)) ||
            self.within(IVec2::new(in_br.x, test.ul.y))
        }
    }

    // Arg rectangle is entirely contained within this one
    pub fn enclose(&self, test: IRect2) -> bool {
        ivec2_le(self.ul, test.ul) && ivec2_ge(self.br, test.br) // For testing enclose the rects only need to coincide
    }

    // Size of this rectangle
    pub fn size(&self) -> IVec2 {
        self.br - self.ul
    }

    // Integer midpoint of this rectangle
    pub fn center(&self) -> IVec2 {
        (self.br + self.ul) / 2
    }

    // Copy of this rectangle offset by arg vector
    pub fn offset(&self, by: IVec2) -> IRect2 {
        return IRect2::new(self.ul + by, self.br + by);
    }

    // Copy of this rectangle, X-offset by whatever places it inside arg rectangle
    pub fn force_enclose_x(&self, test: IRect2) -> IRect2 {
        // ASSUMES SELF SMALLER THAN TEST
        let excess = test.ul.x - self.ul.x;
        if excess > 0 {
            return self.offset(IVec2::new(excess, 0));
        }
        let excess = test.br.x - self.br.x;
        if excess < 0 {
            return self.offset(IVec2::new(excess, 0));
        }
        self.clone()
    }
}
