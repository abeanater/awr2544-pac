#[doc = "Register `DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0` reader"]
pub type R = crate::R<Dss2r5ssBusSafetyErrStatData0Spec>;
#[doc = "Register `DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0` writer"]
pub type W = crate::W<Dss2r5ssBusSafetyErrStatData0Spec>;
#[doc = "Field `d0` reader - 7:0\\]
Refer to 25xx Substem Microarch document for more details"]
pub type D0R = crate::FieldReader;
#[doc = "Field `d0` writer - 7:0\\]
Refer to 25xx Substem Microarch document for more details"]
pub type D0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `d1` reader - 15:8\\]
Refer to 25xx Substem Microarch document for more details"]
pub type D1R = crate::FieldReader;
#[doc = "Field `d1` writer - 15:8\\]
Refer to 25xx Substem Microarch document for more details"]
pub type D1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn d0(&self) -> D0R {
        D0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn d1(&self) -> D1R {
        D1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn d0(&mut self) -> D0W<Dss2r5ssBusSafetyErrStatData0Spec> {
        D0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn d1(&mut self) -> D1W<Dss2r5ssBusSafetyErrStatData0Spec> {
        D1W::new(self, 8)
    }
}
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_err_stat_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_err_stat_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dss2r5ssBusSafetyErrStatData0Spec;
impl crate::RegisterSpec for Dss2r5ssBusSafetyErrStatData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss2r5ss_bus_safety_err_stat_data0::R`](R) reader structure"]
impl crate::Readable for Dss2r5ssBusSafetyErrStatData0Spec {}
#[doc = "`write(|w| ..)` method takes [`dss2r5ss_bus_safety_err_stat_data0::W`](W) writer structure"]
impl crate::Writable for Dss2r5ssBusSafetyErrStatData0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0 to value 0"]
impl crate::Resettable for Dss2r5ssBusSafetyErrStatData0Spec {
    const RESET_VALUE: u32 = 0;
}
