#[doc = "Register `PLL_PER_BWCTRL` reader"]
pub type R = crate::R<PllPerBwctrlSpec>;
#[doc = "Register `PLL_PER_BWCTRL` writer"]
pub type W = crate::W<PllPerBwctrlSpec>;
#[doc = "Field `BW_INCR_DECRZ` reader - 0:0\\]
Direction of Loop Bandwidth 0x0 : decrease BW 0x1 : increase BW"]
pub type BwIncrDecrzR = crate::BitReader;
#[doc = "Field `BW_INCR_DECRZ` writer - 0:0\\]
Direction of Loop Bandwidth 0x0 : decrease BW 0x1 : increase BW"]
pub type BwIncrDecrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWCONTROL` reader - 2:1\\]
Change Loop Bandwidth"]
pub type BwcontrolR = crate::FieldReader;
#[doc = "Field `BWCONTROL` writer - 2:1\\]
Change Loop Bandwidth"]
pub type BwcontrolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Direction of Loop Bandwidth 0x0 : decrease BW 0x1 : increase BW"]
    #[inline(always)]
    pub fn bw_incr_decrz(&self) -> BwIncrDecrzR {
        BwIncrDecrzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Change Loop Bandwidth"]
    #[inline(always)]
    pub fn bwcontrol(&self) -> BwcontrolR {
        BwcontrolR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Direction of Loop Bandwidth 0x0 : decrease BW 0x1 : increase BW"]
    #[inline(always)]
    #[must_use]
    pub fn bw_incr_decrz(&mut self) -> BwIncrDecrzW<PllPerBwctrlSpec> {
        BwIncrDecrzW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Change Loop Bandwidth"]
    #[inline(always)]
    #[must_use]
    pub fn bwcontrol(&mut self) -> BwcontrolW<PllPerBwctrlSpec> {
        BwcontrolW::new(self, 1)
    }
}
#[doc = "PLL_PER_BWCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_bwctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_bwctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllPerBwctrlSpec;
impl crate::RegisterSpec for PllPerBwctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_per_bwctrl::R`](R) reader structure"]
impl crate::Readable for PllPerBwctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_per_bwctrl::W`](W) writer structure"]
impl crate::Writable for PllPerBwctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_PER_BWCTRL to value 0"]
impl crate::Resettable for PllPerBwctrlSpec {
    const RESET_VALUE: u32 = 0;
}
