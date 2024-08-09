#[doc = "Register `DSS_DSP_INT_SEL` reader"]
pub type R = crate::R<DssDspIntSelSpec>;
#[doc = "Register `DSS_DSP_INT_SEL` writer"]
pub type W = crate::W<DssDspIntSelSpec>;
#[doc = "Field `RCSS_CSI2_ICSSM` reader - 2:0\\]
DSS DSP Interrupt selcet 0x0: CSI2 Interrupts are propagated to DSP 0x7 : ICSSM Interrupts are propagted to DSP"]
pub type RcssCsi2IcssmR = crate::FieldReader;
#[doc = "Field `RCSS_CSI2_ICSSM` writer - 2:0\\]
DSS DSP Interrupt selcet 0x0: CSI2 Interrupts are propagated to DSP 0x7 : ICSSM Interrupts are propagted to DSP"]
pub type RcssCsi2IcssmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
DSS DSP Interrupt selcet 0x0: CSI2 Interrupts are propagated to DSP 0x7 : ICSSM Interrupts are propagted to DSP"]
    #[inline(always)]
    pub fn rcss_csi2_icssm(&self) -> RcssCsi2IcssmR {
        RcssCsi2IcssmR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
DSS DSP Interrupt selcet 0x0: CSI2 Interrupts are propagated to DSP 0x7 : ICSSM Interrupts are propagted to DSP"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_csi2_icssm(&mut self) -> RcssCsi2IcssmW<DssDspIntSelSpec> {
        RcssCsi2IcssmW::new(self, 0)
    }
}
#[doc = "DSS_DSP_INT_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_dsp_int_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_dsp_int_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssDspIntSelSpec;
impl crate::RegisterSpec for DssDspIntSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_dsp_int_sel::R`](R) reader structure"]
impl crate::Readable for DssDspIntSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_dsp_int_sel::W`](W) writer structure"]
impl crate::Writable for DssDspIntSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_DSP_INT_SEL to value 0"]
impl crate::Resettable for DssDspIntSelSpec {
    const RESET_VALUE: u32 = 0;
}
