#[doc = "Reader of register PRGPIO"]
pub type R = crate::R<u32, super::PRGPIO>;
#[doc = "Reader of field `R0`"]
pub type R0_R = crate::R<bool, bool>;
#[doc = "Reader of field `R1`"]
pub type R1_R = crate::R<bool, bool>;
#[doc = "Reader of field `R2`"]
pub type R2_R = crate::R<bool, bool>;
#[doc = "Reader of field `R3`"]
pub type R3_R = crate::R<bool, bool>;
#[doc = "Reader of field `R4`"]
pub type R4_R = crate::R<bool, bool>;
#[doc = "Reader of field `R5`"]
pub type R5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Peripheral Ready"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Peripheral Ready"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Peripheral Ready"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Peripheral Ready"]
    #[inline(always)]
    pub fn r3(&self) -> R3_R {
        R3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Peripheral Ready"]
    #[inline(always)]
    pub fn r4(&self) -> R4_R {
        R4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Peripheral Ready"]
    #[inline(always)]
    pub fn r5(&self) -> R5_R {
        R5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
