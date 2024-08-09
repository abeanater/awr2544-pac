#[doc = "Register `MSS_L2_A_BUS_SAFETY_ERR_STAT_READ` reader"]
pub type R = crate::R<MssL2ABusSafetyErrStatReadSpec>;
#[doc = "Register `MSS_L2_A_BUS_SAFETY_ERR_STAT_READ` writer"]
pub type W = crate::W<MssL2ABusSafetyErrStatReadSpec>;
#[doc = "Field `stat` reader - 31:0\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type StatR = crate::FieldReader<u32>;
#[doc = "Field `stat` writer - 31:0\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type StatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<MssL2ABusSafetyErrStatReadSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_err_stat_read::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_err_stat_read::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssL2ABusSafetyErrStatReadSpec;
impl crate::RegisterSpec for MssL2ABusSafetyErrStatReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_l2_a_bus_safety_err_stat_read::R`](R) reader structure"]
impl crate::Readable for MssL2ABusSafetyErrStatReadSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_l2_a_bus_safety_err_stat_read::W`](W) writer structure"]
impl crate::Writable for MssL2ABusSafetyErrStatReadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_L2_A_BUS_SAFETY_ERR_STAT_READ to value 0"]
impl crate::Resettable for MssL2ABusSafetyErrStatReadSpec {
    const RESET_VALUE: u32 = 0;
}
