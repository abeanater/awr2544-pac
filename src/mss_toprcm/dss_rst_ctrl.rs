#[doc = "Register `DSS_RST_CTRL` reader"]
pub type R = crate::R<DssRstCtrlSpec>;
#[doc = "Register `DSS_RST_CTRL` writer"]
pub type W = crate::W<DssRstCtrlSpec>;
#[doc = "Field `assert` reader - 2:0\\]
Reset control for DSP Subsystem Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
pub type AssertR = crate::FieldReader;
#[doc = "Field `assert` writer - 2:0\\]
Reset control for DSP Subsystem Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
pub type AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Reset control for DSP Subsystem Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
    #[inline(always)]
    pub fn assert(&self) -> AssertR {
        AssertR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Reset control for DSP Subsystem Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn assert(&mut self) -> AssertW<DssRstCtrlSpec> {
        AssertW::new(self, 0)
    }
}
#[doc = "DSS_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssRstCtrlSpec;
impl crate::RegisterSpec for DssRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for DssRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for DssRstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_RST_CTRL to value 0"]
impl crate::Resettable for DssRstCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
