#[doc = "Register `PROBE_BUS_SEL0` reader"]
pub type R = crate::R<ProbeBusSel0Spec>;
#[doc = "Register `PROBE_BUS_SEL0` writer"]
pub type W = crate::W<ProbeBusSel0Spec>;
#[doc = "Field `sel` reader - 31:0\\]
Probe Bus 0 Mux Select"]
pub type SelR = crate::FieldReader<u32>;
#[doc = "Field `sel` writer - 31:0\\]
Probe Bus 0 Mux Select"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Probe Bus 0 Mux Select"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Probe Bus 0 Mux Select"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<ProbeBusSel0Spec> {
        SelW::new(self, 0)
    }
}
#[doc = "PROBE_BUS_SEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`probe_bus_sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`probe_bus_sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeBusSel0Spec;
impl crate::RegisterSpec for ProbeBusSel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_bus_sel0::R`](R) reader structure"]
impl crate::Readable for ProbeBusSel0Spec {}
#[doc = "`write(|w| ..)` method takes [`probe_bus_sel0::W`](W) writer structure"]
impl crate::Writable for ProbeBusSel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROBE_BUS_SEL0 to value 0"]
impl crate::Resettable for ProbeBusSel0Spec {
    const RESET_VALUE: u32 = 0;
}
