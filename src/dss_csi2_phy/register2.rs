#[doc = "Register `REGISTER2` reader"]
pub type R = crate::R<Register2Spec>;
#[doc = "Register `REGISTER2` writer"]
pub type W = crate::W<Register2Spec>;
#[doc = "Field `REG_TCLKPREPARE` reader - 7:0\\]
D-PHY spec : 38ns - 95 ns Actual value seen online : = REG_TCLKPREPARE timer + analog delay &amp; slew on LP signals = REG_TCLKPREPARE * DDR Clock Period + \\[~25 ns -+ 5 ns) PROGRAMMED VALUE = ceil (65 ns / DDR Clock Period) Default value is programmed for 400 MHz"]
pub type RegTclkprepareR = crate::FieldReader;
#[doc = "Field `REG_TCLKPREPARE` writer - 7:0\\]
D-PHY spec : 38ns - 95 ns Actual value seen online : = REG_TCLKPREPARE timer + analog delay &amp; slew on LP signals = REG_TCLKPREPARE * DDR Clock Period + \\[~25 ns -+ 5 ns) PROGRAMMED VALUE = ceil (65 ns / DDR Clock Period) Default value is programmed for 400 MHz"]
pub type RegTclkprepareW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMPTY` reader - 10:8\\]
Reserved"]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `EMPTY` writer - 10:8\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REGULPMTX` reader - 15:11\\]
1: Transmit ULPM 0: Don't transmit ULPM"]
pub type RegulpmtxR = crate::FieldReader;
#[doc = "Field `REGULPMTX` writer - 15:11\\]
1: Transmit ULPM 0: Don't transmit ULPM"]
pub type RegulpmtxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDULPMTX` reader - 16:16\\]
1: Override with register 0: Default"]
pub type OvrrdulpmtxR = crate::BitReader;
#[doc = "Field `OVRRDULPMTX` writer - 16:16\\]
1: Override with register 0: Default"]
pub type OvrrdulpmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGLPTXGZ` reader - 21:17\\]
1: Tri-stated 0: Not Tri-stated"]
pub type ReglptxgzR = crate::FieldReader;
#[doc = "Field `REGLPTXGZ` writer - 21:17\\]
1: Tri-stated 0: Not Tri-stated"]
pub type ReglptxgzW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDLPTXGZ` reader - 22:22\\]
1: Override with register 0: Default"]
pub type OvrrdlptxgzR = crate::BitReader;
#[doc = "Field `OVRRDLPTXGZ` writer - 22:22\\]
1: Override with register 0: Default"]
pub type OvrrdlptxgzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATARATE` reader - 23:23\\]
This bit is set high if data rate &lt; 400Mbps => DDR clock &lt; 200MHz. 1 : Will put High-Speed Transmitters in power-saving mode. Default value : 0"]
pub type DatarateR = crate::BitReader;
#[doc = "Field `DATARATE` writer - 23:23\\]
This bit is set high if data rate &lt; 400Mbps => DDR clock &lt; 200MHz. 1 : Will put High-Speed Transmitters in power-saving mode. Default value : 0"]
pub type DatarateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSSYNCPATTERN` reader - 31:24\\]
Default : 184 (10111000). MSB (last received bit of sync pattern), LSB (first received bit of sync pattern)."]
pub type HssyncpatternR = crate::FieldReader;
#[doc = "Field `HSSYNCPATTERN` writer - 31:24\\]
Default : 184 (10111000). MSB (last received bit of sync pattern), LSB (first received bit of sync pattern)."]
pub type HssyncpatternW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
D-PHY spec : 38ns - 95 ns Actual value seen online : = REG_TCLKPREPARE timer + analog delay &amp; slew on LP signals = REG_TCLKPREPARE * DDR Clock Period + \\[~25 ns -+ 5 ns) PROGRAMMED VALUE = ceil (65 ns / DDR Clock Period) Default value is programmed for 400 MHz"]
    #[inline(always)]
    pub fn reg_tclkprepare(&self) -> RegTclkprepareR {
        RegTclkprepareR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
1: Transmit ULPM 0: Don't transmit ULPM"]
    #[inline(always)]
    pub fn regulpmtx(&self) -> RegulpmtxR {
        RegulpmtxR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdulpmtx(&self) -> OvrrdulpmtxR {
        OvrrdulpmtxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - 21:17\\]
1: Tri-stated 0: Not Tri-stated"]
    #[inline(always)]
    pub fn reglptxgz(&self) -> ReglptxgzR {
        ReglptxgzR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdlptxgz(&self) -> OvrrdlptxgzR {
        OvrrdlptxgzR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
This bit is set high if data rate &lt; 400Mbps => DDR clock &lt; 200MHz. 1 : Will put High-Speed Transmitters in power-saving mode. Default value : 0"]
    #[inline(always)]
    pub fn datarate(&self) -> DatarateR {
        DatarateR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Default : 184 (10111000). MSB (last received bit of sync pattern), LSB (first received bit of sync pattern)."]
    #[inline(always)]
    pub fn hssyncpattern(&self) -> HssyncpatternR {
        HssyncpatternR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
D-PHY spec : 38ns - 95 ns Actual value seen online : = REG_TCLKPREPARE timer + analog delay &amp; slew on LP signals = REG_TCLKPREPARE * DDR Clock Period + \\[~25 ns -+ 5 ns) PROGRAMMED VALUE = ceil (65 ns / DDR Clock Period) Default value is programmed for 400 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tclkprepare(&mut self) -> RegTclkprepareW<Register2Spec> {
        RegTclkprepareW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register2Spec> {
        EmptyW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
1: Transmit ULPM 0: Don't transmit ULPM"]
    #[inline(always)]
    #[must_use]
    pub fn regulpmtx(&mut self) -> RegulpmtxW<Register2Spec> {
        RegulpmtxW::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdulpmtx(&mut self) -> OvrrdulpmtxW<Register2Spec> {
        OvrrdulpmtxW::new(self, 16)
    }
    #[doc = "Bits 17:21 - 21:17\\]
1: Tri-stated 0: Not Tri-stated"]
    #[inline(always)]
    #[must_use]
    pub fn reglptxgz(&mut self) -> ReglptxgzW<Register2Spec> {
        ReglptxgzW::new(self, 17)
    }
    #[doc = "Bit 22 - 22:22\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdlptxgz(&mut self) -> OvrrdlptxgzW<Register2Spec> {
        OvrrdlptxgzW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
This bit is set high if data rate &lt; 400Mbps => DDR clock &lt; 200MHz. 1 : Will put High-Speed Transmitters in power-saving mode. Default value : 0"]
    #[inline(always)]
    #[must_use]
    pub fn datarate(&mut self) -> DatarateW<Register2Spec> {
        DatarateW::new(self, 23)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Default : 184 (10111000). MSB (last received bit of sync pattern), LSB (first received bit of sync pattern)."]
    #[inline(always)]
    #[must_use]
    pub fn hssyncpattern(&mut self) -> HssyncpatternW<Register2Spec> {
        HssyncpatternW::new(self, 24)
    }
}
#[doc = "REGISTER2\n\nYou can [`read`](crate::Reg::read) this register and get [`register2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register2Spec;
impl crate::RegisterSpec for Register2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register2::R`](R) reader structure"]
impl crate::Readable for Register2Spec {}
#[doc = "`write(|w| ..)` method takes [`register2::W`](W) writer structure"]
impl crate::Writable for Register2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER2 to value 0"]
impl crate::Resettable for Register2Spec {
    const RESET_VALUE: u32 = 0;
}
