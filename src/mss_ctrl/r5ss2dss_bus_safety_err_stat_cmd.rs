#[doc = "Register `R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD` reader"]
pub type R = crate::R<R5ss2dssBusSafetyErrStatCmdSpec>;
#[doc = "Register `R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD` writer"]
pub type W = crate::W<R5ss2dssBusSafetyErrStatCmdSpec>;
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
    pub fn stat(&mut self) -> StatW<R5ss2dssBusSafetyErrStatCmdSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_err_stat_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_err_stat_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5ss2dssBusSafetyErrStatCmdSpec;
impl crate::RegisterSpec for R5ss2dssBusSafetyErrStatCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5ss2dss_bus_safety_err_stat_cmd::R`](R) reader structure"]
impl crate::Readable for R5ss2dssBusSafetyErrStatCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`r5ss2dss_bus_safety_err_stat_cmd::W`](W) writer structure"]
impl crate::Writable for R5ss2dssBusSafetyErrStatCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD to value 0"]
impl crate::Resettable for R5ss2dssBusSafetyErrStatCmdSpec {
    const RESET_VALUE: u32 = 0;
}
