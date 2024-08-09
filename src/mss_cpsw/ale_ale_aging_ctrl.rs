#[doc = "Register `ALE_ALE_AGING_CTRL` reader"]
pub type R = crate::R<AleAleAgingCtrlSpec>;
#[doc = "Register `ALE_ALE_AGING_CTRL` writer"]
pub type W = crate::W<AleAleAgingCtrlSpec>;
#[doc = "Field `ALE_AGING_TIMER` reader - 23:0\\]
ALE Aging Timer - This field specifies the number of clock cycles times 1,000,000 between aging operations."]
pub type AleAgingTimerR = crate::FieldReader<u32>;
#[doc = "Field `ALE_AGING_TIMER` writer - 23:0\\]
ALE Aging Timer - This field specifies the number of clock cycles times 1,000,000 between aging operations."]
pub type AleAgingTimerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `ALE_PRESCALER_1` reader - 30:30\\]
ALE Prescaler 1 Disable - When set will divide the aging interval by 1000. This bit is designed for device verification and should not be used in production software. Combination of PreScale1Disable and PreScale2Disable will divide the aging interval by 1,000,000 for test purposes."]
pub type AlePrescaler1R = crate::BitReader;
#[doc = "Field `ALE_PRESCALER_1` writer - 30:30\\]
ALE Prescaler 1 Disable - When set will divide the aging interval by 1000. This bit is designed for device verification and should not be used in production software. Combination of PreScale1Disable and PreScale2Disable will divide the aging interval by 1,000,000 for test purposes."]
pub type AlePrescaler1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALE_PRESCALER_2` reader - 31:31\\]
ALE Prescaler 2 Disable - When set will divide the aging interval by 1000. This bit is designed for device verification and should not be used in production software. Combination of PreScale1Disable and PreScale2Disable will divide the aging interval by 1,000,000 for test purposes."]
pub type AlePrescaler2R = crate::BitReader;
#[doc = "Field `ALE_PRESCALER_2` writer - 31:31\\]
ALE Prescaler 2 Disable - When set will divide the aging interval by 1000. This bit is designed for device verification and should not be used in production software. Combination of PreScale1Disable and PreScale2Disable will divide the aging interval by 1,000,000 for test purposes."]
pub type AlePrescaler2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
ALE Aging Timer - This field specifies the number of clock cycles times 1,000,000 between aging operations."]
    #[inline(always)]
    pub fn ale_aging_timer(&self) -> AleAgingTimerR {
        AleAgingTimerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - 30:30\\]
ALE Prescaler 1 Disable - When set will divide the aging interval by 1000. This bit is designed for device verification and should not be used in production software. Combination of PreScale1Disable and PreScale2Disable will divide the aging interval by 1,000,000 for test purposes."]
    #[inline(always)]
    pub fn ale_prescaler_1(&self) -> AlePrescaler1R {
        AlePrescaler1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
ALE Prescaler 2 Disable - When set will divide the aging interval by 1000. This bit is designed for device verification and should not be used in production software. Combination of PreScale1Disable and PreScale2Disable will divide the aging interval by 1,000,000 for test purposes."]
    #[inline(always)]
    pub fn ale_prescaler_2(&self) -> AlePrescaler2R {
        AlePrescaler2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
ALE Aging Timer - This field specifies the number of clock cycles times 1,000,000 between aging operations."]
    #[inline(always)]
    #[must_use]
    pub fn ale_aging_timer(&mut self) -> AleAgingTimerW<AleAleAgingCtrlSpec> {
        AleAgingTimerW::new(self, 0)
    }
    #[doc = "Bit 30 - 30:30\\]
ALE Prescaler 1 Disable - When set will divide the aging interval by 1000. This bit is designed for device verification and should not be used in production software. Combination of PreScale1Disable and PreScale2Disable will divide the aging interval by 1,000,000 for test purposes."]
    #[inline(always)]
    #[must_use]
    pub fn ale_prescaler_1(&mut self) -> AlePrescaler1W<AleAleAgingCtrlSpec> {
        AlePrescaler1W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
ALE Prescaler 2 Disable - When set will divide the aging interval by 1000. This bit is designed for device verification and should not be used in production software. Combination of PreScale1Disable and PreScale2Disable will divide the aging interval by 1,000,000 for test purposes."]
    #[inline(always)]
    #[must_use]
    pub fn ale_prescaler_2(&mut self) -> AlePrescaler2W<AleAleAgingCtrlSpec> {
        AlePrescaler2W::new(self, 31)
    }
}
#[doc = "The ALE Aging Control sets the aging interval which will cause periodic aging to occur. This value specifies the minimum time between aging starts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_aging_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_aging_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleAgingCtrlSpec;
impl crate::RegisterSpec for AleAleAgingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_aging_ctrl::R`](R) reader structure"]
impl crate::Readable for AleAleAgingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_aging_ctrl::W`](W) writer structure"]
impl crate::Writable for AleAleAgingCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_AGING_CTRL to value 0"]
impl crate::Resettable for AleAleAgingCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
