#[doc = "Register `PLL_DSP_PWRCTRL` reader"]
pub type R = crate::R<PllDspPwrctrlSpec>;
#[doc = "Register `PLL_DSP_PWRCTRL` writer"]
pub type W = crate::W<PllDspPwrctrlSpec>;
#[doc = "Field `OFFMODE` reader - 0:0\\]
Do not use. TI Reserved."]
pub type OffmodeR = crate::BitReader;
#[doc = "Field `OFFMODE` writer - 0:0\\]
Do not use. TI Reserved."]
pub type OffmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOSCAN` reader - 1:1\\]
Do not use. TI Reserved."]
pub type IsoscanR = crate::BitReader;
#[doc = "Field `ISOSCAN` writer - 1:1\\]
Do not use. TI Reserved."]
pub type IsoscanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISORET` reader - 2:2\\]
Do not use. TI Reserved."]
pub type IsoretR = crate::BitReader;
#[doc = "Field `ISORET` writer - 2:2\\]
Do not use. TI Reserved."]
pub type IsoretW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RET` reader - 3:3\\]
Do not use. TI Reserved."]
pub type RetR = crate::BitReader;
#[doc = "Field `RET` writer - 3:3\\]
Do not use. TI Reserved."]
pub type RetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGOODIN` reader - 4:4\\]
Do not use. TI Reserved."]
pub type PgoodinR = crate::BitReader;
#[doc = "Field `PGOODIN` writer - 4:4\\]
Do not use. TI Reserved."]
pub type PgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PONIN` reader - 5:5\\]
Do not use. TI Reserved."]
pub type PoninR = crate::BitReader;
#[doc = "Field `PONIN` writer - 5:5\\]
Do not use. TI Reserved."]
pub type PoninW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn offmode(&self) -> OffmodeR {
        OffmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn isoscan(&self) -> IsoscanR {
        IsoscanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn isoret(&self) -> IsoretR {
        IsoretR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn ret(&self) -> RetR {
        RetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn pgoodin(&self) -> PgoodinR {
        PgoodinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn ponin(&self) -> PoninR {
        PoninR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn offmode(&mut self) -> OffmodeW<PllDspPwrctrlSpec> {
        OffmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn isoscan(&mut self) -> IsoscanW<PllDspPwrctrlSpec> {
        IsoscanW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn isoret(&mut self) -> IsoretW<PllDspPwrctrlSpec> {
        IsoretW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ret(&mut self) -> RetW<PllDspPwrctrlSpec> {
        RetW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pgoodin(&mut self) -> PgoodinW<PllDspPwrctrlSpec> {
        PgoodinW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ponin(&mut self) -> PoninW<PllDspPwrctrlSpec> {
        PoninW::new(self, 5)
    }
}
#[doc = "PLL_DSP_PWRCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDspPwrctrlSpec;
impl crate::RegisterSpec for PllDspPwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_dsp_pwrctrl::R`](R) reader structure"]
impl crate::Readable for PllDspPwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_dsp_pwrctrl::W`](W) writer structure"]
impl crate::Writable for PllDspPwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_DSP_PWRCTRL to value 0"]
impl crate::Resettable for PllDspPwrctrlSpec {
    const RESET_VALUE: u32 = 0;
}
