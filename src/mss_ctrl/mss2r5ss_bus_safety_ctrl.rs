#[doc = "Register `MSS2R5SS_BUS_SAFETY_CTRL` reader"]
pub type R = crate::R<Mss2r5ssBusSafetyCtrlSpec>;
#[doc = "Register `MSS2R5SS_BUS_SAFETY_CTRL` writer"]
pub type W = crate::W<Mss2r5ssBusSafetyCtrlSpec>;
#[doc = "Field `enable` reader - 2:0\\]
Refer to 25xx Substem Microarch document for more details"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
Refer to 25xx Substem Microarch document for more details"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `err_clear` reader - 8:8\\]
Refer to 25xx Substem Microarch document for more details"]
pub type ErrClearR = crate::BitReader;
#[doc = "Field `err_clear` writer - 8:8\\]
Refer to 25xx Substem Microarch document for more details"]
pub type ErrClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `type` reader - 23:16\\]
Refer to 25xx Substem Microarch document for more details"]
pub type TypeR = crate::FieldReader;
#[doc = "Field `type` writer - 23:16\\]
Refer to 25xx Substem Microarch document for more details"]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn err_clear(&self) -> ErrClearR {
        ErrClearR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Mss2r5ssBusSafetyCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn err_clear(&mut self) -> ErrClearW<Mss2r5ssBusSafetyCtrlSpec> {
        ErrClearW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TypeW<Mss2r5ssBusSafetyCtrlSpec> {
        TypeW::new(self, 16)
    }
}
#[doc = "MSS2R5SS_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mss2r5ssBusSafetyCtrlSpec;
impl crate::RegisterSpec for Mss2r5ssBusSafetyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss2r5ss_bus_safety_ctrl::R`](R) reader structure"]
impl crate::Readable for Mss2r5ssBusSafetyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mss2r5ss_bus_safety_ctrl::W`](W) writer structure"]
impl crate::Writable for Mss2r5ssBusSafetyCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS2R5SS_BUS_SAFETY_CTRL to value 0"]
impl crate::Resettable for Mss2r5ssBusSafetyCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
