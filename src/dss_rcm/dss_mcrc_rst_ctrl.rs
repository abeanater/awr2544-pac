#[doc = "Register `DSS_MCRC_RST_CTRL` reader"]
pub type R = crate::R<DssMcrcRstCtrlSpec>;
#[doc = "Register `DSS_MCRC_RST_CTRL` writer"]
pub type W = crate::W<DssMcrcRstCtrlSpec>;
#[doc = "Field `assert` reader - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of Device/IP before configuring. Writing 3'b11 will assert reset for DSS MCRC"]
pub type AssertR = crate::FieldReader;
#[doc = "Field `assert` writer - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of Device/IP before configuring. Writing 3'b11 will assert reset for DSS MCRC"]
pub type AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of Device/IP before configuring. Writing 3'b11 will assert reset for DSS MCRC"]
    #[inline(always)]
    pub fn assert(&self) -> AssertR {
        AssertR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug purpose only. Software needs to ensure the state of Device/IP before configuring. Writing 3'b11 will assert reset for DSS MCRC"]
    #[inline(always)]
    #[must_use]
    pub fn assert(&mut self) -> AssertW<DssMcrcRstCtrlSpec> {
        AssertW::new(self, 0)
    }
}
#[doc = "DSS_MCRC_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_mcrc_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_mcrc_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssMcrcRstCtrlSpec;
impl crate::RegisterSpec for DssMcrcRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_mcrc_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for DssMcrcRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_mcrc_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for DssMcrcRstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_MCRC_RST_CTRL to value 0"]
impl crate::Resettable for DssMcrcRstCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
