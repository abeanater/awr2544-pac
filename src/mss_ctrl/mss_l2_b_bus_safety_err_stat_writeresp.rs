#[doc = "Register `MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP` reader"]
pub type R = crate::R<MssL2BBusSafetyErrStatWriterespSpec>;
#[doc = "Register `MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP` writer"]
pub type W = crate::W<MssL2BBusSafetyErrStatWriterespSpec>;
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
    pub fn stat(&mut self) -> StatW<MssL2BBusSafetyErrStatWriterespSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_err_stat_writeresp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_err_stat_writeresp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssL2BBusSafetyErrStatWriterespSpec;
impl crate::RegisterSpec for MssL2BBusSafetyErrStatWriterespSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_l2_b_bus_safety_err_stat_writeresp::R`](R) reader structure"]
impl crate::Readable for MssL2BBusSafetyErrStatWriterespSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_l2_b_bus_safety_err_stat_writeresp::W`](W) writer structure"]
impl crate::Writable for MssL2BBusSafetyErrStatWriterespSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP to value 0"]
impl crate::Resettable for MssL2BBusSafetyErrStatWriterespSpec {
    const RESET_VALUE: u32 = 0;
}
