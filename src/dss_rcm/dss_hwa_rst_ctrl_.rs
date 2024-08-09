#[doc = "Register `DSS_HWA_RST_CTRL_` reader"]
pub type R = crate::R<DssHwaRstCtrl_Spec>;
#[doc = "Register `DSS_HWA_RST_CTRL_` writer"]
pub type W = crate::W<DssHwaRstCtrl_Spec>;
#[doc = "Field `assert` reader - 2:0\\]
This register is for Debug Purposes only. Reset control for DSS TPCCA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertR = crate::FieldReader;
#[doc = "Field `assert` writer - 2:0\\]
This register is for Debug Purposes only. Reset control for DSS TPCCA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This register is for Debug Purposes only. Reset control for DSS TPCCA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    pub fn assert(&self) -> AssertR {
        AssertR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This register is for Debug Purposes only. Reset control for DSS TPCCA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    #[must_use]
    pub fn assert(&mut self) -> AssertW<DssHwaRstCtrl_Spec> {
        AssertW::new(self, 0)
    }
}
#[doc = "DSS_HWA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_rst_ctrl_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_rst_ctrl_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssHwaRstCtrl_Spec;
impl crate::RegisterSpec for DssHwaRstCtrl_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_hwa_rst_ctrl_::R`](R) reader structure"]
impl crate::Readable for DssHwaRstCtrl_Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_hwa_rst_ctrl_::W`](W) writer structure"]
impl crate::Writable for DssHwaRstCtrl_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_HWA_RST_CTRL_ to value 0"]
impl crate::Resettable for DssHwaRstCtrl_Spec {
    const RESET_VALUE: u32 = 0;
}
