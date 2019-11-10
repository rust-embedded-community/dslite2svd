#[doc = "Reader of register RXINTERVAL5"]
pub type R = crate::R<u8, super::RXINTERVAL5>;
#[doc = "Writer for register RXINTERVAL5"]
pub type W = crate::W<u8, super::RXINTERVAL5>;
#[doc = "Register RXINTERVAL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::RXINTERVAL5 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXPOLL`"]
pub type TXPOLL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXPOLL`"]
pub struct TXPOLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPOLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RX Polling"]
    #[inline(always)]
    pub fn txpoll(&self) -> TXPOLL_R {
        TXPOLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX Polling"]
    #[inline(always)]
    pub fn txpoll(&mut self) -> TXPOLL_W {
        TXPOLL_W { w: self }
    }
}
