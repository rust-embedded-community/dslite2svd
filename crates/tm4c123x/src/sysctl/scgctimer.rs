#[doc = "Reader of register SCGCTIMER"]
pub type R = crate::R<u32, super::SCGCTIMER>;
#[doc = "Writer for register SCGCTIMER"]
pub type W = crate::W<u32, super::SCGCTIMER>;
#[doc = "Register SCGCTIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGCTIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S0`"]
pub type S0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0`"]
pub struct S0_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `S1`"]
pub type S1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1`"]
pub struct S1_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `S2`"]
pub type S2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S2`"]
pub struct S2_W<'a> {
    w: &'a mut W,
}
impl<'a> S2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `S3`"]
pub type S3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S3`"]
pub struct S3_W<'a> {
    w: &'a mut W,
}
impl<'a> S3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `S4`"]
pub type S4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S4`"]
pub struct S4_W<'a> {
    w: &'a mut W,
}
impl<'a> S4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `S5`"]
pub type S5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S5`"]
pub struct S5_W<'a> {
    w: &'a mut W,
}
impl<'a> S5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s0(&self) -> S0_R {
        S0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s1(&self) -> S1_R {
        S1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s2(&self) -> S2_R {
        S2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s3(&self) -> S3_R {
        S3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s4(&self) -> S4_R {
        S4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s5(&self) -> S5_R {
        S5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s0(&mut self) -> S0_W {
        S0_W { w: self }
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s1(&mut self) -> S1_W {
        S1_W { w: self }
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s2(&mut self) -> S2_W {
        S2_W { w: self }
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s3(&mut self) -> S3_W {
        S3_W { w: self }
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s4(&mut self) -> S4_W {
        S4_W { w: self }
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s5(&mut self) -> S5_W {
        S5_W { w: self }
    }
}
