#[doc = "Register `R5SS2DSS_BUS_SAFETY_FI` reader"]
pub type R = crate::R<R5ss2dssBusSafetyFiSpec>;
#[doc = "Register `R5SS2DSS_BUS_SAFETY_FI` writer"]
pub type W = crate::W<R5ss2dssBusSafetyFiSpec>;
#[doc = "Field `global_main` reader - 0:0\\]
Refer to 25xx Substem Microarch document for more details"]
pub type GlobalMainR = crate::BitReader;
#[doc = "Field `global_main` writer - 0:0\\]
Refer to 25xx Substem Microarch document for more details"]
pub type GlobalMainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `global_safe` reader - 1:1\\]
Refer to 25xx Substem Microarch document for more details"]
pub type GlobalSafeR = crate::BitReader;
#[doc = "Field `global_safe` writer - 1:1\\]
Refer to 25xx Substem Microarch document for more details"]
pub type GlobalSafeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `global_main_req` reader - 2:2\\]
Refer to 25xx Substem Microarch document for more details"]
pub type GlobalMainReqR = crate::BitReader;
#[doc = "Field `global_main_req` writer - 2:2\\]
Refer to 25xx Substem Microarch document for more details"]
pub type GlobalMainReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `global_safe_req` reader - 3:3\\]
Refer to 25xx Substem Microarch document for more details"]
pub type GlobalSafeReqR = crate::BitReader;
#[doc = "Field `global_safe_req` writer - 3:3\\]
Refer to 25xx Substem Microarch document for more details"]
pub type GlobalSafeReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sec` reader - 4:4\\]
Refer to 25xx Substem Microarch document for more details"]
pub type SecR = crate::BitReader;
#[doc = "Field `sec` writer - 4:4\\]
Refer to 25xx Substem Microarch document for more details"]
pub type SecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ded` reader - 5:5\\]
Refer to 25xx Substem Microarch document for more details"]
pub type DedR = crate::BitReader;
#[doc = "Field `ded` writer - 5:5\\]
Refer to 25xx Substem Microarch document for more details"]
pub type DedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `data` reader - 15:8\\]
Refer to 25xx Substem Microarch document for more details"]
pub type DataR = crate::FieldReader;
#[doc = "Field `data` writer - 15:8\\]
Refer to 25xx Substem Microarch document for more details"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `main` reader - 23:16\\]
Refer to 25xx Substem Microarch document for more details"]
pub type MainR = crate::FieldReader;
#[doc = "Field `main` writer - 23:16\\]
Refer to 25xx Substem Microarch document for more details"]
pub type MainW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `safe` reader - 31:24\\]
Refer to 25xx Substem Microarch document for more details"]
pub type SafeR = crate::FieldReader;
#[doc = "Field `safe` writer - 31:24\\]
Refer to 25xx Substem Microarch document for more details"]
pub type SafeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn global_main(&self) -> GlobalMainR {
        GlobalMainR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn global_safe(&self) -> GlobalSafeR {
        GlobalSafeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn global_main_req(&self) -> GlobalMainReqR {
        GlobalMainReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn global_safe_req(&self) -> GlobalSafeReqR {
        GlobalSafeReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn ded(&self) -> DedR {
        DedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn main(&self) -> MainR {
        MainR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    pub fn safe(&self) -> SafeR {
        SafeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn global_main(&mut self) -> GlobalMainW<R5ss2dssBusSafetyFiSpec> {
        GlobalMainW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn global_safe(&mut self) -> GlobalSafeW<R5ss2dssBusSafetyFiSpec> {
        GlobalSafeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn global_main_req(&mut self) -> GlobalMainReqW<R5ss2dssBusSafetyFiSpec> {
        GlobalMainReqW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn global_safe_req(&mut self) -> GlobalSafeReqW<R5ss2dssBusSafetyFiSpec> {
        GlobalSafeReqW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SecW<R5ss2dssBusSafetyFiSpec> {
        SecW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn ded(&mut self) -> DedW<R5ss2dssBusSafetyFiSpec> {
        DedW::new(self, 5)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<R5ss2dssBusSafetyFiSpec> {
        DataW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn main(&mut self) -> MainW<R5ss2dssBusSafetyFiSpec> {
        MainW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Refer to 25xx Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn safe(&mut self) -> SafeW<R5ss2dssBusSafetyFiSpec> {
        SafeW::new(self, 24)
    }
}
#[doc = "R5SS2DSS_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_fi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_fi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5ss2dssBusSafetyFiSpec;
impl crate::RegisterSpec for R5ss2dssBusSafetyFiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5ss2dss_bus_safety_fi::R`](R) reader structure"]
impl crate::Readable for R5ss2dssBusSafetyFiSpec {}
#[doc = "`write(|w| ..)` method takes [`r5ss2dss_bus_safety_fi::W`](W) writer structure"]
impl crate::Writable for R5ss2dssBusSafetyFiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5SS2DSS_BUS_SAFETY_FI to value 0"]
impl crate::Resettable for R5ss2dssBusSafetyFiSpec {
    const RESET_VALUE: u32 = 0;
}
