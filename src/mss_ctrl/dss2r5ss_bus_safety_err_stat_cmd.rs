#[doc = "Register `DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD` reader"]
pub type R = crate::R<Dss2r5ssBusSafetyErrStatCmdSpec>;
#[doc = "Register `DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD` writer"]
pub type W = crate::W<Dss2r5ssBusSafetyErrStatCmdSpec>;
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
    pub fn stat(&mut self) -> StatW<Dss2r5ssBusSafetyErrStatCmdSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_err_stat_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_err_stat_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dss2r5ssBusSafetyErrStatCmdSpec;
impl crate::RegisterSpec for Dss2r5ssBusSafetyErrStatCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss2r5ss_bus_safety_err_stat_cmd::R`](R) reader structure"]
impl crate::Readable for Dss2r5ssBusSafetyErrStatCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`dss2r5ss_bus_safety_err_stat_cmd::W`](W) writer structure"]
impl crate::Writable for Dss2r5ssBusSafetyErrStatCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD to value 0"]
impl crate::Resettable for Dss2r5ssBusSafetyErrStatCmdSpec {
    const RESET_VALUE: u32 = 0;
}
