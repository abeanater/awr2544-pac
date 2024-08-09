#[doc = "Register `DMM_CTRL_REG` reader"]
pub type R = crate::R<DmmCtrlRegSpec>;
#[doc = "Register `DMM_CTRL_REG` writer"]
pub type W = crate::W<DmmCtrlRegSpec>;
#[doc = "Field `dmm_pad_select` reader - 0:0\\]
0: SOC will be able to send the packet to DMMA/B 1: PAD will be able to send the packet to DMMA/B controlling from PAD"]
pub type DmmPadSelectR = crate::BitReader;
#[doc = "Field `dmm_pad_select` writer - 0:0\\]
0: SOC will be able to send the packet to DMMA/B 1: PAD will be able to send the packet to DMMA/B controlling from PAD"]
pub type DmmPadSelectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: SOC will be able to send the packet to DMMA/B 1: PAD will be able to send the packet to DMMA/B controlling from PAD"]
    #[inline(always)]
    pub fn dmm_pad_select(&self) -> DmmPadSelectR {
        DmmPadSelectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: SOC will be able to send the packet to DMMA/B 1: PAD will be able to send the packet to DMMA/B controlling from PAD"]
    #[inline(always)]
    #[must_use]
    pub fn dmm_pad_select(&mut self) -> DmmPadSelectW<DmmCtrlRegSpec> {
        DmmPadSelectW::new(self, 0)
    }
}
#[doc = "DMM_CTRL_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`dmm_ctrl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmm_ctrl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmmCtrlRegSpec;
impl crate::RegisterSpec for DmmCtrlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmm_ctrl_reg::R`](R) reader structure"]
impl crate::Readable for DmmCtrlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`dmm_ctrl_reg::W`](W) writer structure"]
impl crate::Writable for DmmCtrlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMM_CTRL_REG to value 0"]
impl crate::Resettable for DmmCtrlRegSpec {
    const RESET_VALUE: u32 = 0;
}
