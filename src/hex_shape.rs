use crate::*;
use std::vec::Vec;
// use std::ops;

pub struct HexShape {
    hexes: Vec<HexCoord>,
}

impl HexShape {
    pub fn new(hexes: Vec<HexCoord>) -> HexShape {
        HexShape {
            hexes,
        }
    }

    pub fn contains(&self, coord: HexCoord) -> bool {
        for hex in &self.hexes {
            if *hex == coord {
                return true;
            }
        }
        false
    }

    pub fn len(&self) -> u32 {
        self.hexes.len() as u32
    }

    pub fn transformed<'a>(&'a self, transform: HexTransform) -> HexShapeView<'a> {
        HexShapeView::new(
            self,
            transform,
        )
    }

    pub fn translated(&self, translation: HexCoord) -> HexShapeView {
        HexShapeView::new(
            self,
            HexTransform::from_translation(translation),
        )
    }

    pub fn rotated(&self, rotation: i32) -> HexShapeView {
        HexShapeView::new(
            self,
            HexTransform::from_rotation(rotation),
        )
    }

    pub fn get(&self, index: u32) -> Option<HexCoord> {
        self.hexes.get(index as usize).copied()
    }

    // pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[HexCoord]>>::Output> where I: SliceIndex<[HexCoord]> {
    //     self.hexes.get(index)
    // }
}

// impl<I: SliceIndex<[HexCoord]>> ops::Index<I> for HexShape {
//     type Output = <I as SliceIndex<[HexCoord]>>::Output;

//     fn index(&self, i: I) -> &Self::Output {
//         &self.hexes[i]
//     }
// }

impl<'a> IntoIterator for &'a HexShape {
    type Item = HexCoord;
    type IntoIter = echoes_utility::CopyVecIterator<'a, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        echoes_utility::CopyVecIterator::<Self::Item>::new(&self.hexes)
    }
}





// #[derive(Copy, Clone)]
pub struct HexShapeView<'a> {
    shape: &'a HexShape,
    transform: HexTransform,
}

impl<'a> HexShapeView<'a> {
    pub fn new(shape: &'a HexShape, transform: HexTransform) -> HexShapeView<'a> {
        HexShapeView {
            shape,
            transform,
        }
    }

    pub fn contains(&self, coord: HexCoord) -> bool {
        self.shape.contains(self.transform.inverse() * coord)
    }

    pub fn len(&self) -> u32 {
        self.shape.len()
    }

    // returns a copy because we can't return a transformed slice or reference
    pub fn get(&self, index: u32) -> Option<HexCoord> {
        match self.shape.get(index) {
            Some(coord) => Some(self.transform * coord),
            None => None,

        }
    }

    pub fn transformed(&self, transform: HexTransform) -> HexShapeView {
        HexShapeView::new(
            self.shape,
            transform * self.transform,
        )
    }

    pub fn translated(&self, translation: HexCoord) -> HexShapeView {
        HexShapeView::new(
            self.shape,
            HexTransform::from_translation(translation) * self.transform,
        )
    }

    pub fn rotated(&self, rotation: i32) -> HexShapeView {
        HexShapeView::new(
            self.shape,
            HexTransform::from_rotation(rotation) * self.transform,
        )
    }
}


impl<'a> IntoIterator for &HexShapeView<'a> {
    type Item = HexCoord;
    type IntoIter = echoes_utility::CopyVecIterator<'a, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.shape.into_iter()
    }
}



// impl ops::Index<usize> for HexShapeView<'_> {
//     type Output = HexCoord;

//     fn index(&self, index: usize) -> &HexCoord {
//         &self.shape[index]
//     }
// }