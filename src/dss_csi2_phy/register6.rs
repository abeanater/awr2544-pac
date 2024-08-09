#[doc = "Register `REGISTER6` reader"]
pub type R = crate::R<Register6Spec>;
#[doc = "Register `REGISTER6` writer"]
pub type W = crate::W<Register6Spec>;
#[doc = "Field `BYPASSCOMP` reader - 0:0\\]
1 : Bypass LP-RX and LP-CD comparator with fast buffers 0 : Do not bypass"]
pub type BypasscompR = crate::BitReader;
#[doc = "Field `BYPASSCOMP` writer - 0:0\\]
1 : Bypass LP-RX and LP-CD comparator with fast buffers 0 : Do not bypass"]
pub type BypasscompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSCOMPFILT` reader - 1:1\\]
1 : Bypass LP-RX and LP-CD with fast buffers 0 : Do not bypass"]
pub type BypasscompfiltR = crate::BitReader;
#[doc = "Field `BYPASSCOMPFILT` writer - 1:1\\]
1 : Bypass LP-RX and LP-CD with fast buffers 0 : Do not bypass"]
pub type BypasscompfiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - 3:2\\]
Reserved"]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `EMPTY` writer - 3:2\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGLPTXEN` reader - 8:4\\]
1: Enable 0: Disable"]
pub type ReglptxenR = crate::FieldReader;
#[doc = "Field `REGLPTXEN` writer - 8:4\\]
1: Enable 0: Disable"]
pub type ReglptxenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDLPTXEN` reader - 9:9\\]
1: Override with register 0: Default"]
pub type OvrrdlptxenR = crate::BitReader;
#[doc = "Field `OVRRDLPTXEN` writer - 9:9\\]
1: Override with register 0: Default"]
pub type OvrrdlptxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDEEMPDISABLE` reader - 14:10\\]
1:Disable De-emphasis 0:Enable De-emphasis"]
pub type RegdeempdisableR = crate::FieldReader;
#[doc = "Field `REGDEEMPDISABLE` writer - 14:10\\]
1:Disable De-emphasis 0:Enable De-emphasis"]
pub type RegdeempdisableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDDEEMPDISABLE` reader - 15:15\\]
1: Override with register bit 0: Default"]
pub type OvrrddeempdisableR = crate::BitReader;
#[doc = "Field `OVRRDDEEMPDISABLE` writer - 15:15\\]
1: Override with register bit 0: Default"]
pub type OvrrddeempdisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCLKLANEADDR` reader - 18:16\\]
001: Lane0 -> clock lane 010: Lane1-> clock lane 011: Lane2 -> clock lane 101: Lane3 -> clock lane Other states are reserved. Default: 001"]
pub type RegclklaneaddrR = crate::FieldReader;
#[doc = "Field `REGCLKLANEADDR` writer - 18:16\\]
001: Lane0 -> clock lane 010: Lane1-> clock lane 011: Lane2 -> clock lane 101: Lane3 -> clock lane Other states are reserved. Default: 001"]
pub type RegclklaneaddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVRRDCLKLANEADDR` reader - 19:19\\]
1: Override with register 0: Default"]
pub type OvrrdclklaneaddrR = crate::BitReader;
#[doc = "Field `OVRRDCLKLANEADDR` writer - 19:19\\]
1: Override with register 0: Default"]
pub type OvrrdclklaneaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHSTXTERMEN` reader - 24:20\\]
1: Enable 0: Disable"]
pub type ReghstxtermenR = crate::FieldReader;
#[doc = "Field `REGHSTXTERMEN` writer - 24:20\\]
1: Enable 0: Disable"]
pub type ReghstxtermenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDHSTXTERMEN` reader - 25:25\\]
1: Override with register 0: Default"]
pub type OvrrdhstxtermenR = crate::BitReader;
#[doc = "Field `OVRRDHSTXTERMEN` writer - 25:25\\]
1: Override with register 0: Default"]
pub type OvrrdhstxtermenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHSTXEN` reader - 30:26\\]
1: Enable 0: Disable"]
pub type ReghstxenR = crate::FieldReader;
#[doc = "Field `REGHSTXEN` writer - 30:26\\]
1: Enable 0: Disable"]
pub type ReghstxenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDHSTXEN` reader - 31:31\\]
1: Override with register 0: Default"]
pub type OvrrdhstxenR = crate::BitReader;
#[doc = "Field `OVRRDHSTXEN` writer - 31:31\\]
1: Override with register 0: Default"]
pub type OvrrdhstxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1 : Bypass LP-RX and LP-CD comparator with fast buffers 0 : Do not bypass"]
    #[inline(always)]
    pub fn bypasscomp(&self) -> BypasscompR {
        BypasscompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 : Bypass LP-RX and LP-CD with fast buffers 0 : Do not bypass"]
    #[inline(always)]
    pub fn bypasscompfilt(&self) -> BypasscompfiltR {
        BypasscompfiltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:8 - 8:4\\]
1: Enable 0: Disable"]
    #[inline(always)]
    pub fn reglptxen(&self) -> ReglptxenR {
        ReglptxenR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdlptxen(&self) -> OvrrdlptxenR {
        OvrrdlptxenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:14 - 14:10\\]
1:Disable De-emphasis 0:Enable De-emphasis"]
    #[inline(always)]
    pub fn regdeempdisable(&self) -> RegdeempdisableR {
        RegdeempdisableR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrddeempdisable(&self) -> OvrrddeempdisableR {
        OvrrddeempdisableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
001: Lane0 -> clock lane 010: Lane1-> clock lane 011: Lane2 -> clock lane 101: Lane3 -> clock lane Other states are reserved. Default: 001"]
    #[inline(always)]
    pub fn regclklaneaddr(&self) -> RegclklaneaddrR {
        RegclklaneaddrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdclklaneaddr(&self) -> OvrrdclklaneaddrR {
        OvrrdclklaneaddrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:24 - 24:20\\]
1: Enable 0: Disable"]
    #[inline(always)]
    pub fn reghstxtermen(&self) -> ReghstxtermenR {
        ReghstxtermenR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdhstxtermen(&self) -> OvrrdhstxtermenR {
        OvrrdhstxtermenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
1: Enable 0: Disable"]
    #[inline(always)]
    pub fn reghstxen(&self) -> ReghstxenR {
        ReghstxenR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdhstxen(&self) -> OvrrdhstxenR {
        OvrrdhstxenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1 : Bypass LP-RX and LP-CD comparator with fast buffers 0 : Do not bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypasscomp(&mut self) -> BypasscompW<Register6Spec> {
        BypasscompW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 : Bypass LP-RX and LP-CD with fast buffers 0 : Do not bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypasscompfilt(&mut self) -> BypasscompfiltW<Register6Spec> {
        BypasscompfiltW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register6Spec> {
        EmptyW::new(self, 2)
    }
    #[doc = "Bits 4:8 - 8:4\\]
1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn reglptxen(&mut self) -> ReglptxenW<Register6Spec> {
        ReglptxenW::new(self, 4)
    }
    #[doc = "Bit 9 - 9:9\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdlptxen(&mut self) -> OvrrdlptxenW<Register6Spec> {
        OvrrdlptxenW::new(self, 9)
    }
    #[doc = "Bits 10:14 - 14:10\\]
1:Disable De-emphasis 0:Enable De-emphasis"]
    #[inline(always)]
    #[must_use]
    pub fn regdeempdisable(&mut self) -> RegdeempdisableW<Register6Spec> {
        RegdeempdisableW::new(self, 10)
    }
    #[doc = "Bit 15 - 15:15\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrddeempdisable(&mut self) -> OvrrddeempdisableW<Register6Spec> {
        OvrrddeempdisableW::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
001: Lane0 -> clock lane 010: Lane1-> clock lane 011: Lane2 -> clock lane 101: Lane3 -> clock lane Other states are reserved. Default: 001"]
    #[inline(always)]
    #[must_use]
    pub fn regclklaneaddr(&mut self) -> RegclklaneaddrW<Register6Spec> {
        RegclklaneaddrW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdclklaneaddr(&mut self) -> OvrrdclklaneaddrW<Register6Spec> {
        OvrrdclklaneaddrW::new(self, 19)
    }
    #[doc = "Bits 20:24 - 24:20\\]
1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn reghstxtermen(&mut self) -> ReghstxtermenW<Register6Spec> {
        ReghstxtermenW::new(self, 20)
    }
    #[doc = "Bit 25 - 25:25\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhstxtermen(&mut self) -> OvrrdhstxtermenW<Register6Spec> {
        OvrrdhstxtermenW::new(self, 25)
    }
    #[doc = "Bits 26:30 - 30:26\\]
1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn reghstxen(&mut self) -> ReghstxenW<Register6Spec> {
        ReghstxenW::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhstxen(&mut self) -> OvrrdhstxenW<Register6Spec> {
        OvrrdhstxenW::new(self, 31)
    }
}
#[doc = "REGISTER6\n\nYou can [`read`](crate::Reg::read) this register and get [`register6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register6Spec;
impl crate::RegisterSpec for Register6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register6::R`](R) reader structure"]
impl crate::Readable for Register6Spec {}
#[doc = "`write(|w| ..)` method takes [`register6::W`](W) writer structure"]
impl crate::Writable for Register6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER6 to value 0"]
impl crate::Resettable for Register6Spec {
    const RESET_VALUE: u32 = 0;
}
