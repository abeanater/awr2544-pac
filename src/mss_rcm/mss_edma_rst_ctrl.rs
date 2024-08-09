#[doc = "Register `MSS_EDMA_RST_CTRL` reader"]
pub type R = crate::R<MssEdmaRstCtrlSpec>;
#[doc = "Register `MSS_EDMA_RST_CTRL` writer"]
pub type W = crate::W<MssEdmaRstCtrlSpec>;
#[doc = "Field `assert` reader - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset EDMA"]
pub type AssertR = crate::FieldReader;
#[doc = "Field `assert` writer - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset EDMA"]
pub type AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tpcca_assert` reader - 6:4\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPCCA"]
pub type TpccaAssertR = crate::FieldReader;
#[doc = "Field `tpcca_assert` writer - 6:4\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPCCA"]
pub type TpccaAssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tptca0_assert` reader - 10:8\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCA0"]
pub type Tptca0AssertR = crate::FieldReader;
#[doc = "Field `tptca0_assert` writer - 10:8\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCA0"]
pub type Tptca0AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tptca1_assert` reader - 14:12\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCA1"]
pub type Tptca1AssertR = crate::FieldReader;
#[doc = "Field `tptca1_assert` writer - 14:12\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCA1"]
pub type Tptca1AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tpccb_assert` reader - 18:16\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPCCB"]
pub type TpccbAssertR = crate::FieldReader;
#[doc = "Field `tpccb_assert` writer - 18:16\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPCCB"]
pub type TpccbAssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tptcb0_assert` reader - 26:24\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCB0"]
pub type Tptcb0AssertR = crate::FieldReader;
#[doc = "Field `tptcb0_assert` writer - 26:24\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCB0"]
pub type Tptcb0AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset EDMA"]
    #[inline(always)]
    pub fn assert(&self) -> AssertR {
        AssertR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPCCA"]
    #[inline(always)]
    pub fn tpcca_assert(&self) -> TpccaAssertR {
        TpccaAssertR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCA0"]
    #[inline(always)]
    pub fn tptca0_assert(&self) -> Tptca0AssertR {
        Tptca0AssertR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCA1"]
    #[inline(always)]
    pub fn tptca1_assert(&self) -> Tptca1AssertR {
        Tptca1AssertR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPCCB"]
    #[inline(always)]
    pub fn tpccb_assert(&self) -> TpccbAssertR {
        TpccbAssertR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCB0"]
    #[inline(always)]
    pub fn tptcb0_assert(&self) -> Tptcb0AssertR {
        Tptcb0AssertR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset EDMA"]
    #[inline(always)]
    #[must_use]
    pub fn assert(&mut self) -> AssertW<MssEdmaRstCtrlSpec> {
        AssertW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPCCA"]
    #[inline(always)]
    #[must_use]
    pub fn tpcca_assert(&mut self) -> TpccaAssertW<MssEdmaRstCtrlSpec> {
        TpccaAssertW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCA0"]
    #[inline(always)]
    #[must_use]
    pub fn tptca0_assert(&mut self) -> Tptca0AssertW<MssEdmaRstCtrlSpec> {
        Tptca0AssertW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCA1"]
    #[inline(always)]
    #[must_use]
    pub fn tptca1_assert(&mut self) -> Tptca1AssertW<MssEdmaRstCtrlSpec> {
        Tptca1AssertW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPCCB"]
    #[inline(always)]
    #[must_use]
    pub fn tpccb_assert(&mut self) -> TpccbAssertW<MssEdmaRstCtrlSpec> {
        TpccbAssertW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset MSS_TPTCB0"]
    #[inline(always)]
    #[must_use]
    pub fn tptcb0_assert(&mut self) -> Tptcb0AssertW<MssEdmaRstCtrlSpec> {
        Tptcb0AssertW::new(self, 24)
    }
}
#[doc = "MSS_EDMA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_edma_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_edma_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssEdmaRstCtrlSpec;
impl crate::RegisterSpec for MssEdmaRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_edma_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for MssEdmaRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_edma_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for MssEdmaRstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_EDMA_RST_CTRL to value 0"]
impl crate::Resettable for MssEdmaRstCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
