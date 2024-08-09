#[doc = "Register `MSS_SW_INT` reader"]
pub type R = crate::R<MssSwIntSpec>;
#[doc = "Register `MSS_SW_INT` writer"]
pub type W = crate::W<MssSwIntSpec>;
#[doc = "Field `pulse` reader - 4:0\\]
Write_pulse bit field: writing 1'b1 to each bit will trigger MSS_SW_INT&lt;0-4> respectively to CR5A/B."]
pub type PulseR = crate::FieldReader;
#[doc = "Field `pulse` writer - 4:0\\]
Write_pulse bit field: writing 1'b1 to each bit will trigger MSS_SW_INT&lt;0-4> respectively to CR5A/B."]
pub type PulseW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Write_pulse bit field: writing 1'b1 to each bit will trigger MSS_SW_INT&lt;0-4> respectively to CR5A/B."]
    #[inline(always)]
    pub fn pulse(&self) -> PulseR {
        PulseR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Write_pulse bit field: writing 1'b1 to each bit will trigger MSS_SW_INT&lt;0-4> respectively to CR5A/B."]
    #[inline(always)]
    #[must_use]
    pub fn pulse(&mut self) -> PulseW<MssSwIntSpec> {
        PulseW::new(self, 0)
    }
}
#[doc = "MSS_SW_INT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_sw_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_sw_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssSwIntSpec;
impl crate::RegisterSpec for MssSwIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_sw_int::R`](R) reader structure"]
impl crate::Readable for MssSwIntSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_sw_int::W`](W) writer structure"]
impl crate::Writable for MssSwIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_SW_INT to value 0"]
impl crate::Resettable for MssSwIntSpec {
    const RESET_VALUE: u32 = 0;
}
