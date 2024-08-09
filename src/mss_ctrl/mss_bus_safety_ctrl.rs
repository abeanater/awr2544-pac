#[doc = "Register `MSS_BUS_SAFETY_CTRL` reader"]
pub type R = crate::R<MssBusSafetyCtrlSpec>;
#[doc = "Register `MSS_BUS_SAFETY_CTRL` writer"]
pub type W = crate::W<MssBusSafetyCtrlSpec>;
#[doc = "Field `enable` reader - 2:0\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<MssBusSafetyCtrlSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "MSS_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_bus_safety_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_bus_safety_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssBusSafetyCtrlSpec;
impl crate::RegisterSpec for MssBusSafetyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_bus_safety_ctrl::R`](R) reader structure"]
impl crate::Readable for MssBusSafetyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_bus_safety_ctrl::W`](W) writer structure"]
impl crate::Writable for MssBusSafetyCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_BUS_SAFETY_CTRL to value 0"]
impl crate::Resettable for MssBusSafetyCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
