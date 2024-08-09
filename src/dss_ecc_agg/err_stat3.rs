#[doc = "Register `err_stat3` reader"]
pub type R = crate::R<ErrStat3Spec>;
#[doc = "Register `err_stat3` writer"]
pub type W = crate::W<ErrStat3Spec>;
#[doc = "Field `DELAYED_WRITE_BACK` reader - 0:0\\]
delayed write back pending Status"]
pub type DelayedWriteBackR = crate::BitReader;
#[doc = "Field `DELAYED_WRITE_BACK` writer - 0:0\\]
delayed write back pending Status"]
pub type DelayedWriteBackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEVEL_SVBUS_TIMEOUT` reader - 1:1\\]
Level svbus timeout error Error Status"]
pub type LevelSvbusTimeoutR = crate::BitReader;
#[doc = "Field `LEVEL_SVBUS_TIMEOUT` writer - 1:1\\]
Level svbus timeout error Error Status"]
pub type LevelSvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SVBUS_TIMEOUT` reader - 9:9\\]
Clear svbus timeout Error Status"]
pub type ClearSvbusTimeoutR = crate::BitReader;
#[doc = "Field `CLEAR_SVBUS_TIMEOUT` writer - 9:9\\]
Clear svbus timeout Error Status"]
pub type ClearSvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
delayed write back pending Status"]
    #[inline(always)]
    pub fn delayed_write_back(&self) -> DelayedWriteBackR {
        DelayedWriteBackR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Level svbus timeout error Error Status"]
    #[inline(always)]
    pub fn level_svbus_timeout(&self) -> LevelSvbusTimeoutR {
        LevelSvbusTimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear svbus timeout Error Status"]
    #[inline(always)]
    pub fn clear_svbus_timeout(&self) -> ClearSvbusTimeoutR {
        ClearSvbusTimeoutR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
delayed write back pending Status"]
    #[inline(always)]
    #[must_use]
    pub fn delayed_write_back(&mut self) -> DelayedWriteBackW<ErrStat3Spec> {
        DelayedWriteBackW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Level svbus timeout error Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn level_svbus_timeout(&mut self) -> LevelSvbusTimeoutW<ErrStat3Spec> {
        LevelSvbusTimeoutW::new(self, 1)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear svbus timeout Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn clear_svbus_timeout(&mut self) -> ClearSvbusTimeoutW<ErrStat3Spec> {
        ClearSvbusTimeoutW::new(self, 9)
    }
}
#[doc = "ECC Error Status3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStat3Spec;
impl crate::RegisterSpec for ErrStat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat3::R`](R) reader structure"]
impl crate::Readable for ErrStat3Spec {}
#[doc = "`write(|w| ..)` method takes [`err_stat3::W`](W) writer structure"]
impl crate::Writable for ErrStat3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets err_stat3 to value 0"]
impl crate::Resettable for ErrStat3Spec {
    const RESET_VALUE: u32 = 0;
}
