#[doc = "Register `CPTS_RFTCLK_SEL_REG` reader"]
pub type R = crate::R<CptsRftclkSelRegSpec>;
#[doc = "Register `CPTS_RFTCLK_SEL_REG` writer"]
pub type W = crate::W<CptsRftclkSelRegSpec>;
#[doc = "Field `REFERENCE_CLOCK_SELECT` reader - 4:0\\]
Reference clock select"]
pub type ReferenceClockSelectR = crate::FieldReader;
#[doc = "Field `REFERENCE_CLOCK_SELECT` writer - 4:0\\]
Reference clock select"]
pub type ReferenceClockSelectW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Reference clock select"]
    #[inline(always)]
    pub fn reference_clock_select(&self) -> ReferenceClockSelectR {
        ReferenceClockSelectR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Reference clock select"]
    #[inline(always)]
    #[must_use]
    pub fn reference_clock_select(&mut self) -> ReferenceClockSelectW<CptsRftclkSelRegSpec> {
        ReferenceClockSelectW::new(self, 0)
    }
}
#[doc = "RFTCLK Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_rftclk_sel_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_rftclk_sel_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsRftclkSelRegSpec;
impl crate::RegisterSpec for CptsRftclkSelRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_rftclk_sel_reg::R`](R) reader structure"]
impl crate::Readable for CptsRftclkSelRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_rftclk_sel_reg::W`](W) writer structure"]
impl crate::Writable for CptsRftclkSelRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_RFTCLK_SEL_REG to value 0"]
impl crate::Resettable for CptsRftclkSelRegSpec {
    const RESET_VALUE: u32 = 0;
}
