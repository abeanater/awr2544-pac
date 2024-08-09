#[doc = "Register `PLL_DSP_BWCTRL` reader"]
pub type R = crate::R<PllDspBwctrlSpec>;
#[doc = "Register `PLL_DSP_BWCTRL` writer"]
pub type W = crate::W<PllDspBwctrlSpec>;
#[doc = "Field `BW_INCR_DECRZ` reader - 0:0\\]
Do not use. TI Reserved."]
pub type BwIncrDecrzR = crate::BitReader;
#[doc = "Field `BW_INCR_DECRZ` writer - 0:0\\]
Do not use. TI Reserved."]
pub type BwIncrDecrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWCONTROL` reader - 2:1\\]
Do not use. TI Reserved."]
pub type BwcontrolR = crate::FieldReader;
#[doc = "Field `BWCONTROL` writer - 2:1\\]
Do not use. TI Reserved."]
pub type BwcontrolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn bw_incr_decrz(&self) -> BwIncrDecrzR {
        BwIncrDecrzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn bwcontrol(&self) -> BwcontrolR {
        BwcontrolR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bw_incr_decrz(&mut self) -> BwIncrDecrzW<PllDspBwctrlSpec> {
        BwIncrDecrzW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bwcontrol(&mut self) -> BwcontrolW<PllDspBwctrlSpec> {
        BwcontrolW::new(self, 1)
    }
}
#[doc = "PLL_DSP_BWCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_bwctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_bwctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDspBwctrlSpec;
impl crate::RegisterSpec for PllDspBwctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_dsp_bwctrl::R`](R) reader structure"]
impl crate::Readable for PllDspBwctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_dsp_bwctrl::W`](W) writer structure"]
impl crate::Writable for PllDspBwctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_DSP_BWCTRL to value 0"]
impl crate::Resettable for PllDspBwctrlSpec {
    const RESET_VALUE: u32 = 0;
}
