#[doc = "Register `DSS_CBUFF_TRIGGER_SEL` reader"]
pub type R = crate::R<DssCbuffTriggerSelSpec>;
#[doc = "Register `DSS_CBUFF_TRIGGER_SEL` writer"]
pub type W = crate::W<DssCbuffTriggerSelSpec>;
#[doc = "Field `sel` reader - 6:0\\]
DSS CBUFF HW Trigger select from DSS DSP Interrupt Map. Reset value selects RSS_ADC_CAPTURE_COMPLETE as cbuff trigger"]
pub type SelR = crate::FieldReader;
#[doc = "Field `sel` writer - 6:0\\]
DSS CBUFF HW Trigger select from DSS DSP Interrupt Map. Reset value selects RSS_ADC_CAPTURE_COMPLETE as cbuff trigger"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
DSS CBUFF HW Trigger select from DSS DSP Interrupt Map. Reset value selects RSS_ADC_CAPTURE_COMPLETE as cbuff trigger"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
DSS CBUFF HW Trigger select from DSS DSP Interrupt Map. Reset value selects RSS_ADC_CAPTURE_COMPLETE as cbuff trigger"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<DssCbuffTriggerSelSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "DSS_CBUFF_TRIGGER_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_cbuff_trigger_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_cbuff_trigger_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssCbuffTriggerSelSpec;
impl crate::RegisterSpec for DssCbuffTriggerSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_cbuff_trigger_sel::R`](R) reader structure"]
impl crate::Readable for DssCbuffTriggerSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_cbuff_trigger_sel::W`](W) writer structure"]
impl crate::Writable for DssCbuffTriggerSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_CBUFF_TRIGGER_SEL to value 0"]
impl crate::Resettable for DssCbuffTriggerSelSpec {
    const RESET_VALUE: u32 = 0;
}
