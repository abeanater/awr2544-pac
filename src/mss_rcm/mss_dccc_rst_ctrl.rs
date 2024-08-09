#[doc = "Register `MSS_DCCC_RST_CTRL` reader"]
pub type R = crate::R<MssDcccRstCtrlSpec>;
#[doc = "Register `MSS_DCCC_RST_CTRL` writer"]
pub type W = crate::W<MssDcccRstCtrlSpec>;
#[doc = "Field `assert` reader - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset DCCC"]
pub type AssertR = crate::FieldReader;
#[doc = "Field `assert` writer - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset DCCC"]
pub type AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset DCCC"]
    #[inline(always)]
    pub fn assert(&self) -> AssertR {
        AssertR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of the Device/IP before configuring. Writing '111' will reset DCCC"]
    #[inline(always)]
    #[must_use]
    pub fn assert(&mut self) -> AssertW<MssDcccRstCtrlSpec> {
        AssertW::new(self, 0)
    }
}
#[doc = "MSS_DCCC_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dccc_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dccc_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDcccRstCtrlSpec;
impl crate::RegisterSpec for MssDcccRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dccc_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for MssDcccRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dccc_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for MssDcccRstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DCCC_RST_CTRL to value 0"]
impl crate::Resettable for MssDcccRstCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
