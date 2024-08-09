#[doc = "Register `REGISTER12` reader"]
pub type R = crate::R<Register12Spec>;
#[doc = "Register `REGISTER12` writer"]
pub type W = crate::W<Register12Spec>;
#[doc = "Field `BGTRIMBITS` reader - 7:0\\]
Maximum: 100000 Minimum: 011111 Default: 000000"]
pub type BgtrimbitsR = crate::FieldReader;
#[doc = "Field `BGTRIMBITS` writer - 7:0\\]
Maximum: 100000 Minimum: 011111 Default: 000000"]
pub type BgtrimbitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMPTY` reader - 9:8\\]
Reserved"]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `EMPTY` writer - 9:8\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGLANEENABLE` reader - 14:10\\]
Default: 00000"]
pub type ReglaneenableR = crate::FieldReader;
#[doc = "Field `REGLANEENABLE` writer - 14:10\\]
Default: 00000"]
pub type ReglaneenableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDLANEENABLE` reader - 15:15\\]
Default: 0"]
pub type OvrrdlaneenableR = crate::BitReader;
#[doc = "Field `OVRRDLANEENABLE` writer - 15:15\\]
Default: 0"]
pub type OvrrdlaneenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLKPOST` reader - 23:16\\]
TCLK-POST used in loop-back mode - in number of DDR clocksDefault: 50 DDRCLK cycles"]
pub type TclkpostR = crate::FieldReader;
#[doc = "Field `TCLKPOST` writer - 23:16\\]
TCLK-POST used in loop-back mode - in number of DDR clocksDefault: 50 DDRCLK cycles"]
pub type TclkpostW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCLKPRE` reader - 31:24\\]
TCLK-PRE used in loop-back mode - in number of DDR clocksDefault: 4 DDRCLK cycles"]
pub type TclkpreR = crate::FieldReader;
#[doc = "Field `TCLKPRE` writer - 31:24\\]
TCLK-PRE used in loop-back mode - in number of DDR clocksDefault: 4 DDRCLK cycles"]
pub type TclkpreW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Maximum: 100000 Minimum: 011111 Default: 000000"]
    #[inline(always)]
    pub fn bgtrimbits(&self) -> BgtrimbitsR {
        BgtrimbitsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Default: 00000"]
    #[inline(always)]
    pub fn reglaneenable(&self) -> ReglaneenableR {
        ReglaneenableR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Default: 0"]
    #[inline(always)]
    pub fn ovrrdlaneenable(&self) -> OvrrdlaneenableR {
        OvrrdlaneenableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TCLK-POST used in loop-back mode - in number of DDR clocksDefault: 50 DDRCLK cycles"]
    #[inline(always)]
    pub fn tclkpost(&self) -> TclkpostR {
        TclkpostR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TCLK-PRE used in loop-back mode - in number of DDR clocksDefault: 4 DDRCLK cycles"]
    #[inline(always)]
    pub fn tclkpre(&self) -> TclkpreR {
        TclkpreR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Maximum: 100000 Minimum: 011111 Default: 000000"]
    #[inline(always)]
    #[must_use]
    pub fn bgtrimbits(&mut self) -> BgtrimbitsW<Register12Spec> {
        BgtrimbitsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register12Spec> {
        EmptyW::new(self, 8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Default: 00000"]
    #[inline(always)]
    #[must_use]
    pub fn reglaneenable(&mut self) -> ReglaneenableW<Register12Spec> {
        ReglaneenableW::new(self, 10)
    }
    #[doc = "Bit 15 - 15:15\\]
Default: 0"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdlaneenable(&mut self) -> OvrrdlaneenableW<Register12Spec> {
        OvrrdlaneenableW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TCLK-POST used in loop-back mode - in number of DDR clocksDefault: 50 DDRCLK cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tclkpost(&mut self) -> TclkpostW<Register12Spec> {
        TclkpostW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TCLK-PRE used in loop-back mode - in number of DDR clocksDefault: 4 DDRCLK cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tclkpre(&mut self) -> TclkpreW<Register12Spec> {
        TclkpreW::new(self, 24)
    }
}
#[doc = "REGISTER12\n\nYou can [`read`](crate::Reg::read) this register and get [`register12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register12Spec;
impl crate::RegisterSpec for Register12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register12::R`](R) reader structure"]
impl crate::Readable for Register12Spec {}
#[doc = "`write(|w| ..)` method takes [`register12::W`](W) writer structure"]
impl crate::Writable for Register12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER12 to value 0"]
impl crate::Resettable for Register12Spec {
    const RESET_VALUE: u32 = 0;
}
