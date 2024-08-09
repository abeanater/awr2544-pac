#[doc = "Register `MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE` reader"]
pub type R = crate::R<MssL2CBusSafetyErrStatWriteSpec>;
#[doc = "Register `MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE` writer"]
pub type W = crate::W<MssL2CBusSafetyErrStatWriteSpec>;
#[doc = "Field `stat` reader - 31:0\\]
Refer to 25xx Substem Microarch document for more details"]
pub type StatR = crate::FieldReader<u32>;
#[doc = "Field `stat` writer - 31:0\\]
Refer to 25xx Substem Microarch document for more details"]
pub type StatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<MssL2CBusSafetyErrStatWriteSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_err_stat_write::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_err_stat_write::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssL2CBusSafetyErrStatWriteSpec;
impl crate::RegisterSpec for MssL2CBusSafetyErrStatWriteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_l2_c_bus_safety_err_stat_write::R`](R) reader structure"]
impl crate::Readable for MssL2CBusSafetyErrStatWriteSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_l2_c_bus_safety_err_stat_write::W`](W) writer structure"]
impl crate::Writable for MssL2CBusSafetyErrStatWriteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE to value 0"]
impl crate::Resettable for MssL2CBusSafetyErrStatWriteSpec {
    const RESET_VALUE: u32 = 0;
}
