#[doc = "Register `DSS_DSP_NMI_GATE` reader"]
pub type R = crate::R<DssDspNmiGateSpec>;
#[doc = "Register `DSS_DSP_NMI_GATE` writer"]
pub type W = crate::W<DssDspNmiGateSpec>;
#[doc = "Field `gate` reader - 2:0\\]
Write 3b111 to gate the Non Maskable Interrupt to the DSP. This is not expected to be used"]
pub type GateR = crate::FieldReader;
#[doc = "Field `gate` writer - 2:0\\]
Write 3b111 to gate the Non Maskable Interrupt to the DSP. This is not expected to be used"]
pub type GateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Write 3b111 to gate the Non Maskable Interrupt to the DSP. This is not expected to be used"]
    #[inline(always)]
    pub fn gate(&self) -> GateR {
        GateR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Write 3b111 to gate the Non Maskable Interrupt to the DSP. This is not expected to be used"]
    #[inline(always)]
    #[must_use]
    pub fn gate(&mut self) -> GateW<DssDspNmiGateSpec> {
        GateW::new(self, 0)
    }
}
#[doc = "DSS_DSP_NMI_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_dsp_nmi_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_dsp_nmi_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssDspNmiGateSpec;
impl crate::RegisterSpec for DssDspNmiGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_dsp_nmi_gate::R`](R) reader structure"]
impl crate::Readable for DssDspNmiGateSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_dsp_nmi_gate::W`](W) writer structure"]
impl crate::Writable for DssDspNmiGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_DSP_NMI_GATE to value 0"]
impl crate::Resettable for DssDspNmiGateSpec {
    const RESET_VALUE: u32 = 0;
}
